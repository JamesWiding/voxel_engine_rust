use rusqlite::{Connection, Result, params};

pub struct WorldDatabase {
    conn: Connection,
}

impl WorldDatabase {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                x INTEGER,
                y INTEGER,
                z INTEGER,
                data BLOB,
                PRIMARY KEY (x, y, z)
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn save_chunk(&mut self, x: i64, y: i64, z: i64, data: &[u8]) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "REPLACE INTO chunks (x, y, z, data) VALUES (?1, ?2, ?3, ?4)",
            params![x, y, z, data],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn load_chunk(&self, x: i64, y: i64, z: i64) -> Result<Option<Vec<u8>>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM chunks WHERE x = ?1 AND y = ?2 AND z = ?3"
        )?;
        let mut rows = stmt.query(params![x, y, z])?;
        
        if let Some(row) = rows.next()? {
            let data: Vec<u8> = row.get(0)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}
