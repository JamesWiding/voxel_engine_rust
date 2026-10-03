// src/storage.rs
use rusqlite::{Connection, Result, params};
use crate::coords::V3I128;
use crate::simulation::BrainSpore;

pub struct ChunkDatabase {
    conn: Connection,
}

impl ChunkDatabase {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Chunks table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                x TEXT,
                y TEXT,
                z TEXT,
                data BLOB,
                PRIMARY KEY (x, y, z)
            )",
            [],
        )?;

        // Spores persistence table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS spores (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                x TEXT,
                y TEXT,
                z TEXT,
                energy REAL,
                weights BLOB
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn save_chunk(&mut self, pos: V3I128, data: &[u8]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO chunks (x, y, z, data) VALUES (?, ?, ?, ?)"
            )?;
            stmt.execute(params![
                pos.x.to_string(),
                pos.y.to_string(),
                pos.z.to_string(),
                data
            ])?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn load_chunk(&self, pos: V3I128) -> Result<Option<Vec<u8>>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM chunks WHERE x = ? AND y = ? AND z = ?"
        )?;
        let mut rows = stmt.query(params![
            pos.x.to_string(),
            pos.y.to_string(),
            pos.z.to_string(),
        ])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn save_spores(&mut self, spores: &[BrainSpore]) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM spores", [])?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO spores (x, y, z, energy, weights) VALUES (?, ?, ?, ?, ?)"
            )?;
            for spore in spores {
                let serialized_weights = bincode::serialize(&spore.neural_weights).unwrap_or_default();
                stmt.execute(params![
                    spore.position.x.to_string(),
                    spore.position.y.to_string(),
                    spore.position.z.to_string(),
                    spore.energy,
                    serialized_weights
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn load_spores(&self) -> Result<Vec<BrainSpore>> {
        let mut stmt = self.conn.prepare("SELECT x, y, z, energy, weights FROM spores")?;
        let spore_iter = stmt.query_map([], |row| {
            let x: String = row.get(0)?;
            let y: String = row.get(1)?;
            let z: String = row.get(2)?;
            let energy: f32 = row.get(3)?;
            let weights_blob: Vec<u8> = row.get(4)?;
            
            let neural_weights = bincode::deserialize(&weights_blob).unwrap_or_default();
            let position = V3I128::new(
                x.parse().unwrap_or(0),
                y.parse().unwrap_or(0),
                z.parse().unwrap_or(0),
            );

            Ok(BrainSpore {
                position,
                neural_weights,
                energy,
            })
        })?;

        let mut spores = Vec::new();
        for spore in spore_iter {
            if let Ok(s) = spore {
                spores.push(s);
            }
        }
        Ok(spores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_persistence() {
        let mut db = ChunkDatabase::new(":memory:").unwrap();
        let pos = V3I128::new(100, -200, 300);
        let dummy_data = vec![1, 2, 3, 4, 5];

        db.save_chunk(pos, &dummy_data).unwrap();
        let loaded = db.load_chunk(pos).unwrap().unwrap();

        assert_eq!(dummy_data, loaded);
    }
}
