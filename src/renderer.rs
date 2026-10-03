// src/renderer.rs
pub struct VoxelRenderer;

impl VoxelRenderer {
    /// Renders a cross-section slice of a 16x16x16 chunk payload for visual verification
    pub fn render_ascii_slice(blocks: &[u8], slice_y: usize) {
        println!("--- Visualizing Chunk Slice at Y = {} ---", slice_y);
        for z in 0..16 {
            let mut row_str = String::new();
            for x in 0..16 {
                let index = x + slice_y * 16 + z * 16 * 16;
                let block_id = blocks.get(index).copied().unwrap_or(0);
                
                let symbol = match block_id {
                    0 => ".", // Air
                    1 => "#", // Grass/Surface
                    2 => "X", // Stone
                    _ => "?",
                };
                row_str.push_str(symbol);
            }
            println!("{}", row_str);
        }
        println!("-----------------------------------------");
    }
}
