#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl ChunkCoord {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }
}
