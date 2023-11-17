// Vertex shader!

struct VertexOutput {
    @builtin(position) clip__popsition: vec4<f32>,
};

@Vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1-i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip__popsition = vec4<f32>(x, , 0.0, 1.0);
    return out;
}