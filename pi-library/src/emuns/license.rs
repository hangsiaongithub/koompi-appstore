#![allow(non_camel_case_types)]
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum LICENSE {
    APACHE,
    BSD,
    CCANC,
    GPL,
    GPLv2,
    GPLv3,
    ISC,
    LGPLv21,
    LGPLv3,
    MIT,
    MPL,
}

impl Default for LICENSE {
    fn default() -> Self {
        LICENSE::MIT
    }
}
