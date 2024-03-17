use std::ops::{Add, Mul, Sub};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;
use futures::stream::Enumerate;
use serde::{Deserialize, Serialize};
use std::{thread, vec};
use std::time::Duration;



#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Vector3D {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl Vector3D{
    
    pub fn new(x:f32, y:f32, z:f32) -> Vector3D{
        Vector3D{
            x,
            y,
            z
        }
    }

    pub fn origin() -> Vector3D{
        Vector3D{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn occurence_length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) -> Vector3D{
        let occurence_length = self.occurence_length();
        
        if occurence_length != 0.0 {
            self.x /= occurence_length;
            self.y /= occurence_length;
            self.z /= occurence_length;
        }
        *self
    }

    pub fn get_orthagonal(&self, other: &Vector3D) -> Result<Vector3D, &str>{
        // set z and y value of output to 1 since it can be any value as long as x is acordingly set
        let mut output = Vector3D::new(1.0, 1.0, 1.0);
        let mut second_try = false;
        let mut vector_1 = self;
        let mut vector_2 = other;
        loop {
            // The following formulas where derifed in "orthagonal vector formulas.png"
            // This code is a bit troublesome since there are some cenarios where you are going to divide by zero if you arent carefull. Therefore a bunch of if clauses have been added
            let mut offset = 0.0;

            if (vector_1.x.abs() < 0.01 && vector_2.x.abs() < 0.01) && (vector_1.y.abs() > 0.01 || vector_2.y.abs() > 0.01) && (vector_1.z.abs() > 0.01 || vector_2.z.abs() > 0.01){
                output.y = 0.0;
                output.z = 0.0;
                return Ok(output);
            } else if (vector_1.y.abs() < 0.01 && vector_2.y.abs() < 0.01) && (vector_1.x.abs() > 0.01 || vector_2.x.abs() > 0.01) && (vector_1.z.abs() > 0.01 || vector_2.z.abs() > 0.01){
                output.x = 0.0;
                output.z = 0.0;
                return Ok(output);
            } else if (vector_1.z.abs() < 0.01 && vector_2.z.abs() < 0.01) && (vector_1.x.abs() > 0.01 || vector_2.x.abs() > 0.01) && (vector_1.y.abs() > 0.01 || vector_2.y.abs() > 0.01){
                output.y = 0.0;
                output.x = 0.0;
                return Ok(output);
            } else if vector_1.x.abs() > 0.02{
                // making sure not to divide by zero by slightly moving the vector in case of a via this methode uncomputanionable position
                if (-vector_1.y / vector_1.x * vector_2.x + vector_2.y).abs() < 0.01 {
                    offset = 0.05;
                }
                output.y = (-vector_2.z + vector_1.z / vector_1.x * vector_2.x) / ((-vector_1.y / vector_1.x * vector_2.x + vector_2.y)+ offset);
                output.x = (output.y * vector_1.y + vector_1.z) / -vector_1.x;
                output.normalize();
                println!("created orthagonal Vector via methode 1: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            } else if vector_1.y.abs() > 0.02 {
                // making sure not to divide by zero by slightly moving the vector in case of a via this methode uncomputanionable position
                if (-vector_1.x / vector_1.y * vector_2.y + vector_2.x).abs() < 0.01 {
                    offset = 0.05;
                }
                output.x = (-vector_2.z + vector_1.z / vector_1.y * vector_2.y) / ((-vector_1.x / vector_1.y * vector_2.y + vector_2.x)+ offset);
                output.y = (output.x * vector_1.x + vector_1.z) / -vector_1.y;
                output.normalize();
                println!("created orthagonal Vector via methode 2: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            } else if vector_1.z.abs() > 0.02 {
                // making sure not to divide by zero by slightly moving the vector in case of a via this methode uncomputanionable position
                if (-vector_1.y / vector_1.z * vector_2.z + vector_2.y).abs() < 0.01 {
                    offset = 0.05;
                }
                output.y = (-vector_2.x + vector_1.x / vector_1.z * vector_2.z) / ((-vector_1.y / vector_1.z * vector_2.z + vector_2.y) + offset);
                output.z = (output.y * vector_1.y + vector_1.x) / -vector_1.z;
                output.normalize();
                println!("created orthagonal Vector via methode 3: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            }
            if second_try == true{
                break;
            }
            second_try = true;
            println!("going for a second try to get orthagonal with switched vectors");
            vector_1 = other;
            vector_2 = self;
        }
        Err("unable to find definitiv orthagonal")
    
    }

    pub fn turn_into_screen(self, focus_point: Vector3D, aspect_ratio: f32, with: f32) -> Result<SquareSurface, String> {

        if self == focus_point {
            return Err("Failed to turn Point into SquareSurface since focus_point is located in self, meaning a directionless screen".to_string());

        } else {

            let focus_vector = focus_point - self;
            let mut relativ_vector = Vector3D::new(0.0, 0.0, 0.0);

            if focus_vector.y > 0.01 || focus_vector.y < -0.01 {
                relativ_vector = Vector3D::new(focus_vector.x, 0.0, focus_vector.z);
            } else {
                relativ_vector = Vector3D::new(0.0, -1.0, 0.0);
            }
            println!("going with relativ vector: {:?} because of focus vector: {:?}", relativ_vector, focus_vector);

            // creating a with  and hight Vector and normalizing them
            
            let screen_with_vector = relativ_vector.get_orthagonal(&focus_vector).unwrap_or_else(|_| focus_vector.get_orthagonal(&relativ_vector).unwrap()).normalize();
            let screen_hight_vector = focus_vector.get_orthagonal(&screen_with_vector).unwrap_or_else(|_| screen_with_vector.get_orthagonal(&focus_vector).unwrap()).normalize();

            println!("screen_with_vector: {:?}, screen_hight_vector: {:?}", screen_with_vector, screen_hight_vector);

            let screen_with = with;
            let screen_hight = screen_with / aspect_ratio;

            // calculating Lower left corner of screen (origin)

            let screen_origin = self - screen_hight_vector * (screen_hight / 2.0) - screen_with_vector * (screen_with / 2.0);

            let screen = SquareSurface::new(screen_origin, screen_with_vector, screen_hight_vector, screen_with, screen_hight, Color::new(0, 0, 0), &mut vec!());

            // thread::sleep(Duration::from_secs(600));

            return Ok(screen);
        }
    }

    pub fn get_tip_arrow_vectors(&self, arrow_length: f32, mut angle: f32) -> Result<(Vector3D, Vector3D), String>{
        let mut horizontal_orthagonal = Vector3D::new(0.0, 0.0, 0.0);
        angle /= 90.0;

        if self.y.abs() >= 0.01{
            horizontal_orthagonal = (*self * -1.0).get_orthagonal(&Vector3D::new(self.x, 0.0, self.z))?;
        } else {
            horizontal_orthagonal = (*self * -1.0).get_orthagonal(&Vector3D::new(0.0, 1.0, 0.0))?;
        }

        let vertical_orthagonal = (*self * -1.0).get_orthagonal(&horizontal_orthagonal)?;

        let mut norm_self = self.clone() * -1.0;
        let upper_arrow = (((vertical_orthagonal * 1.0).normalize()) * angle + (norm_self.normalize()) * (1.0 - angle)).normalize();
        let lower_arrow = (((vertical_orthagonal * -1.0).normalize()) * angle + (norm_self.normalize()) * (1.0 - angle)).normalize();

        Ok((upper_arrow * arrow_length, lower_arrow * arrow_length))
    }
}

impl Add for Vector3D{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        let return_vector =  Vector3D{
            x: self.x + other.x,
            z: self.z + other.z,
            y: self.y + other.y
        };
        return_vector
    }
}

impl Sub for Vector3D{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        let return_vector =  Vector3D{
            x: self.x - other.x,
            z: self.z - other.z,
            y: self.y - other.y
        };
        return_vector
    }
}

impl Mul<f32> for Vector3D{
    type Output = Self;

    fn mul(self, factor:f32) -> Self{
        let return_vector =  Vector3D{
            x: self.x * factor,
            z: self.z * factor,
            y: self.y * factor
        };
        return_vector
    }
}



#[derive(Copy, Clone)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color{

    pub fn new(red: u8, green: u8, blue: u8) -> Color{
        Color{
            red,
            blue,
            green
        }
    }

    pub fn black() -> Color{
        Color::new(0, 0, 0)
    }

    pub fn white() -> Color{
        Color::new(255, 255, 255)
    }
}

impl PartialEq for Color {

    fn eq(&self, other: &Self) -> bool {
        // Compare each field for equality
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Point{
    pub location: Vector3D,
    pub color: Color,
}

impl Point{

    pub fn new(location: Vector3D, color: Color,) -> Point{
        Point{
            location,
            color
        }
    }
}

#[derive(Debug)]
pub struct Line{
    starting_point: Vector3D,
    ending_point: Vector3D,
    color: Color
}

impl Line{

    pub fn new(starting_point: Vector3D, ending_point: Vector3D, color: Color) -> Line{
        Line{
            starting_point,
            ending_point,
            color
        }
    }

    pub fn turn_into_points(&self, point_collection:&mut Vec<Point>, point_interval_distance: u32){       
        let ray_direction: Vector3D = (self.ending_point - self.starting_point).normalize();
        let mut current_location: Vector3D = self.starting_point;

        while !relativ_change(self.starting_point.x, self.ending_point.x, current_location.x, self.ending_point.x) ||
              !relativ_change(self.starting_point.y, self.ending_point.y, current_location.y, self.ending_point.y) ||
              !relativ_change(self.starting_point.z, self.ending_point.z, current_location.z, self.ending_point.z)
        {
            println!("pushing point for line with direction: {:?} \nwith current location: {:?}", ray_direction, current_location);
            println!("starting point: {:?} \nending point: {:?} \n", self.starting_point, self.ending_point);
            point_collection.push(Point{ location:current_location, color: self.color});
            current_location = current_location + ray_direction * point_interval_distance as f32;
            // let duration = Duration::from_secs(1);
            // thread::sleep(duration);

        }
    }

    pub fn render_line<'a>(&'a self, screen: &'a SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, given_line_with: f32) -> Result<Vec<ScreenPoint<'a>>, String>{
        println!("\nstarting to render line");
        let mut output = vec!();

        let screen_plain = screen.get_plane();

        let starting_point_collision = screen_plain.find_vector_interception(&Point::new(self.starting_point, self.color), &mut (camera_position - self.starting_point));
        let starting_pixel_truple =  match screen.locate_point(starting_point_collision.clone(), (starting_point_collision.location - self.starting_point).occurence_length())? { 
            RayCollision::Collision(relativ_screen_position,  collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance),
            RayCollision::Miss(relativ_screen_position, collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance)};

        let ending_point_collision = screen_plain.find_vector_interception(&Point::new(self.ending_point, self.color), &mut (camera_position - self.ending_point));
        let ending_pixel_truple =  match screen.locate_point(ending_point_collision.clone(), (ending_point_collision.location - self.ending_point).occurence_length())? { 
            RayCollision::Collision(relativ_screen_position,  collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance),
            RayCollision::Miss(relativ_screen_position,  collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance)};

        let x_distance = ending_pixel_truple.0.x - starting_pixel_truple.0.x;
        let y_distance = ending_pixel_truple.0.y - starting_pixel_truple.0.y;
        let starting_diameter = ((scale(starting_pixel_truple.1) * 2) as f32 * given_line_with) as i64;
        let ending_diameter = ((scale(ending_pixel_truple.1) * 2) as f32 * given_line_with) as i64;
        let start_end_size_differens: i64 = ending_diameter - starting_diameter;

        println!("rendering line with starting size: {} \nand ending size: {}\n meaning a size differers of: {}", starting_diameter, ending_diameter, start_end_size_differens);
        // thread::sleep(Duration::from_secs(2));

        if x_distance.abs() == 0 && y_distance == 0 {

        }
        else if x_distance == 0 {
            for step in (0..(y_distance.abs() + 1)).collect::<Vec<i64>>().iter_mut(){
                *step = *step * y_distance.abs()/y_distance;
                let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*step as f32 / y_distance as f32) as f32).round() as f32;
                for layer in ((line_with as f32 / -2.0) + 0.1).round() as i64..((line_with as f32 / 2.0) - 0.9).round() as i64{
                    output.push(ScreenPoint{
                    parent: &screen,
                    x: starting_pixel_truple.0.x + layer,
                    y: starting_pixel_truple.0.y + *step,
                    color: self.color
                })}   
            }
        } else if y_distance == 0 {
            for step in (0..(x_distance.abs() + 1)).collect::<Vec<i64>>().iter_mut(){
                *step = *step * x_distance.abs()/x_distance;
                let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*step as f32 / x_distance as f32) as f32).round() as f32;
                for layer in ((line_with as f32 / -2.0) + 0.1).round() as i64..((line_with as f32 / 2.0) - 0.9).round() as i64{
                    output.push(ScreenPoint{
                    parent: &screen,
                    x: starting_pixel_truple.0.x + *step,
                    y: starting_pixel_truple.0.y + layer,
                    color: self.color,
                })}   
            }
        } else {

            let x_y_ratio = x_distance as f32 / y_distance as f32;
            let y_x_ratio = y_distance as f32 / x_distance as f32;

            if x_y_ratio.abs() > 1.0 {
                for relativ_x_pixel in (0..(x_distance.abs() + 1)).collect::<Vec<i64>>().iter_mut(){
                    *relativ_x_pixel = *relativ_x_pixel * x_distance.abs()/x_distance;
                    let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*relativ_x_pixel as f32 / x_distance as f32) as f32).round() as f32;
                    let relativ_y_pixel = ((y_x_ratio * relativ_x_pixel.abs() as f32).round() * (x_distance.abs()/x_distance) as f32) as i64;
                    for layer in ((line_with as f32 / -2.0) + 0.1).round() as i64..((line_with as f32 / 2.0) - 0.9).round() as i64{
                        output.push(ScreenPoint{
                            parent: &screen,
                            x: *relativ_x_pixel + starting_pixel_truple.0.x,
                            y: relativ_y_pixel + starting_pixel_truple.0.y + layer,
                            color: self.color
                        })
                    }
                }
            } else {
                for relativ_y_pixel in (0..(y_distance.abs() + 1)).collect::<Vec<i64>>().iter_mut(){
                    *relativ_y_pixel = *relativ_y_pixel * y_distance.abs()/y_distance;
                    let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*relativ_y_pixel as f32 / y_distance as f32) as f32).round() as f32;
                    let relativ_x_pixel = ((x_y_ratio * relativ_y_pixel.abs()  as f32).round() * (y_distance.abs()/y_distance) as f32) as i64;
                    for layer in ((line_with as f32 / -2.0) + 0.1).round() as i64..((line_with as f32 / 2.0) - 0.9).round() as i64{
                        output.push(ScreenPoint{
                            parent: &screen,
                            x: relativ_x_pixel + starting_pixel_truple.0.x + layer,
                            y: *relativ_y_pixel + starting_pixel_truple.0.y,
                            color: self.color
                        })
                    }
                }
            }
        }
        output = output.iter().filter(|pixel| (pixel.x > 0 && pixel.x <= x_resolution as i64) && (pixel.y > 0 && pixel.y <= y_resolution as i64)).map(|element| *element).collect();
        println!("finished rendering line\n");
        Ok(output)
    } 
}

fn relativ_change<T: PartialOrd>(initial_relation_1: T, initial_relation_2: T, current_relation_1: T,  current_relation_2:T) -> bool{
    if initial_relation_1 < initial_relation_2 {
        current_relation_1 > current_relation_2
    } else if initial_relation_1 == initial_relation_2{
        true
    }
    else {
        current_relation_1 < current_relation_2
    }
    
}

#[derive(Debug)]
pub struct Plane{
    x: f32,
    y: f32,
    z: f32,
    value: f32
}

impl Plane{

    pub fn find_vector_interception(&self, vector_origin:&Point, vector: &mut Vector3D) -> Point{
        vector.normalize();
        let collision_occurence_length = (self.value - vector_origin.location.x * self.x - vector_origin.location.y * self.y - vector_origin.location.z * self.z)
                                        / (vector.x * self.x + vector.y * self.y + vector.z * self.z);
        println!("finding Vector interseption");
        println!(" \nself: {:?} \nvector origin: {:?} \nvector : {:?} \ncalculation : \n({} - {} * {} - {} * {} - {} * {}) / ({} * {} + {} * {} + {} * {})", self, vector_origin, vector, self.value, vector_origin.location.x, self.x, vector_origin.location.y, self.y, vector_origin.location.z, self.z, vector.x, self.x,  vector.y, self.y, vector.z, self.z);
        println!("collision occurence_length: {} \n", collision_occurence_length);
        Point::new(vector_origin.location + *vector * collision_occurence_length, vector_origin.color)
    }
}

// origin of a SquareSurface is in the lower left corner
#[derive(Debug)]
#[derive(Clone)]
pub struct SquareSurface{
    origin: Vector3D,
    with_vector: Vector3D,
    hight_vector: Vector3D,
    with: f32,
    hight: f32,
    color: Color
}


impl SquareSurface{

    pub fn new(origin: Vector3D, with_vector: Vector3D, hight_vector: Vector3D, with: f32, hight: f32, color: Color, lines_collection: &mut Vec<Line>) -> SquareSurface{
        let return_square_surface = SquareSurface{
            origin,
            with_vector,
            hight_vector,
            with,
            hight,
            color
        };

        lines_collection.push(Line{starting_point: origin, ending_point:  origin + with_vector * with, color});
        lines_collection.push(Line{starting_point: origin, ending_point:  origin + hight_vector * hight, color});
        lines_collection.push(Line{starting_point: origin + with_vector * with, ending_point:  origin + with_vector * with + hight_vector * hight, color});
        lines_collection.push(Line{starting_point: origin + hight_vector * hight, ending_point:  origin + with_vector * with + hight_vector * hight, color});

        return_square_surface
    }

    pub fn get_plane(&self) -> Plane{
        let orthagonal_vector = self.with_vector.get_orthagonal(&self.hight_vector).unwrap().normalize();
        let output = Plane { 
            x: orthagonal_vector.x, 
            y: orthagonal_vector.y, 
            z: orthagonal_vector.z, 
            value: -1.0 * (orthagonal_vector.x * (- self.origin.x) + orthagonal_vector.y * (- self.origin.y) + orthagonal_vector.z * (- self.origin.z)) 
        };
        println!("created Plane: {:?}", output);
        output

    }

    pub fn locate_point(&self, point:Point, collision_distance:f32) -> Result<RayCollision, &str>{
        println!("\nlocating point:");
        let mut hight = 0.0;
        let mut with = 0.0;
        if self.with_vector.x > 0.01  || self.with_vector.x < -0.01{
            hight = ((point.location.x - self.origin.x)  / self.with_vector.x * self.with_vector.y + self.origin.y - point.location.y) / (-self.hight_vector.y + self.hight_vector.x / self.with_vector.x * self.with_vector.y);
            with = (point.location.x - self.origin.x - hight * self.hight_vector.x) / self.with_vector.x;
        }
        else if self.with_vector.y > 0.01  || self.with_vector.y < -0.01 {
            hight = ((point.location.y - self.origin.y)  / self.with_vector.y * self.with_vector.x + self.origin.x - point.location.x) / (-self.hight_vector.x + self.hight_vector.y / self.with_vector.y * self.with_vector.x);
            with = (point.location.y - self.origin.y - hight * self.hight_vector.y) / self.with_vector.y;
        }
        else if self.with_vector.z > 0.01  || self.with_vector.z < -0.01 {
            hight = ((point.location.z - self.origin.z)  / self.with_vector.z * self.with_vector.y + self.origin.y - point.location.y) / (-self.hight_vector.y + self.hight_vector.z / self.with_vector.z * self.with_vector.y);
            with = (point.location.z - self.origin.z - hight * self.hight_vector.z) / self.with_vector.z;
        } else {
            return Err("unaceptable with vector");
        }
        if (hight > self.hight || with > self.with) || (hight < 0.0 || with < 0.0) || collision_distance < 0.01{
            println!("Point: {:?} \nmissed at: x:{}, y:{}", point, with, hight);
            return Ok(RayCollision::Miss(RelativScreenPosition{
                parent: &self,
                relativ_with: with / self.with,
                relativ_hight: hight / self.hight,
                color: self.color}, CollisionDistance { distance: collision_distance }));
        } else {
            println!("Point: {:?} \nlocated at: x:{}, y:{}", point, with, hight);
            return Ok(RayCollision::Collision(RelativScreenPosition{
                parent: &self,
                relativ_with: with / self.with,
                relativ_hight: hight / self.hight,
                color: self.color
            }, CollisionDistance { distance: collision_distance }));
        }
    }

    // pub fn render_on_surface(&self, point_to_render: Point, camera_position: Vector3D) -> RayCollision{
    //     let ray = (camera_position - point_to_render.location).normalize();

    //     let ax = self.origin.x;
    //     let ay = self.origin.y;
    //     let az = self.origin.z;

    //     let ox = point_to_render.location.x;
    //     let oy = point_to_render.location.y;
    //     let oz = point_to_render.location.z;
        

    //     let rx = ray.x;
    //     let ry = ray.x;
    //     let rz = ray.x;

    //     // in order to avoid division through 0 teh the vectors are shifted a little bit

    //     let hx = self.hight_vector.x + 0.01;
    //     let hy = self.hight_vector.y + 0.01;
    //     let hz = self.hight_vector.z + 0.01;

    //     let wx = self.with_vector.x + 0.01;
    //     let wy = self.with_vector.y + 0.01;
    //     let wz = self.with_vector.z + 0.01;

    //     // This formula was derived in "Render script calculations.png" (its wrong again)
    //     // it took me 15 min to type this out correctly

    //     let ray_collisoin_occurence_length = (- oz + az + ((ox-ax-(oy*hx/(-wx * hy + wy * hx) - ay * hx/ (-wx * hy + wy * hx)- hy * (ox - ax) / (-wx * hy + wy * hx)) * wx) /hx ) *hz 
    //                                     + (oy *hx / (-wx * hy + wy * hx) - ay * hx / (-wx * hy + wy * hx) - hy * (ox - ax) / (-wx * hy + wy * hx)) * wz) 
    //                                     / (rz - (ry * hx / (-wx * hy + wy * hx) - hy * (rx) / (-wx * hy + wy * hx)) * wz - ((rx - (ry * hx / (-wx * hy + wy * hx) - hy * (rx) / (-wx * hy + wy * hx)) * wx) / hx) * hz);

    //     let square_surface_with = oy * hx / (-wx * hy + wy * hx) + ray_collisoin_occurence_length * ry * hx / (-wx * hy + wy * hx) - ay * hx / (-wx * hy + wy * hx) - hy * (ox - ray_collisoin_occurence_length * rx - ax) / (-wx * hy + wy * hx);

    //     let square_surface_hight = (ox + ray_collisoin_occurence_length * rx - ax - square_surface_with * wx) / hx;

    //     println!("Collison occurence_length: {}, \nSquareSurface with: {}, \nSquareSurface hight: {}, \n", ray_collisoin_occurence_length, square_surface_with, square_surface_hight); 

    //     if square_surface_hight <= self.hight && square_surface_hight >= 0.0 && 
    //        square_surface_with <= self.hight && square_surface_with >= 0.0 {

    //         let result = RayCollision::Collision(
    //             RelativScreenPosition{
    //             parent: &self,
    //             relativ_with: square_surface_with,
    //             relativ_hight: square_surface_hight,
    //             color: point_to_render.color
    //         }, CollisionDistance{distance: ray_collisoin_occurence_length});

    //         return result;
    //         }
           
    //     RayCollision::Miss
    // }

    
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct RelativScreenPosition<'a>{
    pub parent: &'a SquareSurface,
    pub relativ_with:f32,
    pub relativ_hight:f32,
    pub color: Color
}

impl<'a> RelativScreenPosition<'a>{

    pub fn turn_into_screen_point(&self, x_resolution: u32, y_resolution:  u32) -> ScreenPoint<'a>{
        println!("\ncreating screen point at relativ position x:{}, y:{} \nwith resolution x:{}, y:{} \nand pixel position x:{}, y:{}", self.relativ_with, self.relativ_hight, x_resolution, y_resolution, (self.relativ_with * x_resolution as f32).round(), ((1.0 - self.relativ_hight) * y_resolution as f32).round());
        ScreenPoint{
            parent: self.parent,
            x: (self.relativ_with * x_resolution as f32).round() as i64,
            y: ((1.0 - self.relativ_hight) * y_resolution as f32).round() as i64,  //because ScreenPoint hight is inverted SquareSurface hight
            color: self.color
        }
    }

    pub fn turn_into_stat_sized_screen_point(self, x_resolution: u32, y_resolution:  u32, radius: u32) -> Vec<ScreenPoint<'a>>{
        let mut output = vec!();
        let center = self.turn_into_screen_point(x_resolution, y_resolution);
        
        for i in 0..(radius + 1) {
            println!("((({} / ({} + 1)).asin()).cos() * {}).round() = {}", i, radius, radius, ((((i as f32 / (radius as f32 +1.0) as f32).asin() as f32).cos() as f32) * radius as f32).round() as i64);
            for unit in 0..((((i as f32 / (radius as f32 +1.0) as f32).asin() as f32).cos() as f32) * radius as f32).round() as i64 + 1{
                println!("{}", unit);
                for t in vec!(-1, 1) {
                    for j in vec!(-1, 1) {
                        if !((center.x as i64 + unit * t) < 0) && !((center.y as i64 + i as i64 * j) < 0){
                            output.push(ScreenPoint{
                                parent: self.parent,
                                x: (center.x as i64 + unit * t) as i64,
                                y: (center.y as i64 + i as i64 * j) as i64, //because ScreenPoint hight is inverted SquareSurface hight
                                color: self.color
                            });
                        }

                    }
                }
            }
        }
        output
    }

    pub fn turn_into_dyn_sized_screen_point(&self, x_resolution: u32, y_resolution: u32, distance: CollisionDistance) -> Vec<ScreenPoint<'a>>{
        let mut output = vec!();
        let center = self.turn_into_screen_point(x_resolution, y_resolution);
        println!("\nsizeing screen point at x:{}, y:{}", self.relativ_with, self.relativ_hight);

        let radius: u32 = scale(distance.distance);

        if distance.distance > 0.01 {

            output = self.turn_into_stat_sized_screen_point(x_resolution, y_resolution, radius);
        }
            
        println!("started with center: {:?} \nand distance: {:?}, \nand turned it into dots: {:?} \nwith radius: {}", center, distance, output, radius);
        output
    }
}

#[derive(Debug)]
pub struct CollisionDistance{
    pub distance:f32
}

#[derive(Debug)]
pub enum RayCollision<'a>{
    Collision(RelativScreenPosition<'a>, CollisionDistance),
    Miss(RelativScreenPosition<'a>, CollisionDistance),
}

// origin upper left corner (opposit to lower left corner origin of SquareSurface)
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct  ScreenPoint<'a>{
    pub parent: &'a SquareSurface,
    pub x: i64,
    pub y: i64,
    pub color: Color
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptimisedScreenPoint{
    color: Color,
    occurence_length: u32,
    x: u32
}

impl OptimisedScreenPoint {

    fn new(color: Color, occurence_length: u32, x :u32) -> OptimisedScreenPoint{
        OptimisedScreenPoint{
            color,
            occurence_length,
            x
        }
    }

    pub fn optimise_screen_point_collection<'a>(screen_point_collection: Vec<Vec<ScreenPoint<'a>>>, screen_height: i64, screen_width: i64, background_color: Color) -> Result<Vec<Vec<OptimisedScreenPoint>>, String>{
        let mut output = vec!();
        let mut current_point = ScreenPoint{
            parent: &SquareSurface::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0), 0.0, 0.0, Color::new(0, 0, 0), & mut vec!()),
            x: 1,
            y: 0,
            color: background_color
        };
        if screen_height < 1 || screen_width < 1 {
            return Err("optimisation of screen points failed because of an inapropiate screen height or width".to_string());
        }
        for screen_point_list in screen_point_collection.iter().filter(|list| !(list.iter().all(|element| (element.x > screen_width || element.y > screen_height) || element.y == 0))){
            // println!("currently optimising vec in row: {:?}", screen_point_list[0].y);
            if !screen_point_list.is_empty(){
                if !(screen_point_list[0].y > screen_height){
                    for empty_row_index in 0..(screen_point_list[0].y - current_point.y - 1){
                        // println!("current screen point y: {}, first list element y: {}, so we are filling {:?} rows", current_point.y, screen_point_list[0].y, 0..(screen_point_list[0].y - current_point.y - 1));
                        output.push(vec!(OptimisedScreenPoint::new(background_color, screen_width as u32, current_point.y as u32 + empty_row_index as u32)))
                    }
                }
                if output.is_empty() && screen_point_list[0].y != 1{
                    for empty_row_index in 0..(screen_point_list[0].y - current_point.y){
                        // println!("current screen point y: {}, first list element y: {}, so we are filling {:?} rows", current_point.y, screen_point_list[0].y, 0..(screen_point_list[0].y - current_point.y));
                        output.push(vec!(OptimisedScreenPoint::new(background_color, screen_width as u32, current_point.y as u32 + empty_row_index as u32)))
                    }
                }
                let mut optimized_line = vec!();
                let mut streak: u32 = 0;
                for (_index, screen_point) in screen_point_list.iter().enumerate().filter(|truple| {if truple.0 == 0 {true} else {!(truple.1.x == screen_point_list[truple.0 - 1].x)} }){
                    // println!("currently optimising point x: {}, in row: {:?}", screen_point.x, screen_point.y);
                    if (screen_point.x > screen_width || screen_point.y > screen_height) || (screen_point.x == current_point.x && screen_point.y == current_point.y) || screen_point.x == 0{
                        continue;
                    }
                    else {
                        if current_point.y != screen_point.y {
                            //optimized_line.push(OptimisedScreenPoint::new(background_color, (screen_point.x - 1) as u32));
                            current_point = *screen_point;
                        }
                        // else 
                        if current_point.color == screen_point.color && current_point.x + streak as i64 + 1 == screen_point.x{
                            streak += 1;
                            // continuing without assinging current_point new point
                            continue;
                        } else {
                            // println!("streak has ended :( \ncurrent color: {:?}, previous color: {:?} \ncurrent x: {:?}, previous x: {:?}", screen_point.color, current_point.color, screen_point.x, current_point.x);
                            optimized_line.push(OptimisedScreenPoint::new(current_point.color, 1 + streak, current_point.x as u32));
                            //optimized_line.push(OptimisedScreenPoint::new(background_color, (screen_point.x - (current_point.x + streak as i64 + 1)) as u32));
                            streak = 0;
                        }
                        current_point = *screen_point;
                    } 
                    
                }
                let mut optimized_line_occurence_length = 0;
                for element in optimized_line.iter(){
                    optimized_line_occurence_length += element.occurence_length;
                }
                if optimized_line.is_empty(){
                    output.push(vec!());
                } else if optimized_line_occurence_length < screen_width as u32{
                    optimized_line.push(OptimisedScreenPoint::new(current_point.color, 1 + streak, current_point.x as u32));
                    //optimized_line.push(OptimisedScreenPoint::new(background_color, (screen_width - (current_point.x + streak as i64 + 0)) as u32));
                    output.push(optimized_line);
                }
            }
        }
        if output.is_empty() || current_point.y != screen_height{
            for _empty_row in 0..(screen_height - current_point.y){
                output.push(vec!())
            }
        }
        Ok(output)
    }
}


pub fn sort_screen_points (mut input_list: Vec<ScreenPoint>) -> Vec<Vec<ScreenPoint>>{
    
    input_list.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut orderd_vec_list: Vec<Vec<ScreenPoint>> = vec!();
    // println!("{:?}", inner_list);

    for value in &input_list{
        if orderd_vec_list.len() > 0{
            if orderd_vec_list[orderd_vec_list.len() -1][0].y != value.y{
                orderd_vec_list.push(input_list.iter().map(|instance| *instance).filter(|instance| instance.y == value.y).collect());
            }
        } else {
            orderd_vec_list.push(input_list.iter().map(|instance| *instance).filter(|instance| instance.y == value.y).collect());
        }
        
    }

    // println!("{:?}", outer_list);

    for screen_point_list in orderd_vec_list.iter_mut(){

        screen_point_list.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }

    orderd_vec_list

}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullReqeustPackage {
    pub title: String,
    pub description: String,      //Karina says this would be beneficial
    pub resolution: (u64, u64),
    pub matrix: Vec<StringRotationMatrix>,
    pub theta: Vec<f32>,
    pub vectors: Vec<Vector3D>,
    pub layers: Vec<f32>,
    pub color_list: Vec<Vec<OptimisedScreenPoint>>,
    pub error: String
}

#[derive(Clone)]
#[derive(Debug, Deserialize, Serialize)]
pub struct StringRotationMatrix {
    pub line_x: [String; 3],
    pub line_y: [String; 3],
    pub line_z: [String; 3]
}


pub struct RotationMatrix {
    line_x: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3],
    line_y: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3],
    line_z: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3],
}
 
fn remove_brackets_content(input_text: String) -> String {
    println!("parsing: {}", input_text);
    let mut  in_brackets: bool = false;
    let mut output = vec!();

    for char in input_text.chars(){
        if in_brackets {
            if char == ')' {
                in_brackets = false;
                output.push(')');
            }
        } else {
            if char == '(' {
                in_brackets = true;
                output.push('(');
            } else {
                output.push(char); 
            }
        }
    }

    output.iter().collect()
}

impl StringRotationMatrix{

    pub fn turn_into_rotation_matrix<'a>(self) -> Result<RotationMatrix, String>{
        let mut parsed_line_x: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3] = [Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x})))];
        let  math_operations:HashMap<&str, Arc<Mutex<dyn Fn(f32, f32) -> f32>>>  = build_math_hashmap();
        for (index, element) in self.line_x.iter().enumerate(){
            let mut cleaned_key: String = element
                                        .to_lowercase()
                                        .chars()
                                        .filter(|charecter| *charecter != ' ')
                                        .collect();
                                        cleaned_key = remove_brackets_content(cleaned_key);
            if math_operations.contains_key(cleaned_key.as_str()){
                parsed_line_x[index] = Arc::clone(&math_operations.get(cleaned_key.as_str()).unwrap());
            }
            else {
                match cleaned_key.parse::<f32>() {
                    Ok(value) => parsed_line_x[index] = Arc::new(Mutex::new((move |_x: f32, _theta:f32| value))),
                    Err(_) => return Err(format!("unable to translate &str into operation, key: {} is not contained in hashmap or parsable", cleaned_key.as_str()))
            } 
            }
        }
        let mut parsed_line_y: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3] = [Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x})))];
        for (index, element) in self.line_y.iter().enumerate(){
            let mut cleaned_key: String = element
                                        .to_lowercase()
                                        .chars()
                                        .filter(|charecter| *charecter != ' ')
                                        .collect();
                                        cleaned_key = remove_brackets_content(cleaned_key);
            if math_operations.contains_key(cleaned_key.as_str()){
                parsed_line_y[index] = Arc::clone(&math_operations.get(cleaned_key.as_str()).unwrap());
            }
            else {
                match cleaned_key.parse::<f32>() {
                    Ok(value) => parsed_line_y[index] = Arc::new(Mutex::new((move |_x: f32, _theta:f32| value))),
                    Err(_) => return Err(format!("unable to translate &str into operation, key: {} is not contained in hashmap or parsable", cleaned_key.as_str()))
            } 
            }
        }
        let mut parsed_line_z: [Arc<Mutex<dyn Fn(f32, f32) -> f32>>; 3] = [Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x}))), Arc::new(Mutex::new((|x, _theta| {0.0 * x})))];
        for (index, element) in self.line_z.iter().enumerate(){
            let mut cleaned_key: String = element
                                        .to_lowercase()
                                        .chars()
                                        .filter(|charecter| *charecter != ' ')
                                        .collect();
                                        cleaned_key = remove_brackets_content(cleaned_key);
            if math_operations.contains_key(cleaned_key.as_str()){
                parsed_line_z[index] = Arc::clone(&math_operations.get(cleaned_key.as_str()).unwrap());
            }
            else {
                match cleaned_key.parse::<f32>() {
                    Ok(value) => parsed_line_z[index] = Arc::new(Mutex::new((move |_x: f32, _theta:f32| value))),
                    Err(_) => return Err(format!("unable to translate &str into operation, key: {} is not contained in hashmap or parsable", cleaned_key.as_str()))
            } 
            }
        }
        Ok(RotationMatrix{
            line_x: parsed_line_x,
            line_y: parsed_line_y,
            line_z: parsed_line_z
        })
    }

}

impl RotationMatrix{ 

    pub fn multiply(&self, factor: Vector3D, theta: &f32) -> Vector3D{
        let mut output = Vector3D::new(0.0, 0.0, 0.0);
        let theta_occourence = theta * (PI / 180.0) as f32;
        let factor_truple = [factor.x, factor.y, factor.z];

        for (index, value) in factor_truple.iter().enumerate(){
            output.x += self.line_x[index].lock().unwrap()(*value, theta_occourence);
            output.y += self.line_y[index].lock().unwrap()(*value, theta_occourence);
            output.z += self.line_z[index].lock().unwrap()(*value, theta_occourence);
        }
        output
    }
}

pub fn scale(collision_distance: f32) -> u32{
    let max_size: f32 = 6.0;
    let min_size: f32 = 2.0;

    if collision_distance <= 0.0 {
        return max_size as u32
    }

    let average_expected_distance = 7.0;

    let mut size = average_expected_distance / collision_distance * min_size;

    if size > max_size {
        size = max_size
    }

    if size < min_size {
        size = min_size
    }

    size.round() as u32
}
    


pub fn build_math_hashmap() -> HashMap<&'static str, Arc<Mutex<dyn Fn(f32, f32) -> f32>>> {
    let mut math_operations: HashMap<&'static str, Arc<Mutex<dyn Fn(f32, f32) -> f32>>> = HashMap::new();

    // Define closures for sin, cos, tan, asin, acos, atan
    math_operations.insert("sin", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.sin())));
    math_operations.insert("cos", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.cos())));
    math_operations.insert("tan", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.tan())));
    math_operations.insert("asin", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.asin())));
    math_operations.insert("acos", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.acos())));
    math_operations.insert("atan", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.atan())));

    // Additional representations for each operation
    math_operations.insert("sine", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.sin())));
    math_operations.insert("cosine", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.cos())));
    math_operations.insert("tangent", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.tan())));
    math_operations.insert("arcsin", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.asin())));
    math_operations.insert("arccos", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.acos())));
    math_operations.insert("arctan", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.atan())));

    // Representations with parentheses for each operation
    math_operations.insert("sin()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.sin())));
    math_operations.insert("cos()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.cos())));
    math_operations.insert("tan()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.tan())));
    math_operations.insert("asin()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.asin())));
    math_operations.insert("acos()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.acos())));
    math_operations.insert("atan()", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.atan())));

    // German spellings for each operation
    math_operations.insert("sinus", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.sin())));
    math_operations.insert("kosinus", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.cos())));
    math_operations.insert("tangens", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.tan())));
    math_operations.insert("arcsinus", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.asin())));
    math_operations.insert("arccosinus", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.acos())));
    math_operations.insert("arctangens", Arc::new(Mutex::new(|x: f32, theta:f32| x * theta.atan())));

    // Define closures for sin, cos, tan, asin, acos, atan
    math_operations.insert("-sin", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.sin()))));
    math_operations.insert("-cos", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.cos()))));
    math_operations.insert("-tan", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.tan()))));
    math_operations.insert("-asin", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.asin()))));
    math_operations.insert("-acos", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.acos()))));
    math_operations.insert("-atan", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.atan()))));

    // Additional representations for each operation
    math_operations.insert("-sine", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.sin()))));
    math_operations.insert("-cosine", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.cos()))));
    math_operations.insert("-tangent", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.tan()))));
    math_operations.insert("-arcsin", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.asin()))));
    math_operations.insert("-arccos", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.acos()))));
    math_operations.insert("-arctan", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.atan()))));

    // Representations with parentheses for each operation
    math_operations.insert("-sin()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.sin()))));
    math_operations.insert("-cos()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.cos()))));
    math_operations.insert("-tan()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.tan()))));
    math_operations.insert("-asin()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.asin()))));
    math_operations.insert("-acos()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.acos()))));
    math_operations.insert("-atan()", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.atan()))));

    // German spellings for each operation
    math_operations.insert("-sinus", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.sin()))));
    math_operations.insert("-kosinus", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.cos()))));
    math_operations.insert("-tangens", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.tan()))));
    math_operations.insert("-arcsinus", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.asin()))));
    math_operations.insert("-arccosinus", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.acos()))));
    math_operations.insert("-arctangens", Arc::new(Mutex::new(|x: f32, theta:f32| -(x * theta.atan()))));

    // Adding Numbers
    math_operations.insert("0", Arc::new(Mutex::new(|_x: f32, _theta:f32| 0.0)));
    math_operations.insert("1", Arc::new(Mutex::new(|x: f32, _theta:f32| x)));
    math_operations.insert("-1", Arc::new(Mutex::new(|x: f32, _theta:f32| -x)));
    math_operations.insert("0.0", Arc::new(Mutex::new(|_x: f32, _theta:f32| 0.0)));
    math_operations.insert("1.0", Arc::new(Mutex::new(|x: f32, _theta:f32| x)));
    math_operations.insert("-1.0", Arc::new(Mutex::new(|x: f32, _theta:f32| -x)));

    math_operations
}