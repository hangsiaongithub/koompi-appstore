#![allow(non_camel_case_types)]
use crate::emuns::{architecture::ARCHITECTURE, license::LICENSE};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct App {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub license: LICENSE,
    pub architecture: Vec<ARCHITECTURE>,
    pub version: String,
    pub build_date: i32,
    pub signature: String,
    pub upstream: String,
    pub tarball_src: String,
    pub owner: String,
    pub owner_website: String,
    pub maintainer: Vec<String>,
    pub build_deps: Vec<String>,
    pub runtime_deps: Vec<String>,
    pub optional_deps: Vec<String>,
    pub conflict_with: Vec<String>,
    pub required_by: Vec<String>,
    pub provide: Vec<String>,
    pub content: Vec<String>,
}
