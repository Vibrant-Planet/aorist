#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use markdown_gen::markdown::*;

#[aorist_concept]
pub struct RemoteStorage {
    #[constrainable]
    pub location: RemoteLocation,
    #[constrainable]
    pub layout: APIOrFileLayout,
    #[constrainable]
    pub encoding: Encoding,
}

impl RemoteStorage {
    pub fn markdown(&self, md: &mut Markdown<Vec<u8>>) {
        md.write("Location".heading(2)).unwrap();
        self.location.markdown(md);
    }
}
