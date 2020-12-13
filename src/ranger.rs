#![allow(non_snake_case)]
use crate::error::AoristError;
use async_trait::async_trait;
use ranger::RangerClient;

#[async_trait]
pub trait TRangerEntity {
    type TCreatePayload;
    type TResultPayload;

    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String>;
    async fn create(&self, client: &RangerClient) -> Result<Self::TResultPayload, AoristError>;
    async fn exists(&self, client: &RangerClient) -> Result<bool, AoristError>;
    async fn enforce(&mut self, client: &RangerClient) -> Result<(), AoristError>;
}