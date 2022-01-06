use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, primary_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

primary_schema! {
    name: TAOExternalCrownHullSchema,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      tao_id: KeyInt64Identifier("ID of TAO (unique in file)", false),
      wkt: WKTString("WKT string of TAO boundary", false),
      stats: JSON("JSON map of metrics", false)
}
