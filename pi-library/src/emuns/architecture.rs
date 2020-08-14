#![allow(non_camel_case_types)]
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum ARCHITECTURE {
    aarch64,
    armhf,
    armv5,
    armv6,
    armv7,
    armv8,
    i386,
    riscv,
    x86_64,
}

impl Default for ARCHITECTURE {
    fn default() -> Self {
        ARCHITECTURE::x86_64
    }
}
