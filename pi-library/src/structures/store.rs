use super::application::App;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Store {
    pub apps: Vec<App>,
}
