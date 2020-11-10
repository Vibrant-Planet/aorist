#![allow(non_snake_case)]
use crate::python::TObjectWithPythonCodeGen;
use crate::storage::Storage;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::prefect::{TObjectWithPrefectCodeGen, TStorageSetupWithPrefectDAGCodeGen, TStorageWithPrefectDAGCodeGen};
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoteImportStorageSetup {
    remote: Storage,
    local: Vec<Storage>,
}
impl TObjectWithPrefectCodeGen for RemoteImportStorageSetup {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.remote.get_prefect_preamble(preamble);
        for storage in &self.local {
            storage.get_prefect_preamble(preamble);
        }
    }
}
impl TStorageSetupWithPrefectDAGCodeGen for RemoteImportStorageSetup {
    fn get_prefect_dag(
        &self,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        table_name: &String,
    ) -> Result<String, String> {

        let remote_dag = self.remote.get_prefect_dag(schema)?;
        let mut dag = format!("{}", remote_dag);
        let columnSchema = schema.get_presto_schema(templates);

        for (i, local_storage) in self.local.iter().enumerate() {
            // TODO: 1st, 2nd and 6th argument should be provided by remote_dag
            let local = local_storage.get_prefect_ingest_dag(
                "/tmp".to_string(),
                "decode_file.no_header".to_string(),
                schema,
                templates,
                format!("upload_{}", i),
                "decode_file_remove_header".to_string(),
            )?;
            // TODO: last argument should come from upstream pipeline
            let schema_creation = self.get_presto_schema_creation_task(
                table_name,
                &columnSchema,
                local_storage,
                format!("create_table_{}", i),
                format!("upload_{}_encode", i),
            );

            dag = format!("{}\n{}\n{}", &dag, local, schema_creation);
        }
        Ok(dag.to_string())
    }
}

impl RemoteImportStorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        &self.local
    }
    // TODO: move to Storage. Also, need presto-cli to be configurable
    pub fn get_presto_schema_creation_task(
        &self,
        name: &String,
        columnSchema: &String,
        storage: &Storage,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        let schema = self.get_presto_schema(
            name,
            columnSchema,
            storage,
        );
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command=\"\"\"
                        presto -e \"{schema}\"
                        \"\"\"
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name = task_name,
            upstream_task_name = upstream_task_name,
            schema = schema,
        )
    }
    // TODO: move to Storage
    pub fn get_presto_schema(
        &self,
        name: &String,
        columnSchema: &String,
        storage: &Storage
    ) -> String {
        if storage.is_hive_storage() {
            let mut tags: HashMap<String, String> = HashMap::new();
            storage.populate_table_creation_tags(&mut tags).unwrap();
            let tags_str = match tags.len() {
                0 => "".to_string(),
                _ => format!(
                    " WITH (\n    {}\n)",
                    tags.iter()
                        .map(|(k, v)| format!("{}='{}'", k, v))
                        .collect::<Vec<String>>()
                        .join(",\n    ")
                )
                .to_string(),
            };
            return format!(
                indoc! {
                    "CREATE TABLE IF NOT EXISTS {table} (
                            {column_schema}
                        ){tags_str};"
                },
                table = name,
                column_schema = columnSchema.replace("\n", "\n    "),
                tags_str = tags_str,
            ).to_string()
        }
        "".to_string()
    }
    pub fn get_presto_schemas(&self, name: &String, columnSchema: String) -> String {
        let mut schemas: Vec<String> = Vec::new();
        for storage in self.get_local_storage() {
            schemas.push(
                self.get_presto_schema(name, &columnSchema, storage)
            );
        }
        schemas.join("\n")
    }
}
impl TObjectWithPythonCodeGen for RemoteImportStorageSetup {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.remote.get_python_imports(preamble);
        for storage in &self.local {
            storage.get_python_imports(preamble);
        }
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum StorageSetup {
    RemoteImportStorageSetup(RemoteImportStorageSetup),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => x.get_local_storage(),
        }
    }
    pub fn get_presto_schemas(&self, name: &String, columnSchema: String) -> String {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => x.get_presto_schemas(name, columnSchema),
        }
    }
}
