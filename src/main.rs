mod lib;
mod gpu_instructions;
mod calc_structs;
mod server_communication;
mod render_lib;
use server_communication::*;
use gpu_instructions::*;
// use lib::*;
use render_lib::*;
use calc_structs::*;
use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http;
use wgpu::{Color as GPUColor, RenderPipeline};
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
    let camera_position = info.camera_position.clone();
    let focus_point = info.focus_point.clone();
    let theta = info.theta.clone();
    let (pixel_sender, pixel_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();

    let resolution_fit: (u16, u16) = ((resolution.0 - resolution.0 % 64) as u16, (resolution.1 - resolution.1 % 64) as u16);
    println!("using resolution: {:?}", resolution_fit);

    println!("startig async thread");
    tokio::task::spawn(async move {

        println!("async thread has begun");

        // let mut points_to_render: Vec<Point> = vec!();
        let mut lines_to_render: Vec<Line> = vec!();

        let resolution_x: u64 = resolution.0;
        let resolution_y: u64 = resolution.1;

        // ratio is width / height

        let aspect_ratio: f32 = resolution_x as f32 / resolution_y as f32;

        let screen_width = 10.0;

        // let screen = match screen_center.turn_into_screen(focus_point, aspect_ratio, screen_width){
        //     Ok(value) => value,
        //     Err(err) => {error_sender.send(err).unwrap(); panic!()},
        // };

        // println!("\nsettig up screen data");

        // let screen_plane = screen.get_plane();
        // let camera_position = screen_center + (screen_center.clone() - focus_point).normalize() * 4.0;

        // println!("\nstarting to render");

        // !!!!!!!!!!!!!!!!!!!!!!!!!!
        // ADD DECENT ERROR HANDELING
        // !!!!!!!!!!!!!!!!!!!!!!!!!!




        let mut corners = vec!();

        let mut test_tryangular_pyramid = vec!();

        let corner_position_1 = Vector3D::new(0.0, 0.0, 0.0);
        let corner_position_2 = Vector3D::new(0.0, 3.0, 0.0);
        let corner_position_3 = Vector3D::new(3.0, 0.0, 0.0);
        let corner_position_4 = Vector3D::new(0.0, 0.0, 3.0);

        corners.push(TriangleCorner::new(corner_position_1, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_2, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_3, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        corners.push(TriangleCorner::new(corner_position_4, Vector3D::new(0.0, 1.0, 0.0), Vector2D::origin()));
        
        // for corner in corners.iter_mut() {
        //     // println!("rotating vector");
        //     // println!("creating matrix");
        //     let rotation_matrix = match string_matrix[0].clone().turn_into_rotation_matrix() {
        //         Ok(a) => a,c
        //         Err(err) => {error_sender.send(err).unwrap(); panic!()}
        //     };
        //     let rotated_corner = rotation_matrix.multiply(corner.position, &info.theta[0]);
        //     corner.position = rotated_corner;
        // }

        if false {
            error_sender.send("error".to_owned());
        }

        test_tryangular_pyramid.push(Triangle::new(vec!(corners[0].clone(), corners[1].clone(), corners[2].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[1].clone(), corners[2].clone(), corners[3].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[2].clone(), corners[3].clone(), corners[0].clone()), "Placeholder").unwrap());
        test_tryangular_pyramid.push(Triangle::new(vec!(corners[3].clone(), corners[0].clone(), corners[1].clone()), "Placeholder").unwrap());

        test_tryangular_pyramid.sort_by({|a, b| (camera_position - a.center_position()).occurence_length().abs().total_cmp(&(camera_position - b.center_position()).occurence_length().abs())});
        test_tryangular_pyramid.reverse();

        // test_tryangular_pyramid = test_tryangular_pyramid.iter().enumerate().filter(|(index, _entry)| *index == (info.theta[0] % test_tryangular_pyramid.len() as f32) as usize).map(|(_index, entry)| entry.clone()).collect();

        //let test_sphere= create_sphere(Vector3D::origin(), 1, 3, 3);
        // println!("{:?}", test_sphere);

        //optimised_screen_points = current_entry.draw(&screen, camera_position, resolution_x as u32, resolution_y as u32, lithing_function(current_entry.center_position(), current_entry.get_plane().unwrap(), Color::new(100, 100, 100), camera_position)).unwrap();

        // let mut optimised_screen_points = vec!();
        // for (index, current_entry) in test_sphere.iter().enumerate() {
        //     if index < 0 {
        //         continue;
        //     }
        //     // let current_lithing_condition = lithing_function(current_entry.center_position(), current_entry.get_plane().unwrap(), Color::new(200 as u8, (10 + 10 * index).min(200) as u8, 200 as u8), camera_position);
        //     let current_lithing_condition = Color::new(200 as u32, (10 + 5 * index).min(200) as u32, 200 as u32);
        //     optimised_screen_points = OptimisedScreenPoint::layer(optimised_screen_points, current_entry.draw_full(&screen, camera_position, resolution_x as u32, resolution_y as u32, current_lithing_condition).unwrap()).unwrap();
        //     optimised_screen_points = OptimisedScreenPoint::layer(optimised_screen_points, current_entry.draw(&screen, camera_position, resolution_x as u32, resolution_y as u32, Color::black()).unwrap()).unwrap();
        // }

        let rotation_x = 2.0*3.141 - *theta.get(1).unwrap_or_else(|| &0.0); 
        let rotation_y = 2.0*3.141 - *theta.get(0).unwrap_or_else(|| &0.0); 
        let rotation_z = 2.0*3.141 - *theta.get(2).unwrap_or_else(|| &0.0);

        let (adjusted_camera_pos, _w) = (create_rotation_matrix_z(rotation_z)) * create_rotation_matrix_y(rotation_y) * create_rotation_matrix_x(rotation_x) * (camera_position, 1.0 as f32);

        let camera = Camera::new(
            adjusted_camera_pos,
            rotation_x, 
            rotation_y, 
            rotation_z, 
            3.141 / 2.0, 
            1.0, 
            20.0,
            0.1
        );

        let matrix = create_transformation_matrix(camera.clone());
        // for corner in corners.iter() {
        //     let mut transformed_corner = matrix * (corner.position, 1.0 as f32);
        //     transformed_corner.0 = transformed_corner.0 * (1.0/transformed_corner.1);
        //     println!("{:?}", transformed_corner);   
        //     // let test_output = matrix * (Vector3D::new(2.0, -1.0, 3.0), 1.0 as f32);
        //     // println!("test output: {:?}", test_output);
        // }
        println!("matrix:  {:?}", matrix.matrix);

        let mut model = Model::import_obj("static/assets/models/Cylinder/Cylinder.obj").unwrap();
        for face in model.faces.iter_mut() {
            for corner in face.corners.iter_mut() {
                corner.position = Vector3D::new(corner.position.x * 3.0, corner.position.y * 3.0, corner.position.z * 0.1);
            }
        }


        println!("going to start awaiting lock");
        let locked_renderer_recources = data.render_recources.read().await;
        let locked_grouping_recources = data.compute_recources.read().await;
        println!("got lock :3");

    
        let renderer_recources = &locked_renderer_recources;
        let grouping_recources = &locked_grouping_recources;

        // Perform GPU rendering
        println!("starting to render");
        let shader_result = gpu_instructions::gpu_render_shader(
            model.faces,
            resolution_fit.0 as u32,
            resolution_fit.1 as u32,
            renderer_recources,
            camera
        ).await;
        println!("finished rendering");

        // println!("input for shader data optimisation: {:?}", shader_result);

        println!("starting to optimise");
        let optimised_screen_points = gpu_instructions::gpu_grouping_shader(
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
                .allowed_origin("https://therotationrenderer.mywire.org") // Update with your frontend's origin
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}




















struct AppState {
    render_recources: Arc<RwLock<RenderRecources>>,
    compute_recources: Arc<RwLock<ComputeGroupingRecources>>
}