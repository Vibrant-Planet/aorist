use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
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
    name: TAOToEcoObjectSchema,
    sources:
      - taos: CrownHullAsset,
      - adjacency: SimpleUndirectedGraphAsset,
    attributes:
      path: KeyStringIdentifier("TAO File Path", false),
      tao_id: KeyInt64Identifier("ID of TAO (unique in file)", false),
      eco_object_id: KeyInt64Identifier("ID of TAO (unique in file)", false),
      wkt: WKTString("WKT string of TAO boundary", false),
      metrics: JSON("JSON map of metrics", false)
    fields:
      tao_height_attribute: String,
      min_num_trees_for_adjacency: usize,
      min_height: FloatValue,
      proportion_of_seed_height_for_valid_non_seed: FloatValue,
      max_neck_size: FloatValue,
      min_crown_width_for_singleton: FloatValue,
      ecobject_hull_concavity_param: FloatValue
}
