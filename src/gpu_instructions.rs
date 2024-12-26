// use lib::*;
use crate::{render_lib::*, Mat4, Vector3D};
use wgpu::Color as GPUColor;
use core::panic;
use std::mem::size_of_val;
use bytemuck;
use wgpu::util::DeviceExt;
use flume;
use std::f64::consts::PI;









pub async fn gpu_render_shader<'a>(input_value: Vec<Triangle<'a>>, resolution_x: u32, resolution_y: u32, recources: &RenderRecources, camera: Camera) -> wgpu::Buffer {

    let flat_input: (Vec<TriangleCorner>, Vec<u32>) = Triangle::flatten(input_value);
    println!("flattend render input: {:?}", flat_input);

    let device = &recources.device;
    let queue = &recources.queue;

    let texture_dimensions = wgpu::Extent3d {
        width: resolution_x,
        height: resolution_y,
        depth_or_array_layers: 1
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output texture"),
        size: texture_dimensions,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm]
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());




    // needs adjusment for different inputs
    let size = (4 * resolution_x * resolution_y) as wgpu::BufferAddress;

    // Buffer to get data back from GPU
    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&flat_input.0[..]),
        usage:  wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&flat_input.1[..]),
        usage:  wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE
    });

    let transformation_matrix = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Dispatch size Buffer"),
        contents: bytemuck::cast_slice(&(create_transformation_matrix(camera).matrix)),
        usage:  wgpu::BufferUsages::UNIFORM
    });





    let binding_group_layout = &recources.binding_group_layout;


    let binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: vertex_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: index_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: transformation_matrix.as_entire_binding()
        }]
    });

    let render_pipeline = &recources.render_pipeline;
    let outline_pipeline = &recources.outline_pipeline;


    let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth texture"),
        size: texture_dimensions,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
    });

    let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());




    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(GPUColor {r:245.0 / 255.0, g:245.0 / 255.0, b:245.0 / 255.0, a:1.0}), 
                    store: true },
                
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                view: &depth_texture_view, 
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }), 
                stencil_ops: None 
            }),
        });
        command.set_pipeline(&render_pipeline);
        command.set_bind_group(0, &binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.set_vertex_buffer(0, vertex_buffer.slice(..));
        command.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        command.draw_indexed(0..(flat_input.1.len() as u32), 0, 0..1);
        command.set_pipeline(&outline_pipeline);
        command.draw_indexed(0..(flat_input.1.len() as u32), 0, 0..1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }, 
        wgpu::ImageCopyBuffer {
            buffer: &output_staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * resolution_x),
                rows_per_image: Some(resolution_y)
            },
        },
        texture_dimensions
    );

    queue.submit(Some(encoder.finish()));

    return output_staging_buffer;


    
    // let shader_output = output_staging_buffer.slice(..);
    // let (sender, receiver) = flume::bounded(1);
    // shader_output.map_async(wgpu::MapMode::Read, move |x| sender.send(x).unwrap());

    // device.poll(wgpu::Maintain::Wait);

    // match receiver.recv_async().await {
    //     Ok(Ok(_)) => {
    //         let data = shader_output.get_mapped_range();
    //         let mut result: Vec<u8> = bytemuck::cast_slice(&data).to_vec();

    //         //println!("Output length: {}", result.len());

    //         // result.reverse();
    //         // let mut expanded_result = vec!();

    //         // for index in 0..resolution_y {
    //         //     let mut current_row = result.split_off((result.len() as u32 - (resolution_x * 4)) as usize);
    //         //     current_row.reverse();

    //         //     let mut pixel_row = vec!();

    //         //     for inner_index in 0..resolution_x {
    //         //         let current_pixel = Pixel {
    //         //             color: Color::new(
    //         //                 current_row[(inner_index * 4) as usize] as u32,
    //         //                 current_row[(inner_index * 4 + 1) as usize] as u32,
    //         //                 current_row[(inner_index * 4 + 2) as usize] as u32,
    //         //             ),
    //         //             transparency: current_row[(inner_index * 4 + 3) as usize] as u32,
    //         //         };

    //         //         pixel_row.push(current_pixel);
                    
    //         //     }

    //         //     expanded_result.push(pixel_row);
    //         // }

    //         // let mut screenpoint_result = vec!();
    //         // let reference_plane = SquareSurface::new(Vector3D::origin(), Vector3D::origin(), Vector3D::origin(), 1.0, 1.0, Color::black(), &mut vec!());

    //         // for (y_index, row) in expanded_result.iter().enumerate() {

    //         //     for (x_index, pixel) in row.iter().enumerate() {

    //         //         let screnpoint = ScreenPoint {
    //         //             parent: &reference_plane,
    //         //             x: x_index as i64,
    //         //             y: y_index as i64,
    //         //             color: pixel.color
    //         //         };

    //         //         screenpoint_result.push(screnpoint);
    //         //     }
    //         // }

    //         // println!("I did it yippy!!!!!!");

    //         // return OptimisedScreenPoint::optimise_screen_point_collection(screenpoint_result, resolution_y as i64, resolution_x as i64).unwrap();

    //         let result_32: Vec<u32> = result.iter().map(|x| *x as u32).collect();
    //         return result_32;
    //     }
    //     _ => {panic!("receiver didnt receive go ahead")}
    // };
}







pub async fn create_render_shader_recources(adapter: &wgpu::Adapter) -> RenderRecources {

    let required_features = wgpu::Features::BUFFER_BINDING_ARRAY | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY | wgpu::Features::POLYGON_MODE_LINE;

    let (render_device, render_queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: required_features,
            limits: wgpu::Limits::downlevel_defaults()
        }, None)
        .await.unwrap();

    
    let render_shader = render_device.create_shader_module(wgpu::include_wgsl!("render_shader.wgsl"));
    let outline_shader = render_device.create_shader_module(wgpu::include_wgsl!("outline_shader.wgsl"));


    let render_binding_group_layout: wgpu::BindGroupLayout = render_device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: None, 
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );

    let pipeline_layout = render_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&render_binding_group_layout], // Ensure this matches
        push_constant_ranges: &[],
    });

    let depth_stencil_state = wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less, // Common default
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default()
    };

    let render_pipeline = render_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label : Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &render_shader,
            entry_point: "vs_main",
            buffers: &[TriangleCorner::layout()]
        },
        fragment: Some(wgpu::FragmentState {
            module: &render_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            })]
        }),
        primitive: wgpu::PrimitiveState { 
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: None, 
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        multisample: wgpu::MultisampleState::default(),
        depth_stencil: Some(depth_stencil_state),
        multiview: None
    });


    let depth_stencil_state = wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less, // Common default
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default()
    };

    let outline_pipeline = render_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label : Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &outline_shader,
            entry_point: "vs_main",
            buffers: &[TriangleCorner::layout()]
        },
        fragment: Some(wgpu::FragmentState {
            module: &outline_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            })]
        }),
        primitive: wgpu::PrimitiveState { 
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: None, 
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Line,
            conservative: false,
        },
        multisample: wgpu::MultisampleState::default(),
        depth_stencil: Some(depth_stencil_state),
        multiview: None
    });

    RenderRecources {
        device: render_device,
        binding_group_layout: render_binding_group_layout,
        render_pipeline,
        outline_pipeline,
        queue: render_queue
    }
}























pub async fn gpu_grouping_shader(input_value: wgpu::Buffer, resolution_x: u32, resolution_y: u32, recources: &ComputeGroupingRecources) -> Vec<u8> {

    // if input_value.len() as u32 != resolution_x * resolution_y * 4 {
    //     panic!("input to gpu_compute_shader() did not fit the defined resolution and I was to lazy to change the return type :3")
    // }
    
    let device = &recources.device;
    let queue = &recources.queue;



    



    let empty_output: Vec<u32> = vec![0; (resolution_x * resolution_y * 4) as usize];
    let input_sized_value: Vec<u32> = vec![0; (resolution_x * resolution_y) as usize];
    let empty_output_length: Vec<u32> = vec![0; resolution_y as usize];

    // needs adjusment for different inputs
    let output_length_size = size_of_val(&empty_output_length[..]) as wgpu::BufferAddress;
    let input_size = size_of_val(&input_sized_value[..]) as wgpu::BufferAddress;
    println!("Input Size: {:?}", input_size);

    // Buffer to get data back from GPU

    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: input_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });

    let length_output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: output_length_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });

    let resolution_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Resolution Buffer"),
        contents: bytemuck::cast_slice(&vec!(resolution_x, resolution_y, 1)[..]),
        usage:  wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let input_buffer = input_value;

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Doubling Buffer"),
        contents: bytemuck::cast_slice(&empty_output[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });

    let output_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Doubling Buffer"),
        contents: bytemuck::cast_slice(&empty_output_length[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });



    let reducing_compute_stage_1_binding_group_layout = &recources.stage_1_binding_layout;

    let reducing_compute_stage_1_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &reducing_compute_stage_1_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: resolution_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: storage_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 3,
            resource: output_length_buffer.as_entire_binding()
        }]
    });

    let reducing_compute_stage_1_pipeline = &recources.stage_1_pipeline;


    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&reducing_compute_stage_1_pipeline);
        command.set_bind_group(0, &reducing_compute_stage_1_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(resolution_x, resolution_y, 1);
    }

    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);




    let reducing_compute_stage_2_binding_group_layout = &recources.stage_2_binding_layout;

    let reducing_compute_stage_2_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &reducing_compute_stage_2_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: resolution_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: storage_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 3,
            resource: output_length_buffer.as_entire_binding()
        }]
    });

    let reducing_compute_stage_2_pipeline = &recources.stage_2_pipeline;


    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&reducing_compute_stage_2_pipeline);
        command.set_bind_group(0, &reducing_compute_stage_2_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(1, resolution_y, 1);
    }

    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);




    let reducing_compute_stage_3_binding_group_layout = &recources.stage_3_binding_layout;

    let reducing_compute_stage_3_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &reducing_compute_stage_3_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: resolution_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: storage_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 3,
            resource: output_length_buffer.as_entire_binding()
        }]
    });

    let reducing_compute_stage_3_pipeline = &recources.stage_3_pipeline;


    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&reducing_compute_stage_3_pipeline);
        command.set_bind_group(0, &reducing_compute_stage_3_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(1, resolution_y, 1);
    }


    // encoder.copy_buffer_to_buffer(&output_cumulative_length_buffer, 0, &cumulative_length_output_staging_buffer, 0, 4);
    encoder.copy_buffer_to_buffer(&output_length_buffer, 0, &length_output_staging_buffer, 0, output_length_size);
    encoder.copy_buffer_to_buffer(&input_buffer, 0, &output_staging_buffer, 0, input_size);


    queue.submit(Some(encoder.finish()));
    

    let shader_length_output = length_output_staging_buffer.slice(..);
    let (length_sender, length_receiver) = flume::bounded(1);
    shader_length_output.map_async(wgpu::MapMode::Read, move |x| length_sender.send(x).unwrap());

    let shader_output = output_staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    shader_output.map_async(wgpu::MapMode::Read, move |x| sender.send(x).unwrap());

    // let shader_cumulative_length_output = cumulative_length_output_staging_buffer.slice(..);
    // let (cumulative_length_sender, cumulative_length_receiver) = flume::bounded(1);
    // shader_cumulative_length_output.map_async(wgpu::MapMode::Read, move |x| cumulative_length_sender.send(x).unwrap());





    device.poll(wgpu::Maintain::Wait);

    let out_length_list: Vec<u32> = match length_receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_length_output.get_mapped_range();
            let result: &[u32] = bytemuck::cast_slice(&data);
            // println!("grouping result: {:?}", result);
            // println!("I made it here! \nHeres the output length: {:?}", result);

            result.to_vec()
        }
        _ => {panic!("receiver didnt receive go ahead")}
    };

    let abs_length: u32 = out_length_list.iter().sum();

    match receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_output.get_mapped_range();
            let result: &[u8] = bytemuck::cast_slice(&data);
            // println!("grouping result: {:?}", result);
            // println!("I made it here! \nHeres the output: {:?}", result[..1024].to_vec());

            return result[..(abs_length * 4) as usize].to_vec();
        }
        _ => {panic!("receiver didnt receive go ahead")}
    };

    
}










pub async fn create_grouping_shader_recources(adapter: &wgpu::Adapter) -> ComputeGroupingRecources {

    let required_features = wgpu::Features::BUFFER_BINDING_ARRAY | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY | wgpu::Features::VERTEX_WRITABLE_STORAGE;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: required_features,
            limits: wgpu::Limits::downlevel_defaults()
        }, None)
        .await.unwrap();


    
    let reducing_compute_stage_1_shader = device.create_shader_module(wgpu::include_wgsl!("reducing_compute_stage_1_shader.wgsl"));
    let reducing_compute_stage_2_shader = device.create_shader_module(wgpu::include_wgsl!("reducing_compute_stage_2_shader.wgsl"));
    let reducing_compute_stage_3_shader = device.create_shader_module(wgpu::include_wgsl!("reducing_compute_stage_3_shader.wgsl"));


    
    let reducing_compute_stage_1_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: None, 
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );


    let reducing_compute_stage_1_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("reducing compute pipeline layout"), 
        bind_group_layouts: &[&reducing_compute_stage_1_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let reducing_compute_stage_1_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&reducing_compute_stage_1_pipeline_layout),
        module: &reducing_compute_stage_1_shader,
        entry_point: "main"

    });



    let reducing_compute_stage_2_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: None, 
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );


    let reducing_compute_stage_2_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("reducing compute pipeline layout"), 
        bind_group_layouts: &[&reducing_compute_stage_2_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let reducing_compute_stage_2_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&reducing_compute_stage_2_pipeline_layout),
        module: &reducing_compute_stage_2_shader,
        entry_point: "main"

    });


    let reducing_compute_stage_3_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: None, 
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );


    let reducing_compute_stage_3_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("reducing compute pipeline layout"), 
        bind_group_layouts: &[&reducing_compute_stage_3_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let reducing_compute_stage_3_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&reducing_compute_stage_3_pipeline_layout),
        module: &reducing_compute_stage_3_shader,
        entry_point: "main"

    });



    ComputeGroupingRecources {
        device,
        stage_1_binding_layout: reducing_compute_stage_1_binding_group_layout,
        stage_1_pipeline: reducing_compute_stage_1_pipeline,
        stage_2_binding_layout: reducing_compute_stage_2_binding_group_layout,
        stage_2_pipeline: reducing_compute_stage_2_pipeline,
        stage_3_binding_layout: reducing_compute_stage_3_binding_group_layout,
        stage_3_pipeline: reducing_compute_stage_3_pipeline,
        queue
    }
    
}











pub fn create_transformation_matrix(camera: Camera) -> Mat4 {
    // println!("creating transformation matrix :3");

    let positoning_matrix = Mat4::new([
        [1.0, 0.0, 0.0, -camera.position.x],
        [0.0, 1.0, 0.0, -camera.position.y],
        [0.0, 0.0, 1.0, -camera.position.z],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    let theta = PI as f32 * 2.0 - camera.rad_rotation_x;
    let rotation_matrix_x = create_rotation_matrix_x(theta);

    let theta = PI as f32 * 2.0 - camera.rad_rotation_y;
    let rotation_matrix_y = create_rotation_matrix_y(theta);

    let theta = PI as f32 * 2.0 - camera.rad_rotation_z;
    let rotation_matrix_z = create_rotation_matrix_z(theta);

    // println!("creating projection matrix");
    let f = 1.0 / (camera.rad_fov / 2.0).tan();
    let aspect = camera.aspect_ratio;
    let zfar = camera.zfar;
    let znear = camera.znear;
    let projection_matrix = Mat4::new([
        [f/aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, zfar / (zfar - znear), -(znear * zfar) / (zfar - znear)],
        [0.0, 0.0, 1.0, 0.0]
    ]);


    // println!("making matrix product");
    let matrix_product = projection_matrix * rotation_matrix_x * rotation_matrix_y * rotation_matrix_z * positoning_matrix;
    
    // println!("finished creating transformation matrix");
    matrix_product
}


pub fn create_rotation_matrix_x(rad: f32) -> Mat4 {

    let cos_theta = rad.cos();
    let sin_theta = rad.sin();
    let rotation_matrix_x = Mat4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_theta, -sin_theta, 0.0],
        [0.0, sin_theta, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    rotation_matrix_x
}

pub fn create_rotation_matrix_y(rad: f32) -> Mat4 {

    let cos_theta = rad.cos();
    let sin_theta = rad.sin();
    let rotation_matrix_y = Mat4::new([
        [cos_theta, 0.0, sin_theta, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_theta, 0.0, cos_theta, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    rotation_matrix_y
}

pub fn create_rotation_matrix_z(rad: f32) -> Mat4 {

    let cos_theta = rad.cos();
    let sin_theta = rad.sin();
    let rotation_matrix_z = Mat4::new([
        [cos_theta, -sin_theta, 0.0, 0.0],
        [sin_theta, cos_theta, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    rotation_matrix_z
}











pub struct RenderRecources {
    device: wgpu::Device,
    binding_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
    outline_pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue
}

pub struct ComputeGroupingRecources {
    device: wgpu::Device,
    stage_1_binding_layout: wgpu::BindGroupLayout,
    stage_1_pipeline: wgpu::ComputePipeline,
    stage_2_binding_layout: wgpu::BindGroupLayout,
    stage_2_pipeline: wgpu::ComputePipeline,
    stage_3_binding_layout: wgpu::BindGroupLayout,
    stage_3_pipeline: wgpu::ComputePipeline,
    queue: wgpu::Queue,
}


#[derive(Clone)]
pub struct Camera {
    position: Vector3D,
    rad_rotation_x: f32,
    rad_rotation_y: f32,
    rad_rotation_z: f32,
    rad_fov: f32,
    aspect_ratio: f32,
    zfar: f32,
    znear: f32
}


impl Camera {

    pub fn new(position: Vector3D, rad_rotation_x: f32, rad_rotation_y: f32, rad_rotation_z: f32, rad_fov: f32, aspect_ratio: f32, zfar: f32, znear: f32) -> Camera {
        Camera {
            position,
            rad_rotation_x,
            rad_rotation_y,
            rad_rotation_z,
            rad_fov,
            aspect_ratio,
            zfar,
            znear
        }
    }
}