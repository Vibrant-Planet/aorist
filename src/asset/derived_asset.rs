#![allow(non_snake_case)]
use crate::algorithms::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct DerivedAsset {
    pub name: String,
    #[constrainable]
    pub setup: StorageSetup,
    #[constrainable]
    pub schema: DataSchema,
}
