@group(0)
@binding(0)

var<storage, read_write> pixel_list: array<u32>; // this is used as both input and output for convenience


@group(0)
@binding(1)

var<uniform> resolution: vec3<u32>;


@group(0)
@binding(2)

var<storage, read_write> output_list: array<u32>; // this is used as both input and output for convenience


@group(0)
@binding(3)

var<storage, read_write> output_length: array<u32>;




@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // STAGE 1: comparing
    
    // comparing the current pixel to the next pixel
    let index = global_id.x + global_id.y * resolution.x;
    let next_index = index + 1u;

    let color_check: bool = (
        (pixel_list[index] == pixel_list[next_index])
    );

// TODO: Placeholder values
    let bounds_check: bool = (index / (resolution.x) == next_index / (resolution.x));

    if (color_check && bounds_check) {
        output_list[index * 4u + 3u] = 1u;
    }

}

// [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0]