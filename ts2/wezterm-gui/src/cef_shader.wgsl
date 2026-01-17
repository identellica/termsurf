// CEF Browser Overlay Shader
// Renders a CEF texture as a full-viewport quad

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

// Full-screen triangle technique: 3 vertices cover the entire viewport
// More efficient than a quad (4 vertices + index buffer)
@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32) -> VertexOutput {
    // Triangle vertices in clip space that cover [-1,1] viewport
    // Vertex 0: (-1, -1) -> bottom-left
    // Vertex 1: (3, -1)  -> extends past right edge
    // Vertex 2: (-1, 3)  -> extends past top edge
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );

    // Texture coordinates: flip Y for correct orientation
    // Maps clip space to [0,1] texture space
    var tex_coords = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0)
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_idx], 0.0, 1.0);
    output.tex_coord = tex_coords[vertex_idx];
    return output;
}

// CEF texture and sampler
@group(0) @binding(0) var cef_texture: texture_2d<f32>;
@group(0) @binding(1) var cef_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the CEF texture
    // The viewport is set to pane bounds, so tex_coord [0,1] maps to the pane area
    let color = textureSample(cef_texture, cef_sampler, input.tex_coord);
    return color;
}
