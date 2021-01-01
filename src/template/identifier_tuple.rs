#![allow(non_snake_case)]

use crate::attributes::Attribute;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Derivative, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct IdentifierTuple {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<Attribute>,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl IdentifierTuple {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}