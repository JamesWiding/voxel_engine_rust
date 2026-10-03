#![allow(dead_code)]

pub struct AABB {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl AABB {
    pub fn new(x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64) -> Self {
        Self {
            x,
            y,
            z,
            width,
            height,
            depth,
        }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
            && self.z < other.z + other.depth
            && self.z + self.depth > other.z
    }
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub width: f64,
    pub height: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            width: 0.6,
            height: 1.8,
        }
    }

    pub fn get_aabb(&self) -> AABB {
        AABB::new(self.x - self.width / 2.0, self.y, self.z - self.width / 2.0, self.width, self.height, self.width)
    }

    pub fn apply_gravity(&mut self, dt: f64) {
        self.vy -= 9.81 * dt;
    }

    pub fn update(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.z += self.vz * dt;
    }
}
