#![allow(non_snake_case)]

use crate::compression::*;
use crate::concept::{AoristConcept, ConceptEnum, WrappedConcept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct GDBEncoding {
    #[py_default = "None"]
    #[constrainable]
    pub compression: Option<DataCompression>,
}
