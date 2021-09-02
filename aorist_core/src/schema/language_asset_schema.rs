use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::fasttext_embedding_schema::*;
use crate::schema::named_entity_schema::*;
use crate::schema::text_corpus_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum LanguageAssetSchema {
    #[constrainable]
    FasttextEmbeddingSchema(AoristRef<FasttextEmbeddingSchema>),
    #[constrainable]
    NamedEntitySchema(AoristRef<NamedEntitySchema>),
}

impl LanguageAssetSchema {
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match self {
            LanguageAssetSchema::FasttextEmbeddingSchema(x) => {
                x.0.read().unwrap().get_source_schema()
            }
            LanguageAssetSchema::NamedEntitySchema(x) => x.0.read().unwrap().get_source_schema(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            LanguageAssetSchema::FasttextEmbeddingSchema(x) => {
                x.0.read().unwrap().get_datum_template()
            }
            LanguageAssetSchema::NamedEntitySchema(x) => x.0.read().unwrap().get_datum_template(),
        }
    }
    pub fn get_text_attribute_name(&self) -> String {
        self.get_source_schema()
            .0
            .read()
            .unwrap()
            .get_text_attribute_name()
    }
    pub fn get_datum_template_name(&self) -> String {
        self.get_datum_template().0.read().unwrap().get_name()
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            LanguageAssetSchema::FasttextEmbeddingSchema(x) => {
                x.0.read().unwrap().get_attribute_names()
            }
            LanguageAssetSchema::NamedEntitySchema(x) => x.0.read().unwrap().get_attribute_names(),
        }
    }
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.get_source_schema()
            .0
            .read()
            .unwrap()
            .should_dedup_text_attribute()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyLanguageAssetSchema {
    #[getter]
    pub fn get_text_attribute_name(&self) -> String {
        self.inner.0.read().unwrap().get_text_attribute_name()
    }
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.inner.0.read().unwrap().should_dedup_text_attribute()
    }
}