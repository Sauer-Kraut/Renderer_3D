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
    // STAGE 2: grouping

    let starting_doulbe_index = global_id.y * resolution.x * 4u;

    var count = 1u;
    var out_position = 0u;

    for (var index = 0u; index < resolution.x; index++) {

        let double_index = starting_doulbe_index + index * 4u;
        let out_index = starting_doulbe_index + out_position * 4u;

        let double = output_list[double_index + 3u];
        if double > 0u && (index + 1u) < resolution.x && count < 255u {

            count += 1u;
        }
        else {

            // Read the packed `u32` value
            let packed_value = pixel_list[double_index / 4u];

            let byte4 = (packed_value / (256u * 256u * 256u));
            let byte3 = (packed_value - byte4 * 256u * 256u * 256u) / (256u * 256u);
            let byte2 = (packed_value - byte4 * 256u * 256u * 256u - byte3 * 256u * 256u) / 256u;
            let byte1 = (packed_value - byte4 * 256u * 256u * 256u - byte3 * 256u * 256u - byte2 * 256u);

            

            output_list[out_index] = byte1;
            output_list[out_index + 1u] = byte2;
            output_list[out_index + 2u] = byte3;
            // output_list[out_index + 3u] = pixel_list[pixel_index + 3u];
            output_list[out_index + 3u] = count;

            output_length[global_id.y] += 1u;

            count = 1u;
            out_position += 1u;
        }
    }

}

// [255, 100, 255, 3, 0, 0, 0, 1, 0, 0, 0, 0]