mod chunk;
mod player;
mod mesh;
mod render;

use voxel_storage::WorldDatabase;
use chunk::Chunk;
use mesh::ChunkMesh;

fn main() {
    println!("Initializing Voxel Engine Window Context...");

    let db_path = "render_world.db3";
    let _db = WorldDatabase::open(db_path).expect("Failed to open world database");
    let mut chunk = Chunk::new();
    chunk.generate_terrain(0, 0);

    let render_mesh = ChunkMesh::generate(&chunk);
    println!("Generated chunk render mesh with {} vertices.", render_mesh.vertices.len());

    // Launch window and graphics context using pollster, passing the vertices vector
    pollster::block_on(render::run_window(render_mesh.vertices));
}
