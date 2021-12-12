use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::concept::{AoristRef, WrappedConcept};
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TwoTierStorageSetup {
    #[constrainable]
    pub scratch: AoristRef<Storage>,
    #[constrainable]
    pub persistent: AoristRef<Storage>,
    pub tmp_dir: AString,
}
