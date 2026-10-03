#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3i128 {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl Vector3i128 {
    pub const fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    pub fn distance_squared(&self, other: &Self) -> u128 {
        let dx = (self.x - other.x).unsigned_abs();
        let dy = (self.y - other.y).unsigned_abs();
        let dz = (self.z - other.z).unsigned_abs();
        dx * dx + dy * dy + dz * dz
    }
}
