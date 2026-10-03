#![allow(dead_code)]

use crate::chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH, BlockType};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],  // 3 floats (12 bytes)
    pub normal: [f32; 3],    // 3 floats (12 bytes)
    pub block_type: u8,      // 1 byte (+ padding)
}

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl ChunkMesh {
    pub fn generate(chunk: &Chunk) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let block = chunk.blocks[x][y][z];
                    if block != BlockType::Air as u8 {
                        let fx = x as f32;
                        let fy = y as f32;
                        let fz = z as f32;

                        vertices.push(Vertex {
                            position: [fx, fy, fz],
                            normal: [0.0, 1.0, 0.0],
                            block_type: block,
                        });
                        indices.push(index_offset);
                        index_offset += 1;
                    }
                }
            }
        }

        Self { vertices, indices }
    }
}
