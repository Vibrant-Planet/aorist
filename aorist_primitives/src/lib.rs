#![allow(non_snake_case)]
use indoc::formatdoc;
use sqlparser::ast::{ColumnDef, DataType, Ident};
use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::fs;

#[macro_export]
macro_rules! define_attribute {
    ($element:ident, $presto_type:ident, $orc_type:ident, $sql_type:ident) => {
        #[derive(
            Debug,
            PartialEq,
            Serialize,
            Deserialize,
            Clone,
            Constrainable,
            $presto_type,
            $orc_type,
            $sql_type,
        )]
        pub struct $element {
            name: String,
            comment: Option<String>,
        }
        impl TAttribute for $element {
            fn get_name(&self) -> &String {
                &self.name
            }
            fn get_comment(&self) -> &Option<String> {
                &self.comment
            }
        }
    };
}

#[macro_export]
macro_rules! define_constraint {
    ($element:ident, $root:ident) => {
        pub struct $element {
        }
        impl TConstraint for $element {
            type Root = $root;
            fn get_root_type_name() -> String {
                stringify!($root).into()
            }
            fn get_required_constraint_names() -> Vec<String> {
                Vec::new()
            }
        }
    };
    ($element:ident, $root:ident, $($required:ident),+) => {
        paste::item! {
            pub struct $element {
                $([<$required:snake:lower>] : Vec<$required>),+
            }
            impl TConstraint for $element {
                type Root = $root;
                fn get_root_type_name() -> String {
                    stringify!($root).into()
                }
                fn get_required_constraint_names() -> Vec<String> {
                    vec![$(
                        stringify!($required).into()
                    ),+]
                }
            }
        }
    };
}
#[macro_export]
macro_rules! register_constraint {
    ( $name:ident, $($element: ident),+ ) => {
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl $name {
            fn get_root_type_names() -> HashMap<String, String> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_root_type_name(),
                    )+
                }
            }
            fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
        }
    }
}
#[macro_export]
macro_rules! register_attribute {
    ( $name:ident, $($element: ident),+ ) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
        #[serde(tag = "type")]
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl TAttribute for $name {
            fn get_name(&self) -> &String {
                match self {
                    $(
                        $name::$element(x) => x.get_name(),
                    )+
                }
            }
            fn get_comment(&self) -> &Option<String> {
                match self {
                    $(
                        $name::$element(x) => x.get_comment(),
                    )+
                }
            }
        }
        impl TPrestoAttribute for $name {
            fn get_presto_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_presto_type(),
                    )+
                }
            }
        }
        impl TOrcAttribute for $name {
            fn get_orc_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_orc_type(),
                    )+
                }
            }
        }
        impl TSQLAttribute for $name {
            fn get_sql_type(&self) -> DataType {
                match self {
                    $(
                        $name::$element(x) => x.get_sql_type(),
                    )+
                }
            }
        }
    }
}

pub trait TAttribute {
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!("{}{}{}", self.get_name(), spaces, self.get_presto_type(),);
        if let Some(comment) = self.get_comment() {
            let formatted_with_comment = formatdoc!(
                "
                {first_line}
                     COMMENT '{comment}'",
                first_line = first_line,
                comment = comment.trim().replace("'", "\\'").to_string()
            );
            return formatted_with_comment;
        }
        first_line
    }
}
pub trait TOrcAttribute: TAttribute {
    fn get_orc_type(&self) -> String;
    fn get_orc_schema(&self) -> String {
        format!("{}:{}", self.get_name(), self.get_orc_type()).to_string()
    }
}
pub trait TSQLAttribute: TAttribute {
    fn get_sql_type(&self) -> DataType;
    fn get_coldef(&self) -> ColumnDef {
        ColumnDef {
            name: Ident::new(self.get_name()),
            data_type: self.get_sql_type(),
            collation: None,
            // TODO: add comments here
            options: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Python {}
#[derive(Debug)]
pub struct R {}
#[derive(Debug)]
pub struct Bash {}

#[derive(Debug)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
}

pub trait DownloadDataFromRemote {
    // TODO: change this to proper error
    fn get_call(&self, dialect: Dialect) -> Result<String, String>;
}

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}
pub fn get_raw_objects_of_type(
    raw_objects: &Vec<HashMap<String, Value>>,
    object_type: String,
) -> Vec<HashMap<String, Value>> {
    raw_objects
        .into_iter()
        .filter(|x| x.get("type").unwrap().as_str().unwrap() == object_type)
        .map(|x| {
            x.get("spec")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.clone()))
                .collect()
        })
        .collect::<Vec<HashMap<String, Value>>>()
}

