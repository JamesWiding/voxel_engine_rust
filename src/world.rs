// src/world.rs
use crate::coords::V3I128;
use crate::storage::ChunkDatabase;
use crate::terrain::TerrainGenerator;
use crate::simulation::BrainSporeWorldBuffer;
use rusqlite::Result;

pub struct Player {
    pub id: u64,
    pub position: V3I128,
}

pub struct WorldManager {
    db: ChunkDatabase,
    terrain: TerrainGenerator,
    pub simulation: BrainSporeWorldBuffer,
    pub players: Vec<Player>,
}

impl WorldManager {
    pub fn new(db_path: &str, sea_level: i128) -> Result<Self> {
        let db = ChunkDatabase::new(db_path)?;
        let terrain = TerrainGenerator::new(sea_level);
        let simulation = BrainSporeWorldBuffer::new();

        Ok(Self {
            db,
            terrain,
            simulation,
            players: Vec::new(),
        })
    }

    pub fn register_player(&mut self, id: u64, initial_pos: V3I128) {
        self.players.push(Player { id, position: initial_pos });
    }

    pub fn update_player_position(&mut self, id: u64, new_pos: V3I128) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == id) {
            player.position = new_pos;
        }
    }

    pub fn get_chunks_in_player_radius(&mut self, player_id: u64, radius: i128, chunk_size: usize) -> Result<Vec<(V3I128, Vec<u8>)>> {
        let player_pos = self.players.iter()
            .find(|p| p.id == player_id)
            .map(|p| p.position)
            .unwrap_or(V3I128::new(0, 0, 0));

        let mut results = Vec::new();

        for x in -radius..=radius {
            for z in -radius..=radius {
                let target_pos = V3I128::new(player_pos.x + x, player_pos.y, player_pos.z + z);
                let chunk_data = self.get_or_generate_chunk(target_pos, chunk_size)?;
                results.push((target_pos, chunk_data));
            }
        }

        Ok(results)
    }

    pub fn get_or_generate_chunk(&mut self, pos: V3I128, chunk_size: usize) -> Result<Vec<u8>> {
        if let Some(data) = self.db.load_chunk(pos)? {
            Ok(data)
        } else {
            let blocks = self.terrain.generate_chunk_blocks_parallel(pos, chunk_size);
            self.db.save_chunk(pos, &blocks)?;
            
            if pos.x == 0 && pos.y == 0 && pos.z == 0 {
                self.simulation.spawn_spore(V3I128::new(2, 1, 2), vec![0.88, -0.34, 0.95]);
            }

            Ok(blocks)
        }
    }

    pub fn tick_world(&mut self) {
        self.simulation.simulate_tick();
    }

    pub fn persist_world(&mut self) -> Result<()> {
        self.db.save_spores(&self.simulation.spores)
    }

    pub fn restore_spores(&mut self) -> Result<()> {
        self.simulation.spores = self.db.load_spores()?;
        Ok(())
    }
}
