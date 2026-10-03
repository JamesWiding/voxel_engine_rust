// src/game_loop.rs
use crate::world::WorldManager;
use crate::coords::V3I128;
use crate::renderer::VoxelRenderer;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

// src/game_loop.rs (around line 10)
pub async fn run_interactive_console(world: Arc<Mutex<WorldManager>>, player_id: u64) -> io::Result<()> {
    let stdin_lock = io::stdin(); // Removed 'mut' here
    println!("\n=== Planetary Voxel Engine Interactive Terminal ===");
    println!("Commands:");
    println!("  pos <x> <y> <z>  - Move player to new coordinate");
    println!("  render           - Render ASCII slice of current chunk");
    println!("  tick             - Run simulation and physics tick");
    println!("  save             - Persist world state to SQLite");
    println!("  exit             - Shutdown engine");
    println!("==================================================");

    loop {
        print!("\nvoxel-engine> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if stdin_lock.read_line(&mut input)? == 0 {
            break;
        }

        let cmd = input.trim();
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mut w = world.lock().await;

        match parts[0] {
            "exit" => {
                println!("[Engine] Shutting down gracefully...");
                break;
            }
            "tick" => {
                w.tick_world();
                println!("[Simulation] Executed 1 world simulation tick.");
                for spore in &w.simulation.spores {
                    println!(" -> Spore at ({},{},{}) | Energy: {:.1}", spore.position.x, spore.position.y, spore.position.z, spore.energy);
                }
            }
            "save" => {
                if let Ok(()) = w.persist_world() {
                    println!("[Storage] World state and active entities successfully saved to SQLite.");
                } else {
                    eprintln!("[Storage Error] Failed to persist world state.");
                }
            }
            "render" => {
                // Get player position and render chunk slice
                let player_pos = w.players.iter().find(|p| p.id == player_id).map(|p| p.position).unwrap_or(V3I128::new(0,0,0));
                if let Ok(blocks) = w.get_or_generate_chunk(player_pos, 16) {
                    VoxelRenderer::render_ascii_slice(&blocks, 1);
                }
            }
            "pos" => {
                if parts.len() == 4 {
                    if let (Ok(x), Ok(y), Ok(z)) = (parts[1].parse::<i128>(), parts[2].parse::<i128>(), parts[3].parse::<i128>()) {
                        let new_pos = V3I128::new(x, y, z);
                        w.update_player_position(player_id, new_pos);
                        println!("[Player] Position updated to ({}, {}, {})", x, y, z);
                    } else {
                        println!("Invalid coordinates. Must be valid i128 integers.");
                    }
                } else {
                    println!("Usage: pos <x> <y> <z>");
                }
            }
            _ => {
                println!("Unknown command: '{}'. Type 'exit', 'tick', 'save', 'render', or 'pos <x> <y> <z>'.", parts[0]);
            }
        }
    }

    Ok(())
}
