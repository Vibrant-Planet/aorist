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
    name: PointCloudIndexSchema,
    source: PointCloudAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      minx: FloatLongitude("Bbox min x value", false),
      miny: FloatLatitude("Bbox min y value", false),
      maxx: FloatLongitude("Bbox max x value", false),
      maxy: FloatLatitude("Bbox max y value", false),
      json: FreeText("JSON for pdal info", false)
}
