use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::role::role::TRole;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct GlobalPermissionsAdmin {}
impl TRole for AoristRef<GlobalPermissionsAdmin> {
    fn get_permissions(&self) -> AVec<AString> {
        vec!["gitea/admin".into(), "ranger/admin".into()]
            .into_iter()
            .collect()
    }
}
