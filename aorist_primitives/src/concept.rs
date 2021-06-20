use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hasher;
use uuid::Uuid;

pub trait ConceptEnum<'a> {}
pub trait AoristConcept<'a> {
    type TChildrenEnum: ConceptEnum<'a>;

    fn get_children(
        &'a self,
    ) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Option<Uuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    fn get_uuid(&self) -> Uuid;
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_tag(&self) -> Option<String>;

    fn get_uuid_from_children_uuid(&self) -> Uuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            eprintln!("There are child uuids.");
            let uuids = child_uuids.into_iter().collect::<BTreeSet<Uuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            Uuid::from_bytes(bytes)
        } else {
            eprintln!("There are no child uuids.");
            // TODO: this should just be created from the hash
            Uuid::new_v4()
        }
    }
    fn compute_uuids(&mut self);
}
pub trait TConceptEnum<'a>: Sized {
    fn get_parent_id(&self) -> Option<(Uuid, String)>;
    fn get_type(&self) -> String;
    fn get_uuid(&self) -> Uuid;
    fn get_tag(&self) -> Option<String>;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&'a self) -> Vec<Self>;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Self>);
}

pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a> + Clone + TConceptEnum<'a>;
}
pub trait TAoristObject {
    fn get_name(&self) -> &String;
}