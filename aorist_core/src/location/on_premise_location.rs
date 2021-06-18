use crate::location::alluxio_location::*;
use crate::location::local_file_system_location::*;
use crate::location::minio_location::*;
use crate::location::postgres_location::*;
use crate::location::sqlite_location::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum OnPremiseLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    LocalFileSystemLocation(LocalFileSystemLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
    #[constrainable]
    SQLiteLocation(SQLiteLocation),
    #[constrainable]
    PostgresLocation(PostgresLocation),
}