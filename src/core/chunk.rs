pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Clone)]
pub struct Chunk {
    pub blocks: [u16; CHUNK_VOLUME],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [0; CHUNK_VOLUME],
        }
    }

    /// Serialize the fixed array into a raw byte vector for SQLite BLOB storage
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(CHUNK_VOLUME * 2);
        for &block in &self.blocks {
            bytes.extend_from_slice(&block.to_le_bytes());
        }
        bytes
    }

    /// Deserialize a raw SQLite BLOB back into a chunk array safely
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != CHUNK_VOLUME * 2 {
            return Err("Invalid BLOB byte length for chunk deserialization");
        }
        let mut blocks = [0u16; CHUNK_VOLUME];
        for (i, chunk_bytes) in data.chunks_exact(2).enumerate() {
            blocks[i] = u16::from_le_bytes([chunk_bytes[0], chunk_bytes[1]]);
        }
        Ok(Self { blocks })
    }
}
