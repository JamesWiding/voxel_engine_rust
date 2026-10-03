use std::collections::HashMap;
use crate::core::coords::ChunkCoord;
use crate::core::chunk::Chunk;
use crate::storage::db::SystemRegistryDB;
use rusqlite::Result;

pub struct WorldManager {
    db: SystemRegistryDB,
    active_chunks: HashMap<ChunkCoord, Chunk>,
}

impl WorldManager {
    /// Initialize the world manager with a path to the SQLite system registry partition
    pub fn new(db_path: &str) -> Result<Self> {
        let db = SystemRegistryDB::new(db_path)?;
        Ok(Self {
            db,
            active_chunks: HashMap::new(),
        })
    }

    /// Get a chunk from active memory, or load it from the SQLite database if missing
    pub fn get_or_load_chunk(&mut self, coord: &ChunkCoord) -> Result<Option<&Chunk>> {
        if self.active_chunks.contains_key(coord) {
            return Ok(self.active_chunks.get(coord));
        }

        // Fallback to loading from the SQLite database BLOB storage
        if let Some(chunk) = self.db.load_node_state(coord)? {
            self.active_chunks.insert(*coord, chunk);
            Ok(self.active_chunks.get(coord))
        } else {
            Ok(None)
        }
    }

    /// Insert or update a chunk in active memory and persist it to the database partition
    pub fn save_chunk_state(&mut self, coord: ChunkCoord, chunk: Chunk) -> Result<()> {
        self.db.write_node_state(&coord, &chunk)?;
        self.active_chunks.insert(coord, chunk);
        Ok(())
    }
}
