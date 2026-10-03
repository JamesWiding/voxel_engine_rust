// src/terrain.rs
use crate::coords::V3I128;
use rayon::prelude::*;

pub struct TerrainGenerator {
    sea_level: i128,
}

impl TerrainGenerator {
    pub fn new(sea_level: i128) -> Self {
        Self { sea_level }
    }

    /// Parallel chunk generation using Rayon
    pub fn generate_chunk_blocks_parallel(&self, chunk_pos: V3I128, chunk_size: usize) -> Vec<u8> {
        let total_size = chunk_size * chunk_size * chunk_size;
        let mut data = vec![0; total_size];
        
        // Parallelize across the Z slices of the chunk
        data.par_chunks_mut(chunk_size * chunk_size)
            .enumerate()
            .for_each(|(_z, slice)| {
                for y in 0..chunk_size {
                    for x in 0..chunk_size {
                        let world_y = chunk_pos.y * (chunk_size as i128) + (y as i128);
                        let index = x + y * chunk_size;
                        
                        if world_y < self.sea_level {
                            slice[index] = 2; // Stone
                        } else if world_y == self.sea_level {
                            slice[index] = 1; // Grass
                        } else {
                            slice[index] = 0; // Air
                        }
                    }
                }
            });

        data
    }
}
