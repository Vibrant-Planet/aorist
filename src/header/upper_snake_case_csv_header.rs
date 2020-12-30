#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct UpperSnakeCaseCSVHeader {
    uuid: Option<Uuid>,
    tag: Option<String>,
    // number of lines for the header (i.e., extra lines describing the
    // file, e.g. for snap.stanford.edu data), if this is None, assume
    pub num_lines: Option<usize>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
