mod lib;
use lib::*;
use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http;
use core::panic;
use std::thread::{self};
use std::sync::mpsc;
use actix_files::Files;
use colored::Colorize;

// fn main() {
//     println!("{:?}", Vector3D::new(2.0, 4.0, 2.0));

//     let mut points_to_render: Vec<Point> = vec!();
//     let mut lines_to_render: Vec<Line> = vec!();

//     // ratio is with / hight

//     let aspect_ratio: f32 = (16/9) as f32;

//     let screen_center = Vector3D::new(5.0, 0.0 , 5.0);

//     // creating a with  and hight Vector and normalizing them

//     let screen_width_vector = Vector3D::new(1.0, 0.0, -1.0).normalize();
//     let screen_height_vector = Vector3D::new(0.0, 1.0, 0.0).normalize();

//     let screen_width = 10.0;
//     let screen_height = screen_width / aspect_ratio;

//     // calculating Lower left corner of screen (origin)

//     let screen_origin = screen_center - screen_height_vector * (screen_height / 2.0) - screen_width_vector * (screen_width / 2.0);

//     let screen = SquareSurface::new(screen_origin, screen_width_vector, screen_height_vector, screen_width, screen_height, Color::new(0, 0, 0), &mut lines_to_render);

//     let test_calculation = screen.render_on_surface(Point::new(Vector3D::new(0.0, 0.0, 0.0), Color::new(0, 0, 0)), Vector3D::new(18.0, 0.0, 18.0));
//     println!("\n {:?} \n", test_calculation);

//     let sorted_screen_points = sort_screen_points(vec!(ScreenPoint{
//         parent: &screen,
//         x: 2,
//         y: 12,
//         color: Color::new(2, 3, 23)
//     }, ScreenPoint{
//         parent: &screen,
//         x: 7,
//         y: 12,
//         color: Color::new(2, 3, 23)
//     }, 
//     ScreenPoint{
//         parent: &screen,
//         x: 3,
//         y: 14,
//         color: Color::new(2, 3, 23)
//     }, 
//     ScreenPoint{
//         parent: &screen,
//         x: 21,
//         y: 2,
//         color: Color::new(2, 3, 23)
//     },
//     ScreenPoint{
//         parent: &screen,
//         x: 2,
//         y: 5,
//         color: Color::new(2, 3, 23)
//     },
//     ScreenPoint{
//         parent: &screen,
//         x: 22,
//         y: 17,
//         color: Color::new(2, 3, 23)
//     },
//     ScreenPoint{
//         parent: &screen,
//         x: 324,
//         y: 132,
//         color: Color::new(2, 3, 23)
//     }));
//     println!("\n {:?} \n", sorted_screen_points);
//     println!("\n {:?} \n", screen.get_plane());
//     println!("\n {:?} \n", screen.get_plane().find_vector_interception(Point::new(Vector3D::new(0.0, 0.0, 0.0), Color::new(0, 255, 0)), Vector3D::new(18.0, 0.0 , 18.0)));
//     println!("\n screen: {:?}\n", screen);

//     let screen_point_collection = RelativScreenPosition{
//         parent: &screen,
//         relativ_with: 0.5,
//         relativ_hight: 0.5,
//         color: Color::new(0, 0, 255)
//     }.turn_into_stat_sized_screen_point(2000, 1000, 4);
//     for element in screen_point_collection{
//         println!("screen point location x:{}, y:{}", element.x, element.y);
//     }

//     let optimized_screen_points = OptimisedScreenPoint::optimise_screen_point_collection(sorted_screen_points, 16, 16, Color::new(0, 0, 0));
//     for (index, element) in optimized_screen_points.iter().enumerate() {
//         println!("optimised screen point location :{:?}, index: {}", element, index);
//     }



// }



#[derive(Debug, Deserialize, Serialize)]
struct PullRequest {
    title: String,
    colors: Vec<String>,
}

async fn index() -> impl Responder {
    println!("\nGot request, poggies");
    HttpResponse::Ok().body(fs::read_to_string("static/index.html").await.unwrap())
}

async fn index2d() -> impl Responder {
    println!("Got request, poggies");
    HttpResponse::Ok().body(fs::read_to_string("static/index2D.html").await.unwrap())
}

async fn pull_request(info: web::Json<PullReqeustPackage>) -> impl Responder {
    println!("\n\n\n\n{} {} \ndescription: {}", "Received Pull Request:".bold().cyan(), info.title.bold().italic().cyan(), info.description.italic());
    // let parent = SquareSurface::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), 0.0, 0.0, Color::new(0, 0, 0), & mut vec!());
    let string_matrix = info.matrix.clone();
    let resolution = info.resolution.clone();
    let screen_center = info.camera_position.clone();
    let focus_point = info.focus_point.clone();
    let (pixel_sender, pixel_receiver) = mpsc::channel();
    let (vector_sender, vector_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();
    let (layer_sender, layer_receiver) = mpsc::channel();
    thread::spawn(move || {

        let mut points_to_render: Vec<Point> = vec!();
        let mut lines_to_render: Vec<Line> = vec!();

        let resolution_x: u64 = resolution.0;
        let resolution_y: u64 = resolution.1;

        let vector_colors: Vec<Color> = info.vector_colors.clone();

        // ratio is width / height

        let aspect_ratio: f32 = resolution_x as f32 / resolution_y as f32;

        // allready defined before thread
        // let screen_center = Vector3D::new(5.0, 3.0 , 5.0);
        // let focus_point = Vector3D::new(0.0, 2.0, 0.0);


        // let screen_width_vector = Vector3D::new(1.0, 0.0, -1.0).normalize();
        // let screen_height_vector = Vector3D::new(0.0, 1.0, 0.0).normalize();

        let screen_width = 10.0;
        // let screen_height = screen_width / aspect_ratio;

        // calculating Lower left corner of screen (origin)

        // let screen_origin = screen_center - screen_height_vector * (screen_height / 2.0) - screen_width_vector * (screen_width / 2.0);

        //let mut screen = SquareSurface::new(screen_origin, screen_width_vector, screen_height_vector, screen_width, screen_height, Color::new(0, 0, 0), &mut lines_to_render);
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


        println!("rotating vectors");
        let mut rotated_vectors = vec!();
        let mut layers = vec!();

        for (index, vector) in info.vectors.iter().enumerate() {
            // println!("rotating vector");
            // println!("creating matrix");
            let rotation_matrix = match string_matrix[index].clone().turn_into_rotation_matrix() {
                Ok(a) => a,
                Err(err) => {error_sender.send(err).unwrap(); panic!()}
            };
            let mut rotated_vector = rotation_matrix.multiply(vector.clone(), &info.theta[index]);
            let layer = (rotated_vector.clone().normalize() - camera_position).occurence_length();
            rotated_vectors.push(rotated_vector);
            layers.push(layer);
            if rotated_vector.occurence_length() > 10.0 {
                rotated_vector = rotated_vector.normalize() * 10.0;
            }
            for _index in 0..1{
                let arrow_parts = match rotated_vector.get_tip_arrow_vectors(1.0, 30.0){
                    Ok(value) => value,
                    Err(err) => {error_sender.send(err).unwrap(); panic!()},
                };
                lines_to_render.push(Line::new(
                    rotated_vector.clone(), 
                    rotated_vector.clone() + arrow_parts.0,
                    vector_colors[index]));
                lines_to_render.push(Line::new(
                    rotated_vector.clone(), 
                    rotated_vector.clone() + arrow_parts.1,
                    vector_colors[index]));
            }
            lines_to_render.push(Line::new(
                Vector3D::new(0.0, 0.0, 0.0), 
                rotated_vector.clone(),
                vector_colors[index]));
        }

        // Adding 3 coordinate axises

        // let mut coordinate_lines: Vec<Line> = vec!();

        // coordinate_lines.push(Line::new(
        //     Vector3D::origin(),
        //     Vector3D::new(10.0, 0.0, 0.0),
        //     Color::black()
        // ));
        // coordinate_lines.push(Line::new(
        //     Vector3D::origin(),
        //     Vector3D::new(0.0, 12.0, 0.0),
        //     Color::black()
        // ));
        // coordinate_lines.push(Line::new(
        //     Vector3D::origin(),
        //     Vector3D::new(0.0, 0.0, 10.0),
        //     Color::black()
        // ));

        // println!("sending rotated vector");
        vector_sender.send(rotated_vectors).unwrap_or_else(|_| panic!("{}", "AAAAAAAAA, main thread should always be able to receive".bold().red()));
        println!("rotated vector succesfully sent");

        // println!("sending layers");
        layer_sender.send(layers).unwrap_or_else(|_| panic!("{}", "AAAAAAAAA, main thread should always be able to receive".bold().red()));
        println!("layers succesfully sent");

        // println!("\ngenerating points for lines in vector: {:?} \n", lines_to_render);

        // for line in lines_to_render.iter(){
        //      println!("turning line {:?} into points", line);
        //      // line.clone().turn_into_points(&mut points_to_render, 1)
        // }


        // let screen_point_vecs: Vec<Vec<ScreenPoint>> = points_to_render.iter()
        //                                 .map(|point| (screen_plane.find_vector_interception(point, &mut(camera_position - point.location)),  (screen_plane.find_vector_interception(point, &mut(camera_position - point.location)).location - point.location).occurence_length()))
        //                                 .map(|truple| screen.locate_point(truple.0, truple.1).unwrap())
        //                                 .filter(|collision| match collision {RayCollision::Collision(_, _) => true, RayCollision::Miss(_, _) => false})
        //                                 .map(|collision| match collision {RayCollision::Collision(relativ_position, distance) => (relativ_position, distance), RayCollision::Miss(_, _) => panic!("should have been filtered out")})
        //                                 .filter(|truple| truple.1.distance > 0.01)
        //                                 .map(|truple| truple.0.turn_into_dyn_sized_screen_point(resolution_x as u32, resolution_y as u32, truple.1))
        //                                 .collect();
                        
        // let mut line_screen_point_vecs: Vec<Vec<ScreenPoint>> = vec!();
        // for line_to_render in lines_to_render.iter(){
        //     let (result_sender, result_receiver) = mpsc::channel();
        //     let screen_instance = screen.clone();
        //     thread::spawn(move || {
        //         let result = line_to_render.render_line(&screen_instance, camera_position, resolution_x as u32, resolution_y as u32, 0.8).unwrap();
        //         result_sender.send(result).unwrap();
        //     });
        //     line_screen_point_vecs.push(result_receiver.recv().unwrap());
        // }

        // println!("\n{}", "starting to render coordinate system".bold());

        // let coordinate_lines_screen_point_vecs: Vec<Vec<ScreenPoint>> = coordinate_lines.iter()
        //                                 .map(|line| match line.render_line(&screen, camera_position, resolution_x as u32, resolution_y as u32, 0.8){
        //                                     Ok(value) => value,
        //                                     Err(err) => {error_sender.send(err).unwrap(); panic!()},
        //                                 })
        //                                 .collect();

        // println!("{}", "finished rendering coordinate system".bold());
        println!("\n{}", "starting to render Vectors".bold());

        let line_screen_point_vecs: Vec<Vec<ScreenPoint>> = lines_to_render.iter()
                                        .map(|line| match line.render_line(&screen, camera_position, resolution_x as u32, resolution_y as u32, 0.8){
                                            Ok(value) => value,
                                            Err(err) => {error_sender.send(err).unwrap(); panic!()},
                                        })
                                        .collect();

        println!("{}", "finished rendering Vectors".bold());

        // for list in coordinate_lines_screen_point_vecs{
        //     line_screen_point_vecs.push(list);
        // }

        let mut screen_points = vec!();

        // for list in screen_point_vecs {
        //     for element in list{
        //         screen_points.push(element)
        //     }
        // }

        for list in line_screen_point_vecs {
            for element in list{
                screen_points.push(element)
            }
        }

        

        let mut optimised_screen_points = match OptimisedScreenPoint::optimise_screen_point_collection(sort_screen_points(screen_points), resolution_y as i64, resolution_x as i64, Color::new(255, 255, 255)){
            Ok(value) => value,
            Err(err) => {error_sender.send(err).unwrap(); panic!()},
        };

        // for some reason the screen inverts the y-axis when the screen center is higher then the focus point. might one day acctually fix it but for now this should work
        if screen_center.y > focus_point.y {
            optimised_screen_points.reverse();
        }

        
        pixel_sender.send(optimised_screen_points).unwrap_or_else(|err| println!("{}",err));

        println!("{}", "all done here".bold().green());
    });
    // println!("trying to receive");
    let error = match error_receiver.recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); err},
        Err(_) => "No Error detected".to_string(),
    };
    let vectors = vector_receiver.recv().unwrap_or_else(|err| {println!("{}",err); vec!(Vector3D::new(0.0, 0.0, 0.0))});
    let layers = layer_receiver.recv().unwrap_or_else(|err| {println!("{}",err); vec!(0.0)});
    let color_list = pixel_receiver.recv().unwrap_or_else(|_err| vec!());
    HttpResponse::Ok().json(PullReqeustPackage {
        title: "Server Respons".to_string(),
        description: "results calculated with given data".to_string(),
        resolution: resolution,
        matrix: vec!(StringRotationMatrix{line_x: [String::default(), String::default(), String::default()], line_y:[String::default(), String::default(), String::default()], line_z:[String::default(), String::default(), String::default()]}),
        theta: vec!(0.0),
        vectors,
        vector_colors: vec!(),
        layers,
        camera_position: Vector3D::new(0.0, 0.0, 0.0),
        focus_point: Vector3D::new(0.0, 0.0, 0.0),
        color_list,
        error
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
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
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").to(index))
            .service(web::resource("/3D").to(index))
            .service(web::resource("/2D").to(index2d))
            .service(web::resource("/api/pull-request").route(web::put().to(pull_request)))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}

// fn generate_random_screen_point<'a>(parent: &'a SquareSurface) ->  ScreenPoint<'a> {
//     ScreenPoint{
//         parent,
//         x: rand::random::<u8>() as i64,
//         y: rand::random::<u8>() as i64,
//         color: Color::new(255, 0, 0)
//     }
// }