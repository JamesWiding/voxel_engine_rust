// src/net.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde::{Serialize, Deserialize};
use crate::coords::V3I128;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::world::WorldManager;

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    RequestChunk(V3I128),
    ChunkData {
        pos: V3I128,
        data: Vec<u8>,
    },
}

pub async fn start_multi_client_server(addr: &str, world: Arc<Mutex<WorldManager>>, chunk_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("[Network] Multi-client chunk streamer server listening on {}", addr);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        println!("[Network] New client connected from: {}", peer);
        
        let world_clone = Arc::clone(&world);
        
        // Spawn an independent asynchronous task for each connected client
        tokio::spawn(async move {
            let mut len_buf = [0u8; 4];
            if socket.read_exact(&mut len_buf).await.is_err() {
                return;
            }
            let len = u32::from_be_bytes(len_buf) as usize;
            
            let mut buf = vec![0u8; len];
            if socket.read_exact(&mut buf).await.is_err() {
                return;
            }
            
            if let Ok(packet) = bincode::deserialize::<Packet>(&buf) {
                if let Packet::RequestChunk(requested_pos) = packet {
                    println!("[Network] Client requested chunk at ({}, {}, {})", requested_pos.x, requested_pos.y, requested_pos.z);
                    
                    // Fetch chunk safely from the shared world manager across threads
                    let mut world_lock = world_clone.lock().await;
                    if let Ok(chunk_data) = world_lock.get_or_generate_chunk(requested_pos, chunk_size) {
                        let response = Packet::ChunkData {
                            pos: requested_pos,
                            data: chunk_data,
                        };
                        
                        if let Ok(encoded) = bincode::serialize(&response) {
                            let resp_len = (encoded.len() as u32).to_be_bytes();
                            let _ = socket.write_all(&resp_len).await;
                            let _ = socket.write_all(&encoded).await;
                            println!("[Network] Streamed chunk payload to client {}.", peer);
                        }
                    }
                }
            }
        });
    }
}

pub async fn request_chunks_in_radius(addr: &str, center: V3I128, radius: i128) -> Result<Vec<V3I128>, Box<dyn std::error::Error>> {
    let mut fetched_chunks = Vec::new();

    // Iterate across the planetary i128 radius grid
    for x in -radius..=radius {
        for z in -radius..=radius {
            let target_pos = V3I128::new(center.x + x, center.y, center.z + z);
            
            let mut stream = TcpStream::connect(addr).await?;
            let packet = Packet::RequestChunk(target_pos);
            let encoded = bincode::serialize(&packet)?;
            let len_buf = (encoded.len() as u32).to_be_bytes();
            
            stream.write_all(&len_buf).await?;
            stream.write_all(&encoded).await?;
            
            let mut resp_len_buf = [0u8; 4];
            stream.read_exact(&mut resp_len_buf).await?;
            let resp_len = u32::from_be_bytes(resp_len_buf) as usize;
            
            let mut resp_buf = vec![0u8; resp_len];
            stream.read_exact(&mut resp_buf).await?;
            
            let response_packet: Packet = bincode::deserialize(&resp_buf)?;
            if let Packet::ChunkData { pos, data: _ } = response_packet {
                fetched_chunks.push(pos);
            }
        }
    }

    Ok(fetched_chunks)
}
