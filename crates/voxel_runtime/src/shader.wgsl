// Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) block_type: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Scale down coordinates temporarily to fit normalized device coordinates [-1, 1]
    out.clip_position = vec4<f32>(model.position * 0.05 - vec3<f32>(0.5, 0.5, 0.0), 1.0);
    
    // Color mapping based on block type
    var col = vec3<f32>(0.2, 0.8, 0.3); // Grass (Green)
    if model.block_type == 1u {
        col = vec3<f32>(0.5, 0.5, 0.5); // Stone (Gray)
    } else if model.block_type == 2u {
        col = vec3<f32>(0.6, 0.4, 0.2); // Dirt (Brown)
    }
    out.color = col;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
