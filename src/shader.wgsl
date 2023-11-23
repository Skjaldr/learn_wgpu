// Vertex shader!
// createing a new vertex input struct to get data from the buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// struct to store the output of the vertex shaderd
// @builtin(position) tells WGPU that this is the value we use as the vertex's clip
// coordinates.  Similar to GLSL's gl_position.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};


// @vertex is used to mark the function as a valid entry point to the vertex shader
@vertex
fn vs_main(
    model: VertexInput,
    //@builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// An expected u32 titled in_vertex_index gets values from @builtin(vertex_index)
//  a variable 'out' is declared using VertexOutput, then create x and y for the triangle
// f32(i32()) are small examples of casting.  Similar to C++/Java.

