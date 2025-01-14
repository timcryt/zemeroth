use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteInfo {
    pub paths: HashMap<String, String>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub shadow_size_coefficient: f32,

    #[serde(default = "default_sub_tile_z")]
    pub sub_tile_z: f32,
}

fn default_sub_tile_z() -> f32 {
    0.0
}
