use crate::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gunaxe_gun_config: GunConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GunConfig {
    pub gun_depth: usize,
}
