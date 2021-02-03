#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::encoding::Encoding;
use crate::layout::HiveStorageLayout;
use crate::location::HiveLocation;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct HiveTableStorage {
    #[constrainable]
    location: HiveLocation,
    #[constrainable]
    layout: HiveStorageLayout,
    #[constrainable]
    pub encoding: Encoding,
}
#[pymethods]
impl HiveTableStorage {
    #[new]
    fn new(location: HiveLocation, layout: HiveStorageLayout, encoding: Encoding) -> Self {
        Self {
            location,
            layout,
            encoding,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
