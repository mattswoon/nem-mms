use serde::{Serialize, Deserialize};
use crate::packages::Package;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    packages: Vec<Package>,
}

impl Config {
    pub fn init() -> Self {
        Config {
            packages: Vec::new()
        }
    }
}
