#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::long_tabular_schema::*;
use crate::schema::language_asset_schema::*;
use crate::schema::tabular_schema::*;
use crate::schema::time_ordered_tabular_schema::*;
use crate::schema::undefined_tabular_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::template::*;

#[aorist]
pub enum DataSchema {
    #[constrainable]
    LanguageAssetSchema(AoristRef<LanguageAssetSchema>),
    #[constrainable]
    LongTabularSchema(AoristRef<LongTabularSchema>),
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    TimeOrderedTabularSchema(AoristRef<TimeOrderedTabularSchema>),
    #[constrainable]
    UndefinedTabularSchema(AoristRef<UndefinedTabularSchema>),
}

impl DataSchema {
    pub fn get_datum_template(&self) -> Result<AoristRef<DatumTemplate>, String> {
        match self {
            DataSchema::TabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().clone()
            ),
            DataSchema::LongTabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().clone()
            ),
            DataSchema::LanguageAssetSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().clone()
            ),
            DataSchema::TimeOrderedTabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().clone()
            ),
            DataSchema::UndefinedTabularSchema(_) => {
                Err("UndefinedTabularSchema has no datum template.".to_string())
            }
        }
    }
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            DataSchema::TabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().0.read().unwrap().get_name()
            ),
            DataSchema::LongTabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().0.read().unwrap().get_name()
            ),
            DataSchema::LanguageAssetSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().0.read().unwrap().get_name()
            ),
            DataSchema::TimeOrderedTabularSchema(x) => Ok(
                x.0.read().unwrap().get_datum_template().0.read().unwrap().get_name()
            ),
            DataSchema::UndefinedTabularSchema(_) => {
                Err("UndefinedTabularSchema has no datum template.".to_string())
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::TabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::LongTabularSchema(x) => x.0.read().unwrap().get_attribute_names(),
            DataSchema::LanguageAssetSchema(x) => x.0.read().unwrap().get_attribute_names(),
            DataSchema::TimeOrderedTabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::UndefinedTabularSchema(_) => vec![],
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyDataSchema {
    pub fn get_datum_template_name(&self) -> PyResult<String> {
        match self.inner.0.read().unwrap().get_datum_template_name() {
            Ok(s) => Ok(s),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
    #[getter]
    pub fn get_datum_template(&self) -> PyResult<PyDatumTemplate> {
        match self.inner.0.read().unwrap().get_datum_template() {
            Ok(s) => Ok(PyDatumTemplate{ inner: s.clone() }),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}
