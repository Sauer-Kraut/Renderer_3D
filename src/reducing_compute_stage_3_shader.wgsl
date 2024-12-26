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
    // STAGE 3: reducing


    // !!!!! TODO: THIS IS WRONG, IT INDEXES TO THE WRONG SPOT; NEED INFO ON HOW LONG PREVIOUS ROWS WERE
    var starting_out_u8_index =  0u;

    for (var index = 0u; index < global_id.y; index++) {
        starting_out_u8_index += output_length[index];
    }

    let starting_out_u32_index = global_id.y * resolution.x * 4u;
    
    for (var index = 0u; index < output_length[global_id.y]; index++) {

        let u32_list_index = starting_out_u32_index + index * 4u;
        let u8_list_index = starting_out_u8_index + index;

        let count = output_list[u32_list_index + 3u];

        // if count <= 0u {
        //     break;
        // }

        let red = output_list[u32_list_index];
        let green = output_list[u32_list_index + 1u];
        let blue = output_list[u32_list_index + 2u];

        let combined_value = red + green * 256u + blue * 256u * 256u + count * 256u * 256u * 256u;
        //let combined_value = 40u + 59u * 256u + 255u * 256u * 256u + 17u * 256u * 256u * 256u;

        pixel_list[u8_list_index] = combined_value;       
    }
}