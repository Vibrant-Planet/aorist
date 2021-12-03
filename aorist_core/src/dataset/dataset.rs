#![allow(dead_code)]
use crate::access_policy::*;
use crate::asset::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
#[cfg(feature = "python")]
use crate::encoding::*;
#[cfg(feature = "python")]
use crate::storage::*;
use crate::storage_setup::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::TAoristObject;
use derivative::Derivative;
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
#[cfg(feature = "python")]
use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use uuid::Uuid;

#[aorist]
pub struct DataSet {
    pub name: String,
    pub description: String,
    pub source_path: String,
    #[constrainable]
    pub access_policies: Vec<AoristRef<AccessPolicy>>,
    #[constrainable]
    pub datum_templates: Vec<AoristRef<DatumTemplate>>,
    #[constrainable]
    pub assets: BTreeMap<String, AoristRef<Asset>>,
}

impl DataSet {
    pub fn add_asset(&mut self, asset: AoristRef<Asset>) {
        self.assets
            .insert(asset.0.read().get_name(), asset.clone());
    }
    pub fn add_template(&mut self, template: AoristRef<DatumTemplate>) {
        self.datum_templates.push(template);
    }
    pub fn get_assets(&self) -> Vec<AoristRef<Asset>> {
        self.assets
            .values()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| (*x).clone())
            .collect()
    }

    pub fn get_templates(&self) -> Vec<AoristRef<DatumTemplate>> {
        self.datum_templates.clone()
    }

    pub fn get_template_for_asset<T: TAsset>(
        &self,
        asset: &T,
    ) -> Result<AoristRef<DatumTemplate>, String> {
        let schema = asset.get_schema();
        let template_name = schema.0.read().get_datum_template_name().unwrap();
        let mapped_templates = self.get_mapped_datum_templates();
        let template = mapped_templates.get(&template_name);
        match template {
            Some(template) => Ok(template.clone()),
            None => Err(format!(
                "Could not find template for asset {} in dataset {}",
                asset.get_name(),
                self.name,
            )),
        }
    }

    pub fn get_asset(&self, name: String) -> Result<AoristRef<Asset>, String> {
        if let Some(asset) = self.assets.get(&name) {
            return Ok(asset.clone());
        }
        Err(format!("Could not find asset {} in dataset {}.", name, self.name).to_string())
    }

    // TODO: should reference identifier tuple
    pub fn get_source_assets(
        &self,
        setup: AoristRef<ComputedFromLocalData>,
    ) -> Result<BTreeMap<String, AoristRef<Asset>>, String> {
        let mut assets = BTreeMap::new();
        for name in setup.0.read().source_asset_names.values() {
            assets.insert(name.clone(), self.get_asset(name.clone())?);
        }
        Ok(assets)
    }
}

impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub trait TDataSet {
    fn get_mapped_datum_templates(&self) -> LinkedHashMap<String, AoristRef<DatumTemplate>>;
}

impl TDataSet for DataSet {
    fn get_mapped_datum_templates(&self) -> LinkedHashMap<String, AoristRef<DatumTemplate>> {
        self.datum_templates
            .iter()
            .map(|x| (x.0.read().get_name().clone(), x.clone()))
            .collect()
    }
}
#[cfg(feature = "python")]
impl PyDataSet {
    fn get_mapped_datum_templates(&self) -> LinkedHashMap<String, PyDatumTemplate> {
        self.inner
            .0
            .read()
            .get_mapped_datum_templates()
            .into_iter()
            .map(|(k, v)| (k.clone(), PyDatumTemplate { inner: v.clone() }))
            .collect()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyDataSet {
    pub fn add_asset(&self, asset: PyAsset) {
        self.inner.0.write().add_asset(asset.inner.clone());
    }
    pub fn get_asset(&self, name: String) -> PyAsset {
        PyAsset {
            inner: self.inner.0.read().get_asset(name).unwrap(),
        }
    }
    pub fn add_template(&self, template: PyDatumTemplate) {
        self.inner
            .0
            .write()
            .add_template(template.inner.clone());
    }
    pub fn persist_local(&self, storage: PyStorage) -> PyResult<Self> {
        let dt = &*self.inner.0.read();
        let mut persisted_assets = BTreeMap::new();
        for (key, asset_rw) in dt.assets.iter() {
            let asset = &*asset_rw.0.read();
            persisted_assets.insert(
                key.clone(),
                AoristRef(RArc::new(RRwLock::new(
                    asset.persist_local(storage.inner.deep_clone()),
                ))),
            );
        }
        let inner = AoristRef(RArc::new(RRwLock::new(DataSet {
            name: dt.name.clone(),
            description: dt.description.clone(),
            source_path: dt.source_path.clone(),
            access_policies: dt.access_policies.clone(),
            datum_templates: dt.datum_templates.clone(),
            assets: persisted_assets,
            tag: dt.tag.clone(),
            uuid: dt.uuid.clone(),
        })));
        Ok(PyDataSet { inner })
    }
    pub fn replicate_to_local(
        &self,
        storage: PyStorage,
        tmp_dir: String,
        tmp_encoding: PyEncoding,
    ) -> PyResult<Self> {
        let dt = &*self.inner.0.read();
        let mut replicated_assets = BTreeMap::new();
        for (key, asset_rw) in dt.assets.iter() {
            let asset = &*asset_rw.0.read();
            let replicated_asset = match asset.replicate_to_local(
                storage.inner.deep_clone(),
                tmp_dir.clone(),
                tmp_encoding.inner.deep_clone(),
            ) {
                Some(x) => AoristRef(RArc::new(RRwLock::new(x))),
                None => asset_rw.clone(),
            };
            replicated_assets.insert(key.clone(), replicated_asset);
        }
        let inner = AoristRef(RArc::new(RRwLock::new(DataSet {
            name: dt.name.clone(),
            description: dt.description.clone(),
            source_path: dt.source_path.clone(),
            access_policies: dt.access_policies.clone(),
            datum_templates: dt.datum_templates.clone(),
            assets: replicated_assets,
            tag: dt.tag.clone(),
            uuid: dt.uuid.clone(),
        })));
        Ok(PyDataSet { inner })
    }
    pub fn get_template(&self, asset: PyAsset) -> PyResult<PyDatumTemplate> {
        let schema = asset.get_schema();
        let template_name = schema.get_datum_template_name()?;
        let mapped_templates = self.get_mapped_datum_templates();
        let template = mapped_templates.get(&template_name);
        match template {
            Some(template) => Ok(template.clone()),
            None => Err(PyValueError::new_err(format!(
                "Could not find template {} for asset {} in dataset {}.\nTemplate names: {}",
                template_name,
                asset.name(),
                self.name()?,
                mapped_templates
                    .keys()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))),
        }
    }
}
