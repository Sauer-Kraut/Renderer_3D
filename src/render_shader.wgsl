@group(0) @binding(2)
var<uniform> trans_mat: mat4x4<f32>;


struct Color {
    red: u32,
    green: u32,
    blue: u32
}

struct Pixel{
    color: Color,
    transparency: f32
}




struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coordinate: vec2<f32>,
    @builtin(vertex_index) vertex_index: u32
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coordinate: vec2<f32>,
    @location(3) barycentrics: vec3<f32>
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput{
    var output: VertexOutput;
    var pos = vec4<f32>(input.position, 1.0);
    output.position = pos;
    output.frag_position = input.position;
    output.normal = input.normal;
    output.tex_coordinate = input.tex_coordinate;

    // output.barycentrics = vec3<f32>(f32(input.vertex_index) / 4.0, 0.0, 1.0 - f32(input.vertex_index) / 4.0);

    return output;
}




struct FragmentInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coordinate: vec2<f32>,
    @location(3) barycentrics: vec3<f32>
}

@fragment
fn fs_main(@builtin(position) clip_pos: vec4<f32>, input: FragmentInput) -> @location(0) vec4<f32>{

    return vec4(0.0, 0.0, 0.0, 255.0);
    

    // return vec4(input.barycentrics.x, 0.0, input.barycentrics.z, 255.0);
}