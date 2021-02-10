#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    #[py_default = "19998"]
    pub rpcPort: usize,
    #[py_default = "39999"]
    pub apiPort: usize,
}
