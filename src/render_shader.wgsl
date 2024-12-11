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
    @location(2) tex_coordinate: vec2<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coordinate: vec2<f32>
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput{
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 1.0);
    output.frag_position = input.position;
    output.normal = input.normal;
    output.tex_coordinate = input.tex_coordinate;

    return output;
}




struct FragmentInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coordinate: vec2<f32>
}

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32>{
    return vec4(0.0, 255.0, 0.0, 255.0);
}