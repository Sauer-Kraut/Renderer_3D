@group(0)
@binding(0)

var<uniform> resolution: vec3<u32>;

@group(0)
@binding(1)

var<storage, read_write> output_u32_length: array<u32>;


@group(0)
@binding(2)

var<storage, read_write> output_u32_list: array<u32>;


@group(0)
@binding(3)

var<storage, read_write> output_u8_list: array<u32>;




@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    // !!!!! TODO: THIS IS WRONG, IT INDEXES TO THE WRONG SPOT; NEED INFO ON HOW LONG PREVIOUS ROWS WERE
    var starting_out_u8_index =  0u;

    for (var index = 0u; index < global_id.y; index++) {
        starting_out_u8_index += output_u32_length[index];
    }

    let starting_out_u32_index = global_id.y * resolution.x * 4u;
    
    for (var index = 0u; index < output_u32_length[global_id.y]; index++) {

        let u32_list_index = starting_out_u32_index + index * 4u;
        let u8_list_index = starting_out_u8_index + index;

        let count = output_u32_list[u32_list_index + 3u];

        // if count <= 0u {
        //     break;
        // }

        let red = output_u32_list[u32_list_index];
        let green = output_u32_list[u32_list_index + 1u];
        let blue = output_u32_list[u32_list_index + 2u];

        let combined_value = red + green * 256u + blue * 256u * 256u + count * 256u * 256u * 256u;
        //let combined_value = 40u + 59u * 256u + 255u * 256u * 256u + 17u * 256u * 256u * 256u;

        output_u8_list[u8_list_index] = combined_value;       
    }
}