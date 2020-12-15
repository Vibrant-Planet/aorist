use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::role::role::TRole;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash, Constrainable)]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
