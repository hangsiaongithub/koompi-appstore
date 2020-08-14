use super::repository::Repo;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub repos: Vec<Repo>,
    pub production: bool,
    pub db_directory: String,
    pub cache_directory: String,
    pub local_db: String,
    pub registry: String,
    pub version_file: String,
    pub executable_ext: String,
    pub installation_target: String,
}
