use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct InlineBlobStorage {
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
    pub data: AVec<AString>,
}
