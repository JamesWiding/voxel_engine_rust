// src/coords.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V3I128 {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl V3I128 {
    pub const fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    pub fn to_chunk_pos(&self, chunk_size: i128) -> V3I128 {
        V3I128 {
            x: self.x.div_euclid(chunk_size),
            y: self.y.div_euclid(chunk_size),
            z: self.z.div_euclid(chunk_size),
        }
    }
}
