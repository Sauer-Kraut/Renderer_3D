@group(0)
@binding(0)

var<storage, read_write> input_colors: array<u32>; // this is used as both input and output for convenience


@group(0)
@binding(1)

var<uniform> dispatch_size: vec3<u32>;


@group(0)
@binding(2)

var<storage, read_write> doubling_list: array<u32>; // this is used as both input and output for convenience



@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    
    // comparing the current pixel to the next pixel
    let index = global_id.x * 4u + global_id.y * dispatch_size.x * 4u;
    let next_index = index + 4u;

    let color_check: bool = (
        (input_colors[index] == input_colors[next_index]) &&
        (input_colors[index + 1u] == input_colors[next_index + 1u]) &&
        (input_colors[index + 2u] == input_colors[next_index + 2u]) &&
        (input_colors[index + 3u] == input_colors[next_index + 3u])
    );

// TODO: Placeholder values
    let bounds_check: bool = (index / (dispatch_size.x * 4u) == next_index / (dispatch_size.x * 4u));

    if (color_check && bounds_check) {
        doubling_list[global_id.x + global_id.y * dispatch_size.x] = 1u;
    }
}