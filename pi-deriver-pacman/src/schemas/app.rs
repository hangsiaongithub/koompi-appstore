use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct App {
    pub FILENAME: String,
    pub NAME: String,
    pub BASE: String,
    pub VERSION: String,
    pub DESC: String,
    pub SUBREPO: String,
    pub CSIZE: String,
    pub ISIZE: String,
    pub MD5SUM: String,
    pub SHA256SUM: String,
    pub PGPSIG: String,
    pub URL: String,
    pub LICENSE: String,
    pub ARCH: String,
    pub BUILDDATE: String,
    pub PACKAGER: String,
    pub PROVIDES: Vec<String>,
    pub DEPENDS: Vec<String>,
}
