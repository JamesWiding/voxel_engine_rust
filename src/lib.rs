// src/lib.rs
pub mod coords;
pub mod storage;
pub mod terrain;
pub mod simulation;
pub mod net;
pub mod world;
pub mod renderer;
pub mod game_loop;

#[cfg(test)]
mod integration_tests {
    use crate::coords::V3I128;
    use crate::storage::ChunkDatabase;
    use crate::terrain::TerrainGenerator;
    use crate::simulation::BrainSporeWorldBuffer;

    #[test]
    fn test_full_engine_pipeline() {
        let mut db = ChunkDatabase::new(":memory:").unwrap();
        let terrain = TerrainGenerator::new(0);
        let chunk_pos = V3I128::new(0, 0, 0);
        
        let blocks = terrain.generate_chunk_blocks_parallel(chunk_pos, 16);
        db.save_chunk(chunk_pos, &blocks).unwrap();

        let mut sim_buffer = BrainSporeWorldBuffer::new();
        sim_buffer.spawn_spore(V3I128::new(5, 1, 5), vec![0.5, -0.2, 0.9]);
        sim_buffer.simulate_tick();

        let loaded_chunk = db.load_chunk(chunk_pos).unwrap().unwrap();
        assert_eq!(loaded_chunk.len(), 16 * 16 * 16);
        assert_eq!(sim_buffer.spores.len(), 1);
    }
}
