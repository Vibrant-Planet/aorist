use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Preamble {
    pub imports: Vec<(String, Option<String>)>,
    pub from_imports: Vec<(String, String, Option<String>)>,
    pub body: String,
}
impl<'a> Preamble {
    pub fn new(body: String, py: Python<'a>) -> Preamble {
        let helpers = PyModule::from_code(
            py,
            r#"
import ast
import astor

def build_preamble(body):
    module = ast.parse(body)

    imports = []
    from_imports = []
    other = []

    for elem in module.body:
        if isinstance(elem, ast.Import):
            for name in elem.names:
                imports += [(name.name, name.asname)]
        elif isinstance(elem, ast.ImportFrom):
            for name in elem.names:
                from_imports += [(elem.module, name.name, name.asname)]
        else:
            other += [astor.to_source(elem)]

    return imports, from_imports, other
        "#,
            "helpers.py",
            "helpers",
        )
        .unwrap();

        let tpl: &PyTuple = helpers
            .call1("build_preamble", (body,))
            .unwrap()
            .downcast()
            .unwrap();

        let imports_list: &PyList = tpl.get_item(0).extract().unwrap();
        let imports: Vec<(String, Option<String>)> = imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let name: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(1);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                if asname.is_some() {
                    panic!("Aliased imports not supported yet.");
                }
                (name, asname)
            })
            .collect();

        let from_imports_list: &PyList = tpl.get_item(1).extract().unwrap();
        let from_imports: Vec<(String, String, Option<String>)> = from_imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let module: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let name: String = tpl
                    .get_item(1)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(2);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                if asname.is_some() {
                    panic!("Aliased imports not supported yet.");
                }
                (module, name, asname)
            })
            .collect();

        let body_no_imports: &PyList = tpl.get_item(2).extract().unwrap();
        Self {
            imports,
            from_imports,
            body: body_no_imports
                .iter()
                .map(|x| {
                    x.extract::<&PyString>()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
    pub fn get_body_ast<'b>(&self, py: Python<'b>) -> Vec<&'b PyAny> {
        let helpers = PyModule::from_code(
            py,
            r#"
import ast

def to_nodes(body):
    module = ast.parse(body)
    return module.body
        "#,
            "helpers.py",
            "helpers",
        )
        .unwrap();

        let out: &PyList = helpers
            .call1("to_nodes", (self.body.clone(),))
            .unwrap()
            .downcast()
            .unwrap();

        out.into_iter().collect()
    }
}