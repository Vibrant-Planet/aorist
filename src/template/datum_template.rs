#![allow(non_snake_case)]

use crate::attributes::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::template::identifier_tuple::*;
use crate::template::keyed_struct::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
    IdentifierTuple(IdentifierTuple),
}

pub trait TDatumTemplate {
    fn get_attributes(&self) -> Vec<Attribute>;
    fn get_name(&self) -> String;
}
pub trait TInnerDatumTemplate {
    fn get_attributes(&self) -> Vec<InnerAttribute>;
    fn get_name(&self) -> String;
}
impl TDatumTemplate for DatumTemplate {
    fn get_name(&self) -> String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_name(),
            DatumTemplate::IdentifierTuple(x) => x.get_name(),
        }
    }
    fn get_attributes(&self) -> Vec<Attribute> {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_attributes(),
            DatumTemplate::IdentifierTuple(x) => x.get_attributes(),
        }
    }
}
impl TInnerDatumTemplate for InnerDatumTemplate {
    fn get_name(&self) -> String {
        match self {
            InnerDatumTemplate::KeyedStruct(x) => x.name.clone(),
            InnerDatumTemplate::IdentifierTuple(x) => x.name.clone(),
        }
    }
    fn get_attributes(&self) -> Vec<InnerAttribute> {
        match self {
            InnerDatumTemplate::KeyedStruct(x) => x.attributes.clone(),
            InnerDatumTemplate::IdentifierTuple(x) => x.attributes.clone(),
        }
    }
}
