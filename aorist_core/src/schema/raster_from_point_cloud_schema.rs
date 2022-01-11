use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: RasterFromPointCloudSchema,
    source: PointCloudAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false)
    fields:
      pdal_pipeline: Option<String>,
      lidr_call: Option<String>,
      window_size: FloatValue,
      resolution: FloatValue
}
