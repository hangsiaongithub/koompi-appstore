use async_graphql::*;
// use serde::ser::Serializer;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Store {
    pub applications: Vec<Application>,
}

#[async_graphql::Object]
impl Store {
    async fn applications(&self) -> &Vec<Application> {
        &self.applications
    }
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

#[async_graphql::Object]
impl Language {
    async fn percentage(&self) -> &str {
        &self.percentage
    }
    async fn name(&self) -> &str {
        &self.name
    }
}

// Provide =========================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Provide {
    pub r#type: String,
    pub text: String,
}

#[async_graphql::Object]
impl Provide {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn text(&self) -> &str {
        &self.text
    }
}
// Release =========================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Release {
    pub r#type: String,
    pub version: String,
    pub timestamp: String,
    pub infos: Vec<String>,
}

#[async_graphql::Object]
impl Release {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn version(&self) -> &str {
        &self.version
    }
    async fn timestamp(&self) -> &str {
        &self.timestamp
    }
    async fn infos(&self) -> &Vec<String> {
        &self.infos
    }
}

// Keyword =========================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub lang: String,
    pub keys: Vec<String>,
}

#[async_graphql::Object]
impl Keyword {
    async fn lang(&self) -> &str {
        &self.lang
    }
    async fn keys(&self) -> &Vec<String> {
        &self.keys
    }
}

// URL =============================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Url {
    pub r#type: String,
    pub text: String,
}

#[async_graphql::Object]
impl Url {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn text(&self) -> &str {
        &self.text
    }
}

// Launchable ======================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Launchable {
    pub r#type: String,
    pub text: String,
}

#[async_graphql::Object]
impl Launchable {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn text(&self) -> &str {
        &self.text
    }
}

// ScreenShot ======================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScreenShot {
    pub r#type: String,
    pub data: Vec<Shot>,
}
#[async_graphql::Object]
impl ScreenShot {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn data(&self) -> &Vec<Shot> {
        &self.data
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shot {
    pub r#type: ShotEnum,
    pub text: String,
}

#[async_graphql::Object]
impl Shot {
    async fn r#type(&self) -> &ShotEnum {
        &self.r#type
    }
    async fn text(&self) -> &str {
        &self.text
    }
}

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, EnumString)]
#[Enum(desc = "One of the films in the Star Wars Trilogy")]
#[derive(Debug, Serialize, Deserialize)]
pub enum ShotEnum {
    #[item(desc = "Released in 1977.")]
    image,
    #[item(desc = "Released in 1977.")]
    caption,
}
// #[async_graphql::Object]
// impl ShotEnum {
//     async fn image(&self) -> &ShotEnum {
//         &self
//     }
//     async fn image(&self) -> &ShotEnum {
//         &self
//     }
// }

// Input ============================================= //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Input {
    pub lang: String,
    pub value: String,
}
#[async_graphql::Object]
impl Input {
    async fn lang(&self) -> &str {
        &self.lang
    }
    async fn value(&self) -> &str {
        &self.value
    }
}

// Icon ============================================= //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Icon {
    pub r#type: String,
    pub width: u8,
    pub height: u8,
    pub name: String,
}
#[async_graphql::Object]
impl Icon {
    async fn r#type(&self) -> &str {
        &self.r#type
    }
    async fn width(&self) -> &u8 {
        &self.width
    }
    async fn height(&self) -> &u8 {
        &self.height
    }
    async fn name(&self) -> &str {
        &self.name
    }
}
// Description ====================================== //

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    pub lang: String,
    pub data: Vec<DOM>,
}

#[async_graphql::Object]
impl Description {
    async fn lang(&self) -> &str {
        &self.lang
    }
    async fn data(&self) -> &Vec<DOM> {
        &self.data
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DOM {
    pub r#type: DOM_OBJ,
    pub text: String,
    pub chlidren: Vec<DOM>,
}
#[async_graphql::Object]
impl DOM {
    async fn r#type(&self) -> &DOM_OBJ {
        &self.r#type
    }
}

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, EnumString)]
#[Enum(desc = "One of the films in the Star Wars Trilogy")]
#[derive(Debug, Serialize, Deserialize)]
pub enum DOM_OBJ {
    #[item(desc = "Released in 1977.")]
    p,
    #[item(desc = "Released in 1977.")]
    li,
    #[item(desc = "Released in 1977.")]
    ul,
    #[item(desc = "Released in 1977.")]
    h1,
    #[item(desc = "Released in 1977.")]
    h2,
    #[item(desc = "Released in 1977.")]
    h3,
    #[item(desc = "Released in 1977.")]
    h4,
    #[item(desc = "Released in 1977.")]
    h5,
    #[item(desc = "Released in 1977.")]
    a,
}

impl Default for DOM_OBJ {
    fn default() -> DOM_OBJ {
        DOM_OBJ::p
    }
}

// pub trait Serialize {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer;
// }
