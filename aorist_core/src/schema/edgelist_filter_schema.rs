use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: EdgelistFilterSchema,
    source: SimpleUndirectedGraphAsset,
    attributes:
      id1: KeyStringIdentifier("Node 1 Identifier", false),
      id2: KeyStringIdentifier("Node 2 Identifier", false)
    fields:
      sql_predicate: AString
}
