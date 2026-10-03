use noise::{NoiseFn, Perlin};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Dirt = 2,
    Grass = 3,
}

pub struct Chunk {
    pub blocks: [[[u8; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[BlockType::Air as u8; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }

    pub fn generate_terrain(&mut self, chunk_x: i32, chunk_z: i32) {
        let perlin = Perlin::new(42);

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                let world_x = (chunk_x * CHUNK_WIDTH as i32 + x as i32) as f64 * 0.05;
                let world_z = (chunk_z * CHUNK_DEPTH as i32 + z as i32) as f64 * 0.05;
                
                let noise_val = perlin.get([world_x, world_z]);
                let surface_height = ((noise_val + 1.0) * 0.5 * 40.0 + 30.0) as usize;

                for y in 0..CHUNK_HEIGHT {
                    if y < surface_height - 4 {
                        self.blocks[x][y][z] = BlockType::Stone as u8;
                    } else if y < surface_height {
                        self.blocks[x][y][z] = BlockType::Dirt as u8;
                    } else if y == surface_height {
                        self.blocks[x][y][z] = BlockType::Grass as u8;
                    } else {
                        self.blocks[x][y][z] = BlockType::Air as u8;
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH);
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    bytes.push(self.blocks[x][y][z]);
                }
            }
        }
        bytes
    }
}
