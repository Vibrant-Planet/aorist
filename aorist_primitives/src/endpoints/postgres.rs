#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PostgresConfig {
    pub server: String,
    pub port: usize,
    pub username: String,
    pub password: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl PostgresConfig {
    #[new]
    fn new(server: String, port: usize, username: String, password: String) -> Self {
        PostgresConfig {
            server,
            port,
            username,
            password,
        }
    }
}
