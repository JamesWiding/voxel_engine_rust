// src/main.rs
use voxel_engine_rust::coords::V3I128;
use voxel_engine_rust::world::WorldManager;
use voxel_engine_rust::game_loop;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Initializing Planetary Voxel Engine (Interactive Core) ---");

    // 1. Initialize Shared World
    let world = Arc::new(Mutex::new(WorldManager::new("interactive_world.db", 0)?));
    
    // 2. Register Player & Spawn Test Spore
    let player_id = 1;
    {
        let mut w = world.lock().await;
        w.register_player(player_id, V3I128::new(0, 0, 0));
        w.simulation.spawn_spore(V3I128::new(2, 1, 2), vec![0.75, -0.5, 0.99]);
    }

    // 3. Launch Interactive Terminal Console
    game_loop::run_interactive_console(world, player_id).await?;

    println!("--- Engine Session Ended Successfully ---");
    Ok(())
}
