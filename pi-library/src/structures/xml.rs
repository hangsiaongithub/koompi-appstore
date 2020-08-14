use serde::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Store {
    pub applications: Vec<Application>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: Vec<Input>,
    pub pkgname: String,
    pub summaries: Vec<Input>,
    pub descriptions: Vec<Description>,
    pub screenshots: Vec<ScreenShot>,
    pub categories: Vec<String>,
    pub icons: Vec<Icon>,
    pub launchable: Launchable,
    pub mimetypes: Vec<String>,
    pub urls: Vec<Url>,
    pub keywords: Vec<Keyword>,
    pub releases: Vec<Release>,
    pub provides: Vec<Provide>,
    pub languages: Vec<Language>,
    pub project_licenses: Vec<String>,
    pub developer_name: Vec<Input>,
    pub project_group: Vec<String>,
}
// Language ========================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Language {
    pub percentage: String,
    pub name: String,
}

// Provide =========================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Provide {
    pub r#type: String,
    pub text: String,
}

// Release =========================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Release {
    pub r#type: String,
    pub version: String,
    pub timestamp: String,
    pub infos: Vec<String>,
}

// Keyword =========================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub lang: String,
    pub keys: Vec<String>,
}

// URL =============================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Url {
    pub r#type: String,
    pub text: String,
}

// Launchable ======================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Launchable {
    pub r#type: String,
    pub text: String,
}

// ScreenShot ======================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScreenShot {
    pub r#type: String,
    pub data: Vec<Shot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shot {
    pub r#type: ShotEnum,
    pub text: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, EnumString)]
pub enum ShotEnum {
    image,
    caption,
}

// Input ============================================= //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Input {
    pub lang: String,
    pub value: String,
}

// Icon ============================================= //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Icon {
    pub r#type: String,
    pub width: u8,
    pub height: u8,
    pub name: String,
}

// Description ====================================== //
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    pub lang: String,
    pub data: Vec<DOM>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DOM {
    pub r#type: DOM_OBJ,
    pub text: String,
    pub chlidren: Vec<DOM>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, EnumString)]
pub enum DOM_OBJ {
    p,
    li,
    ul,
    h1,
    h2,
    h3,
    h4,
    h5,
    a,
}

impl Default for DOM_OBJ {
    fn default() -> DOM_OBJ {
        DOM_OBJ::p
    }
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
