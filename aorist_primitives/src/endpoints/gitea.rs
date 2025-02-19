#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl GiteaConfig {
    #[new]
    fn new(server: String, port: usize, token: String) -> Self {
        GiteaConfig {
            server,
            port,
            token,
        }
    }
}
