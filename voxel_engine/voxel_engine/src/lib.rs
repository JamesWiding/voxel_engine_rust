#[cfg(test)]
mod tests {
    use crate::core::coords::ChunkCoord;
    use crate::core::chunk::Chunk;
    use crate::core::world::WorldManager;

    #[test]
    fn test_world_persistence() {
        let db_path = "test_system_registry.db";
        let _ = std::fs::remove_file(db_path);

        let mut manager = WorldManager::new(db_path).unwrap();
        let coord = ChunkCoord::new(1000000000000, 0, -500000000000);
        let chunk = Chunk::new();

        manager.save_chunk_state(coord, chunk).unwrap();

        let loaded = manager.get_or_load_chunk(&coord).unwrap();
        assert!(loaded.is_some());

        let _ = std::fs::remove_file(db_path);
    }
}
