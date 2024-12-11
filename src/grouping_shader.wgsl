@group(0)
@binding(0)

var<storage, read_write> pixel_list: array<vec4<u32>>;

@group(0)
@binding(1)

var<uniform> resolution: vec3<u32>;

@group(0)
@binding(2)

var<storage, read_write> doubling_list: array<u32>;

@group(0)
@binding(3)

var<storage, read_write> output_length: array<u32>;

@group(0)
@binding(4)

var<storage, read_write> output_list: array<vec4<u32>>;




@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    
    let starting_doulbe_index = global_id.y * resolution.x;

    var count = 1u;
    var out_position = 0u;

    for (var index = 0u; index < resolution.x; index++) {

        let double_index = starting_doulbe_index + index;
        let out_index = starting_doulbe_index + out_position;

        let double = doubling_list[double_index];
        if double > 0u && (index + 1u) < resolution.x && count < 255u {

            count += 1u;
        }
        else {

            output_list[out_index].x = pixel_list[double_index].x;
            output_list[out_index].y = pixel_list[double_index].y;
            output_list[out_index].z = pixel_list[double_index].z;
            // output_list[out_index + 3u] = pixel_list[pixel_index + 3u];
            output_list[out_index].w = count;

            output_length[global_id.y] += 1u;

            count = 1u;
            out_position += 1u;
        }
    }
}