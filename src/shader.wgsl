// Vertex shader!


// struct to store the output of the vertex shaderd
// @builtin(position) tells WGPU that this is the value we use as the vertex's clip
// coordinates.  Similar to GLSL's gl_position.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};


// @vertex is used to mark the function as a valid entry point to the vertex shader
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1-i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

// An expected u32 titled in_vertex_index gets values from @builtin(vertex_index)
//  a variable 'out' is declared using VertexOutput, then create x and y for the triangle
// f32(i32()) are small examples of casting.  Similar to C++/Java.

