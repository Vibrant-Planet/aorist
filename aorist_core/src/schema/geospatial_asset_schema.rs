use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::linz_primary_parcels_schema::*;
use crate::schema::linz_property_titles_schema::*;
use crate::schema::point_cloud_boundary_schema::*;
use crate::schema::point_cloud_info_schema::*;
use crate::schema::point_cloud_metadata_schema::*;
use crate::schema::point_cloud_schema::*;
use crate::schema::raster_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

#[aorist]
pub enum GeospatialAssetSchema {
    LINZPrimaryParcelsSchema(AoristRef<LINZPrimaryParcelsSchema>),
    LINZPropertyTitlesSchema(AoristRef<LINZPropertyTitlesSchema>),
    PointCloudSchema(AoristRef<PointCloudSchema>),
    PointCloudBoundarySchema(AoristRef<PointCloudBoundarySchema>),
    PointCloudInfoSchema(AoristRef<PointCloudInfoSchema>),
    PointCloudMetadataSchema(AoristRef<PointCloudMetadataSchema>),
    RasterSchema(AoristRef<RasterSchema>),
}
impl GeospatialAssetSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        match self {
            Self::LINZPrimaryParcelsSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::LINZPropertyTitlesSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PointCloudSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PointCloudBoundarySchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PointCloudInfoSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PointCloudMetadataSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::RasterSchema(x) => x.0.read().unwrap().get_attributes(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            Self::LINZPrimaryParcelsSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::LINZPropertyTitlesSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::PointCloudSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::PointCloudBoundarySchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::PointCloudInfoSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::PointCloudMetadataSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::RasterSchema(x) => x.0.read().unwrap().get_datum_template(),
        }
    }
}