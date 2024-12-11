mod lib;
use lib::*;
use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http;
use wgpu::RenderPipeline;
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{self};
use std::sync::mpsc;
use actix_files::Files;
use colored::Colorize;
use std::process;
use std::default;
use std::mem::size_of_val;
use std::ops::{Add, Mul, Sub};
use bytemuck;
use wgpu::util::DeviceExt;
use flume;
use tokio;
use std::sync::{Arc};
use tokio::sync::RwLock;
use rand::Rng;







#[derive(Debug, Deserialize, Serialize)]
struct PullRequest {
    title: String,
    colors: Vec<String>,
}

async fn index() -> impl Responder {
    println!("\nGot request, poggies");
    HttpResponse::Ok().body(fs::read_to_string("static/index_renderer.html").await.unwrap())
}

async fn index2d() -> impl Responder {
    println!("Got request, poggies");
    HttpResponse::Ok().body(fs::read_to_string("static/index2D.html").await.unwrap())
}









async fn pull_request(info: web::Json<PullReqeustRecvPackage>, data: web::Data<AppState>) -> impl Responder {
    println!("\n\n\n\n{} {} \ndescription: {}", "Received Pull Request:".bold().cyan(), info.title.bold().italic().cyan(), info.description.italic());
    // let parent = SquareSurface::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), 0.0, 0.0, Color::new(0, 0, 0), & mut vec!());
    let string_matrix = info.matrix.clone();
    let resolution = info.resolution.clone();
    let screen_center = info.camera_position.clone();
    let focus_point = info.focus_point.clone();
    let (pixel_sender, pixel_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();

    let resolution_fit: (u16, u16) = ((resolution.0 - resolution.0 % 64) as u16, (resolution.1 - resolution.1 % 64) as u16);
    println!("using resolution: {:?}", resolution_fit);

    println!("startig async thread");
    tokio::task::spawn(async move {

        println!("async thread has begun");

        let mut points_to_render: Vec<Point> = vec!();
        let mut lines_to_render: Vec<Line> = vec!();

        let resolution_x: u64 = resolution.0;
        let resolution_y: u64 = resolution.1;

        // ratio is width / height

        let aspect_ratio: f32 = resolution_x as f32 / resolution_y as f32;

        let screen_width = 10.0;

        let screen = match screen_center.turn_into_screen(focus_point, aspect_ratio, screen_width){
            Ok(value) => value,
            Err(err) => {error_sender.send(err).unwrap(); panic!()},
        };

        // println!("\nsettig up screen data");

        // let screen_plane = screen.get_plane();
        let camera_position = screen_center + (screen_center.clone() - focus_point).normalize() * 4.0;

        // println!("\nstarting to render");

        // !!!!!!!!!!!!!!!!!!!!!!!!!!
        // ADD DECENT ERROR HANDELING
        // !!!!!!!!!!!!!!!!!!!!!!!!!!




        let mut corners = vec!();

        let mut test_tryangular_pyramid = vec!();

        let corner_position_1 = Vector3D::new(0.0, 0.5, 0.1);
        let corner_position_2 = Vector3D::new(0.4, 0.7, 0.2);
        let corner_position_3 = Vector3D::new(0.5, 0.0, 0.1);
        let corner_position_4 = Vector3D::new(5.0, 0.5, 6.0);

        corners.push(TriangleCorner::new(corner_position_1, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_2, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_3, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_4, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        
        for corner in corners.iter_mut() {
            // println!("rotating vector");
            // println!("creating matrix");
            let rotation_matrix = match string_matrix[0].clone().turn_into_rotation_matrix() {
                Ok(a) => a,
                Err(err) => {error_sender.send(err).unwrap(); panic!()}
            };
            let rotated_corner = rotation_matrix.multiply(corner.position, &info.theta[0]);
            corner.position = rotated_corner;
        }

        test_tryangular_pyramid.push(Triangle::new(vec!(corners[0].clone(), corners[1].clone(), corners[2].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[1].clone(), corners[2].clone(), corners[3].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[2].clone(), corners[3].clone(), corners[0].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[3].clone(), corners[0].clone(), corners[1].clone()), "Placeholder").unwrap());

        test_tryangular_pyramid.sort_by({|a, b| (camera_position - a.center_position()).occurence_length().abs().total_cmp(&(camera_position - b.center_position()).occurence_length().abs())});
        test_tryangular_pyramid.reverse();

        let test_sphere= create_sphere(Vector3D::origin(), 1, 3, 3);
        // println!("{:?}", test_sphere);

        //optimised_screen_points = current_entry.draw(&screen, camera_position, resolution_x as u32, resolution_y as u32, lithing_function(current_entry.center_position(), current_entry.get_plane().unwrap(), Color::new(100, 100, 100), camera_position)).unwrap();
        let mut optimised_screen_points = vec!();
        for (index, current_entry) in test_sphere.iter().enumerate() {
            if index < 0 {
                continue;
            }
            // let current_lithing_condition = lithing_function(current_entry.center_position(), current_entry.get_plane().unwrap(), Color::new(200 as u8, (10 + 10 * index).min(200) as u8, 200 as u8), camera_position);
            let current_lithing_condition = Color::new(200 as u32, (10 + 5 * index).min(200) as u32, 200 as u32);
            optimised_screen_points = OptimisedScreenPoint::layer(optimised_screen_points, current_entry.draw_full(&screen, camera_position, resolution_x as u32, resolution_y as u32, current_lithing_condition).unwrap()).unwrap();
            optimised_screen_points = OptimisedScreenPoint::layer(optimised_screen_points, current_entry.draw(&screen, camera_position, resolution_x as u32, resolution_y as u32, Color::black()).unwrap()).unwrap();
        }




        println!("going to start awaiting lock");
        let locked_renderer_recources = data.render_recources.read().await;
        let locked_grouping_recources = data.compute_recources.read().await;
        println!("got lock :3");

    
        let renderer_recources = &locked_renderer_recources;
        let grouping_recources = &locked_grouping_recources;

        // Perform GPU rendering
        println!("starting to render");
        let shader_result = gpu_render_shader(
            test_tryangular_pyramid,
            resolution_fit.0 as u32,
            resolution_fit.1 as u32,
            renderer_recources
        ).await;
        println!("finished rendering");

        // println!("input for shader data optimisation: {:?}", shader_result);

        println!("starting to optimise");
        let optimised_screen_points = gpu_grouping_shader(
            shader_result, 
            resolution_fit.0 as u32, 
            resolution_fit.1 as u32, 
            grouping_recources
        ).await;
        println!("finished optimising");
            
        // Use the optimized points as needed
        // println!("The final optimised screen points are: \n{:?}", optimised_screen_points);

        // optimised_screen_points = test_polygon.draw(&screen, camera_position, resolution_x as u32, resolution_y as u32, Color::black()).unwrap();

        // for some reason the screen inverts the y-axis when the screen center is higher then the focus point. might one day acctually fix it but for now this should work
        // if screen_center.y > focus_point.y {
        //     optimised_screen_points.reverse();
        // }

        
        pixel_sender.send(optimised_screen_points).unwrap_or_else(|err| println!("{}",err));

        println!("{}", "all done here".bold().green());

        let mut rng = rand::thread_rng();
        let rand_int = rng.gen_range(0..100);

        if rand_int == 0 {
            // process::exit(0);
        }

        // process::exit(0);
    }).await.unwrap();
    println!("arrived behind async thread");
    // println!("trying to receive");
    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); err},
        Err(_) => "No Error detected".to_string(),
    };
    let color_list = pixel_receiver.recv().unwrap();
    // let color_list = optimised_screen_points;
    HttpResponse::Ok().json(PullReqeustSendPackage {
        title: "Server Respons".to_string(),
        description: "results calculated with given data".to_string(),
        resolution: resolution_fit,
        color_list,
        error
    })
}









#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await.unwrap();

    let render_recources = Arc::new(RwLock::new(create_render_shader_recources(&adapter).await));

    let compute_recources = Arc::new(RwLock::new(create_grouping_shader_recources(&adapter).await));



    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("http://therotationrenderer.mywire.org") // Update with your frontend's origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::ORIGIN,
                    http::header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600))
            .app_data(web::Data::new(AppState {
                render_recources: render_recources.clone(),
                compute_recources: compute_recources.clone()
            }))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").to(index))
            .service(web::resource("/3D").to(index))
            .service(web::resource("/2D").to(index2d))
            .service(web::resource("/api/pull-request")
            .route(web::put().to(pull_request)))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}









async fn gpu_render_shader<'a>(input_value: Vec<Triangle<'a>>, resolution_x: u32, resolution_y: u32, recources: &RenderRecources) -> Vec<u32> {

    let flat_input = Triangle::flatten(input_value);

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
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
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

    let dispatch_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Dispatch size Buffer"),
        contents: bytemuck::cast_slice(&vec!(resolution_x, resolution_y, 0)[..]),
        usage:  wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE
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
            resource: dispatch_size_buffer.as_entire_binding()
        }]
    });

    let render_pipeline = &recources.render_pipeline;




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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE), 
                    store: true },
                
            })],
            depth_stencil_attachment: None,
        });
        command.set_pipeline(&render_pipeline);
        command.set_bind_group(0, &binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.set_vertex_buffer(0, vertex_buffer.slice(..));
        command.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        command.draw_indexed(0..(flat_input.0.len() as u32), 0, 0..1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }, 
        wgpu::ImageCopyBuffer {
            buffer: &staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * resolution_x),
                rows_per_image: Some(resolution_y)
            },
        },
        texture_dimensions
    );

    queue.submit(Some(encoder.finish()));




    
    let shader_output = staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    shader_output.map_async(wgpu::MapMode::Read, move |x| sender.send(x).unwrap());

    device.poll(wgpu::Maintain::Wait);

    match receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_output.get_mapped_range();
            let mut result: Vec<u8> = bytemuck::cast_slice(&data).to_vec();

            //println!("Output length: {}", result.len());

            // result.reverse();
            // let mut expanded_result = vec!();

            // for index in 0..resolution_y {
            //     let mut current_row = result.split_off((result.len() as u32 - (resolution_x * 4)) as usize);
            //     current_row.reverse();

            //     let mut pixel_row = vec!();

            //     for inner_index in 0..resolution_x {
            //         let current_pixel = Pixel {
            //             color: Color::new(
            //                 current_row[(inner_index * 4) as usize] as u32,
            //                 current_row[(inner_index * 4 + 1) as usize] as u32,
            //                 current_row[(inner_index * 4 + 2) as usize] as u32,
            //             ),
            //             transparency: current_row[(inner_index * 4 + 3) as usize] as u32,
            //         };

            //         pixel_row.push(current_pixel);
                    
            //     }

            //     expanded_result.push(pixel_row);
            // }

            // let mut screenpoint_result = vec!();
            // let reference_plane = SquareSurface::new(Vector3D::origin(), Vector3D::origin(), Vector3D::origin(), 1.0, 1.0, Color::black(), &mut vec!());

            // for (y_index, row) in expanded_result.iter().enumerate() {

            //     for (x_index, pixel) in row.iter().enumerate() {

            //         let screnpoint = ScreenPoint {
            //             parent: &reference_plane,
            //             x: x_index as i64,
            //             y: y_index as i64,
            //             color: pixel.color
            //         };

            //         screenpoint_result.push(screnpoint);
            //     }
            // }

            // println!("I did it yippy!!!!!!");

            // return OptimisedScreenPoint::optimise_screen_point_collection(screenpoint_result, resolution_y as i64, resolution_x as i64).unwrap();

            let result_32: Vec<u32> = result.iter().map(|x| *x as u32).collect();
            return result_32;
        }
        _ => {panic!("receiver didnt receive go ahead")}
    };
}

pub async fn create_render_shader_recources(adapter: &wgpu::Adapter) -> RenderRecources {

    let required_features = wgpu::Features::BUFFER_BINDING_ARRAY | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY;

    let (render_device, render_queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: required_features,
            limits: wgpu::Limits::downlevel_defaults()
        }, None)
        .await.unwrap();

    
    let render_shader = render_device.create_shader_module(wgpu::include_wgsl!("render_shader.wgsl"));


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
                ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );

    let render_pipeline = render_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label : Some("Render Pipeline"),
        layout: None,
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
            ..Default::default()},
        multisample: wgpu::MultisampleState::default(),
        depth_stencil: None,
        multiview: None
    });

    RenderRecources {
        device: render_device,
        binding_group_layout: render_binding_group_layout,
        render_pipeline,
        queue: render_queue,
    }
}













async fn gpu_grouping_shader(input_value: Vec<u32>, resolution_x: u32, resolution_y: u32, recources: &ComputeGroupingRecources) -> Vec<u8> {

    if input_value.len() as u32 != resolution_x * resolution_y * 4 {
        panic!("input to gpu_compute_shader() did not fit the defined resolution and I was to lazy to change the return type :3")
    }
    
    let device = &recources.device;
    let queue = &recources.queue;



    let empty_doubles: Vec<u32> = vec![0; input_value.len() / 4];

    // needs adjusment for different inputs
    let size = size_of_val(&empty_doubles[..]) as wgpu::BufferAddress;

    // Buffer to get data back from GPU

    let debuging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });

    let resolution_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Resolution Buffer"),
        contents: bytemuck::cast_slice(&vec!(resolution_x, resolution_y, 1)[..]),
        usage:  wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&input_value[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });

    let doubling_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Doubling Buffer"),
        contents: bytemuck::cast_slice(&empty_doubles[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });




    let counting_binding_group_layout = &recources.counting_binding_layout;

    let counting_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &counting_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: resolution_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: doubling_buffer.as_entire_binding()
        }]
    });

    let counting_pipeline = &recources.counting_pipeline;




    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&counting_pipeline);
        command.set_bind_group(0, &counting_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(resolution_x, resolution_y, 1);
    }

    encoder.copy_buffer_to_buffer(&doubling_buffer, 0, &debuging_buffer, 0, size);

    





    let pixel_buffer = storage_buffer;

    // let pixel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Pixel Buffer"),
    //     contents: bytemuck::cast_slice(&input_value[..]),
    //     usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
    // });

    // let output_cumulative_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Storage Buffer"),
    //     contents: bytemuck::cast_slice(&[0]),
    //     usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    // });


    let output_length_vector = vec![0; resolution_y as usize];

    let out_length_size = size_of_val(&output_length_vector[..]) as wgpu::BufferAddress;

    let output_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&output_length_vector[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });


    let output_u32_vector = vec![0; input_value.len()];

    let out_u32_size = size_of_val(&output_u32_vector[..]) as wgpu::BufferAddress;

    let output_u32_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&output_u32_vector[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });


    // let cumulative_length_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
    //     label: None,
    //     size: 4,
    //     usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
    //     mapped_at_creation: false
    // });

    let length_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: out_length_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });

    let output_u32_debugging_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: out_u32_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });





    let grouping_binding_group_layout = &recources.grouping_binding_layout;

    let grouping_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &grouping_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: pixel_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 1,
            resource: resolution_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: doubling_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 3,
            resource: output_length_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 4,
            resource: output_u32_buffer.as_entire_binding()
        }]
    });

    let grouping_pipeline = &recources.grouping_pipeline;






    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&grouping_pipeline);
        command.set_bind_group(0, &grouping_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(1, resolution_y, 1);
    }

    // encoder.copy_buffer_to_buffer(&output_cumulative_length_buffer, 0, &cumulative_length_staging_buffer, 0, 4);
    encoder.copy_buffer_to_buffer(&output_length_buffer, 0, &length_staging_buffer, 0, out_length_size);
    encoder.copy_buffer_to_buffer(&output_u32_buffer, 0, &output_u32_debugging_staging_buffer, 0, out_u32_size);


    queue.submit(Some(encoder.finish()));
    

    let shader_length_output = length_staging_buffer.slice(..);
    let (length_sender, length_receiver) = flume::bounded(1);
    shader_length_output.map_async(wgpu::MapMode::Read, move |x| length_sender.send(x).unwrap());

    let shader_out_u32_debugging_output = output_u32_debugging_staging_buffer.slice(..);
    let (out_u32_debugging_sender, out_u32_debugging_receiver) = flume::bounded(1);
    shader_out_u32_debugging_output.map_async(wgpu::MapMode::Read, move |x| out_u32_debugging_sender.send(x).unwrap());

    // let shader_cumulative_length_output = cumulative_length_staging_buffer.slice(..);
    // let (cumulative_length_sender, cumulative_length_receiver) = flume::bounded(1);
    // shader_cumulative_length_output.map_async(wgpu::MapMode::Read, move |x| cumulative_length_sender.send(x).unwrap());


    device.poll(wgpu::Maintain::Wait);

    // TODO: Might be able to skip this by cumulating u8_length, might take longer because of CPU though
    // let u8_cumulative_length: u32 = match cumulative_length_receiver.recv_async().await {
    //     Ok(Ok(_)) => {
    //         let data = shader_cumulative_length_output.get_mapped_range();
    //         let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

    //         *result.first().unwrap()
    //     }
    //     _ => {0}
    // };

    let grouping_debugging_output: Vec<u32> = match out_u32_debugging_receiver.recv_async().await {
            Ok(Ok(_)) => {
                let data = shader_out_u32_debugging_output.get_mapped_range();
                let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    
                // println!("{} {:?}", "grouping u32 output:".bold(), result);
                result
            }
            _ => {panic!("failed to receive output during buffer mapping")}
        };

    let u8_length: Vec<u32> = match length_receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_length_output.get_mapped_range();
            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

            result
        }
        _ => {panic!("failed to receive output during buffer mapping")}
    };

    let u8_cumulative_length: u32 = u8_length.iter().sum();




    let u8_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&u8_length[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
    });

    let reduced_output_u8_vector = vec![0; u8_cumulative_length as usize];

    let reduced_out_u8_size = size_of_val(&reduced_output_u8_vector[..]) as wgpu::BufferAddress;

    let reduced_output_u8_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&reduced_output_u8_vector[..]),
        usage:  wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: reduced_out_u8_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false
    });




    let reducing_binding_group_layout = &recources.reducing_binding_layout;

    let reducing_binding_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &reducing_binding_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: resolution_buffer.as_entire_binding()
        },wgpu::BindGroupEntry {
            binding: 1,
            resource: u8_length_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 2,
            resource: output_u32_buffer.as_entire_binding()
        }, wgpu::BindGroupEntry {
            binding: 3,
            resource: reduced_output_u8_buffer.as_entire_binding()
        }]
    });

    let reducing_pipeline = &recources.reducing_pipeline;


    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None
    });
    {
        let mut command = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None
        });
        command.set_pipeline(&reducing_pipeline);
        command.set_bind_group(0, &reducing_binding_group, &[]);
        command.insert_debug_marker("debug marker");
        command.dispatch_workgroups(1, resolution_y, 1);
    }

    encoder.copy_buffer_to_buffer(&reduced_output_u8_buffer, 0, &staging_buffer, 0, reduced_out_u8_size);















    queue.submit(Some(encoder.finish()));
    

    let shader_debuging_output = debuging_buffer.slice(..);
    let (debuging_sender, debuging_receiver) = flume::bounded(1);
    shader_debuging_output.map_async(wgpu::MapMode::Read, move |x| debuging_sender.send(x).unwrap());

    let shader_output = staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    shader_output.map_async(wgpu::MapMode::Read, move |x| sender.send(x).unwrap());


    device.poll(wgpu::Maintain::Wait);

    match debuging_receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_debuging_output.get_mapped_range();
            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

            // println!("shader debuging output: {:?}", result);
        }
        _ => {panic!("receiver didnt receive go ahead")}
    };

    match receiver.recv_async().await {
        Ok(Ok(_)) => {
            let data = shader_output.get_mapped_range();
            let result: &[u8] = bytemuck::cast_slice(&data);
            // println!("grouping result: {:?}", result);

            return result.to_vec();
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


    
    let counting_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
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
                ty: wgpu::BufferBindingType::Uniform { }, 
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
        }]}
    );


    let doubling_shader = device.create_shader_module(wgpu::include_wgsl!("compute_shader.wgsl"));
    let grouping_shader = device.create_shader_module(wgpu::include_wgsl!("grouping_shader.wgsl"));
    let reducing_shader = device.create_shader_module(wgpu::include_wgsl!("reduction_u8_shader.wgsl"));


    let counting_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("compute pipeline layout"), 
        bind_group_layouts: &[&counting_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let counting_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&counting_pipeline_layout),
        module: &doubling_shader,
        entry_point: "main"

    });

    


    let grouping_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
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
        }, wgpu::BindGroupLayoutEntry {
            binding: 4,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }]}
    );


    let grouping_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("compute pipeline layout"), 
        bind_group_layouts: &[&grouping_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let grouping_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&grouping_pipeline_layout),
        module: &grouping_shader,
        entry_point: "main"

    });




    let reducing_binding_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: None, 
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None },
            count: None,
        }, wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::all(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false }, 
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


    let reducing_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("compute pipeline layout"), 
        bind_group_layouts: &[&reducing_binding_group_layout], 
        push_constant_ranges: &[] 
    });

    let reducing_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&reducing_pipeline_layout),
        module: &reducing_shader,
        entry_point: "main"

    });



    ComputeGroupingRecources {
        device,
        counting_binding_layout: counting_binding_group_layout,
        counting_pipeline,
        grouping_binding_layout: grouping_binding_group_layout,
        grouping_pipeline,
        reducing_binding_layout: reducing_binding_group_layout,
        reducing_pipeline,
        queue,
    }
    
}







#[derive(Copy, Clone)]
#[derive(Debug)]
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Pixel{
    color: Color,
    transparency: u32
}




struct AppState {
    render_recources: Arc<RwLock<RenderRecources>>,
    compute_recources: Arc<RwLock<ComputeGroupingRecources>>
}




pub struct RenderRecources {
    device: wgpu::Device,
    binding_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue
}

pub struct ComputeGroupingRecources {
    device: wgpu::Device,
    counting_binding_layout: wgpu::BindGroupLayout,
    counting_pipeline: wgpu::ComputePipeline,
    grouping_binding_layout: wgpu::BindGroupLayout,
    grouping_pipeline: wgpu::ComputePipeline,
    reducing_binding_layout: wgpu::BindGroupLayout,
    reducing_pipeline: wgpu::ComputePipeline,
    queue: wgpu::Queue
}