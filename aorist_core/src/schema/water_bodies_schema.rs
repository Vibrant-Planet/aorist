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
    name: WaterBodiesSchema,
    attributes:
      id: KeyStringIdentifier("Polygon Identifier", false),
      name: FreeText("Polygon name", true),
      wkt: WKTString("WKT string", false)
}
