use core::panic;
// use core::error;
use std::cell::RefCell;
use std::ops::{Add, Mul, Sub};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;
use actix_web::web::get;
use askama::filters::upper;
use futures::stream::Enumerate;
use futures::task::noop_waker_ref;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::error;
use std::{thread, vec};
use std::time::Duration;
use std::collections::HashSet;
use std::default;
use std::mem::size_of_val;
use bytemuck;
use wgpu::util::DeviceExt;
use flume;
use tokio;



#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
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

    pub fn dot_product(&self, other: Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn get_orthagonal(&self, other: &Vector3D) -> Result<Vector3D, &str>{
        // JUST USE CROSS LMAO
        // println!("getting orthagonal Vector");
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
                // println!("created orthagonal Vector via methode 1: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            } else if vector_1.y.abs() > 0.02 {
                // making sure not to divide by zero by slightly moving the vector in case of a via this methode uncomputanionable position
                if (-vector_1.x / vector_1.y * vector_2.y + vector_2.x).abs() < 0.01 {
                    offset = 0.05;
                }
                output.x = (-vector_2.z + vector_1.z / vector_1.y * vector_2.y) / ((-vector_1.x / vector_1.y * vector_2.y + vector_2.x)+ offset);
                output.y = (output.x * vector_1.x + vector_1.z) / -vector_1.y;
                output.normalize();
                // println!("created orthagonal Vector via methode 2: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            } else if vector_1.z.abs() > 0.02 {
                // making sure not to divide by zero by slightly moving the vector in case of a via this methode uncomputanionable position
                if (-vector_1.y / vector_1.z * vector_2.z + vector_2.y).abs() < 0.01 {
                    offset = 0.05;
                }
                output.y = (-vector_2.x + vector_1.x / vector_1.z * vector_2.z) / ((-vector_1.y / vector_1.z * vector_2.z + vector_2.y) + offset);
                output.z = (output.y * vector_1.y + vector_1.x) / -vector_1.z;
                output.normalize();
                // println!("created orthagonal Vector via methode 3: {:?} with vector_1: {:?}, and vector_2: {:?}", output, vector_1, vector_2);
                return Ok(output);
            }
            if second_try == true{
                break;
            }
            second_try = true;
            // println!("going for a second try to get orthagonal with switched vectors");
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
            // println!("going with relativ vector: {:?} because of focus vector: {:?}", relativ_vector, focus_vector);

            // creating a with  and hight Vector and normalizing them
            
            let screen_with_vector = relativ_vector.get_orthagonal(&focus_vector).unwrap_or_else(|_| focus_vector.get_orthagonal(&relativ_vector).unwrap()).normalize();
            let screen_hight_vector = focus_vector.get_orthagonal(&screen_with_vector).unwrap_or_else(|_| screen_with_vector.get_orthagonal(&focus_vector).unwrap()).normalize();

            // println!("screen_with_vector: {:?}, screen_hight_vector: {:?}", screen_with_vector, screen_hight_vector);

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

    // All values in radians
    pub fn find_angle(&self, other: Vector3D) -> f32 {
        let mut result = (self.dot_product(other) / (self.occurence_length() * other.occurence_length())).acos();
        if result > 3.141 / 2.0 {
            result = result - 3.141 / 2.0;
        }
        result
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



#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vector2D {
    pub x:f32,
    pub y:f32
}

impl Vector2D{
    
    pub fn new(x:f32, y:f32) -> Vector2D{
        Vector2D{
            x,
            y
        }
    }

    pub fn origin() -> Vector2D{
        Vector2D{
            x: 0.0,
            y: 0.0
        }
    }

    pub fn occurence_length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) -> Vector2D{
        let occurence_length = self.occurence_length();
        
        if occurence_length != 0.0 {
            self.x /= occurence_length;
            self.y /= occurence_length;
        }
        *self
    }

    pub fn dot_product(&self, other: Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn combinate(&self, other: &Vector2D, target: Vector2D) -> Result<(f32, f32), String> {

        // use cross product
        if self.clone().normalize() * (self.occurence_length() / self.occurence_length().abs()) == other.clone().normalize() * (other.occurence_length() / other.occurence_length().abs()) {
            return Err("The 2 vectors provided were not independent".to_string());
        }

        let matrix = (Vector2D::new(self.x, other.x), 
                                            Vector2D::new(self.y, other.y));

        let determinant = matrix.0.x * matrix.1.y - matrix.1.x * matrix.1.y;
        let inverse_matrix = (Vector2D::new(other.y /determinant, -other.x /determinant), 
                                                    Vector2D::new(-self.y /determinant, self.x /determinant));

        let factor_x = inverse_matrix.0.x * target.x + inverse_matrix.0.y * target.y;
        let factor_y = inverse_matrix.1.x * target.x + inverse_matrix.1.y * target.y;

        Ok((factor_x, factor_y))
    }
}

impl Add for Vector2D{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        let return_vector =  Vector2D{
            x: self.x + other.x,
            y: self.y + other.y
        };
        return_vector
    }
}

impl Sub for Vector2D{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        let return_vector =  Vector2D{
            x: self.x - other.x,
            y: self.y - other.y
        };
        return_vector
    }
}

impl Mul<f32> for Vector2D{
    type Output = Self;

    fn mul(self, factor:f32) -> Self{
        let return_vector =  Vector2D{
            x: self.x * factor,
            y: self.y * factor
        };
        return_vector
    }
}



#[derive(Copy, Clone)]
#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    red: u32,
    green: u32,
    blue: u32
}

impl Color{

    pub fn new(red: u32, green: u32, blue: u32) -> Color{
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

impl Mul<f32> for Color{
    type Output = Self;

    fn mul(self, factor:f32) -> Self{
        let return_vector =  Color{
            red: (self.red as f32 * factor).clamp(0.0, 255.0) as u32,
            green: (self.green as f32 * factor).clamp(0.0, 255.0) as u32,
            blue: (self.blue as f32 * factor).clamp(0.0, 255.0) as u32
        };
        return_vector
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

    pub fn draw<'a> (&'a self, camera_position: Vector3D, screen: &'a SquareSurface, x_resolution: u32, y_resolution: u32) -> Result<(ScreenPoint, f32), String> {
        let point_collision = screen.get_plane().find_vector_interception(self, &mut (camera_position - self.location)).unwrap();
        let point_collision_location = point_collision.location;
        let pixel_truple =  match screen.locate_point(point_collision, (point_collision_location - self.location).occurence_length())? { 
            RayCollision::Collision(relativ_screen_position,  collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance),
            RayCollision::Miss(relativ_screen_position, collision_distance) => (relativ_screen_position.turn_into_screen_point(x_resolution, y_resolution), collision_distance.distance)};
        Ok(pixel_truple)
    }
}

#[derive(Debug)]
#[derive(Clone)]
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
        println!("turning line into points");       
        let ray_direction: Vector3D = (self.ending_point - self.starting_point).normalize();
        let mut current_location: Vector3D = self.starting_point;

        while !relativ_change(self.starting_point.x, self.ending_point.x, current_location.x, self.ending_point.x) ||
              !relativ_change(self.starting_point.y, self.ending_point.y, current_location.y, self.ending_point.y) ||
              !relativ_change(self.starting_point.z, self.ending_point.z, current_location.z, self.ending_point.z)
        {
            // println!("pushing point for line with direction: {:?} \nwith current location: {:?}", ray_direction, current_location);
            // println!("starting point: {:?} \nending point: {:?} \n", self.starting_point, self.ending_point);
            point_collection.push(Point{ location:current_location, color: self.color});
            current_location = current_location + ray_direction * point_interval_distance as f32;
            // let duration = Duration::from_secs(1);
            // thread::sleep(duration);

        }
    }

    pub fn render_line<'a>(self, screen: &'a SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, given_line_with: f32) -> Result<Vec<ScreenPoint<'a>>, String>{
        
        // println!("starting to render line");
        let mut output = vec!();

        let starting_point = Point::new(self.starting_point, self.color);
        let starting_pixel_truple = starting_point.draw(camera_position, screen, x_resolution, y_resolution)?;

        let ending_point = Point::new(self.ending_point, self.color);
        let ending_pixel_truple = ending_point.draw(camera_position, screen, x_resolution, y_resolution)?;

        let x_distance = ending_pixel_truple.0.x - starting_pixel_truple.0.x;
        let y_distance = ending_pixel_truple.0.y - starting_pixel_truple.0.y;

        // let starting_diameter = ((scale(starting_pixel_truple.1) * 2) as f32 * given_line_with) as i64;
        // let ending_diameter = ((scale(ending_pixel_truple.1) * 2) as f32 * given_line_with) as i64;

        // let starting_diameter = 1 as i64;
        // let ending_diameter = 1 as i64;

        // let start_end_size_differens: i64 = 4;

        // println!("rendering line with starting size: {} \nand ending size: {}\n meaning a size differers of: {}", starting_diameter, ending_diameter, start_end_size_differens);
        // thread::sleep(Duration::from_secs(2));

        if x_distance.abs() == 0 && y_distance == 0 {

        }
        else if x_distance == 0 {
            for step in (0..(y_distance.abs() + 1)).collect::<Vec<i64>>().iter_mut(){
                *step = *step * y_distance.abs()/y_distance;
                let line_with = 2;
                // let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*step as f32 / y_distance as f32) as f32).round() as f32;
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
                let line_with = 2;
                // let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*step as f32 / x_distance as f32) as f32).round() as f32;
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
                    let line_with = 2;
                    // let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*relativ_x_pixel as f32 / x_distance as f32) as f32).round() as f32;
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
                    let line_with = 2;
                    // let line_with = starting_diameter as f32 + (start_end_size_differens as f32 * (*relativ_y_pixel as f32 / y_distance as f32) as f32).round() as f32;
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
        // println!("finished rendering line\n");
        Ok(output)
    } 

    pub fn find_intercept(&self, other_line: &Line) -> Option<Vector3D>{

        // checking if both are in same plane fist 
        let self_direction = self.ending_point - self.starting_point;
        let other_direction = self.ending_point - self.starting_point;

        let self_direction_norm = self_direction.clone().normalize();
        let other_direction_norm = other_direction.clone().normalize();

        if  self_direction_norm == other_direction_norm ||  self_direction_norm * -1.0 == other_direction_norm {
            return None;
        }

        let ortahgonal_vector = self_direction.get_orthagonal(&other_direction).unwrap();
        let encompasing_plane = Plane::new(ortahgonal_vector.x, ortahgonal_vector.y, ortahgonal_vector.z, ortahgonal_vector.x * self.starting_point.x + ortahgonal_vector.y * self.starting_point.y + ortahgonal_vector.z * self.starting_point.z).unwrap();

        if !encompasing_plane.check_point(other_line.starting_point) || !encompasing_plane.check_point(other_line.ending_point) {
            return None;
        }


        let mut orthangonal_Plane = Plane::new(self_direction.x, self_direction.y, self_direction.z, self_direction.x * self.starting_point.x + self_direction.y* self.starting_point.y + self_direction.z * self.starting_point.z).unwrap();
        let other_line_point_start = orthangonal_Plane.find_vector_interception(&Point::new(other_line.starting_point, Color::white()), &mut (other_line.ending_point - other_line.starting_point))?;
        orthangonal_Plane = Plane::new(self_direction.x, self_direction.y, self_direction.z, self_direction.x * self.ending_point.x + self_direction.y* self.ending_point.y + self_direction.z * self.ending_point.z).unwrap();
        let other_line_point_end = orthangonal_Plane.find_vector_interception(&Point::new(other_line.ending_point, Color::white()), &mut (other_line.ending_point - other_line.starting_point))?;

        let distance_start = (other_line_point_start.location - self.starting_point).occurence_length();
        let distance_end = (other_line_point_end.location - self.ending_point).occurence_length();

        let distance_shift = (distance_end - distance_start);

        let way_to_go = distance_start / distance_shift;

        // if way_to_go.abs() > 1.0 {
        //    return None;
        // }
        
        let collision = self.starting_point + self_direction * way_to_go;

        Some(collision)
    }

    pub fn find_intercept_plane(&self, plane: &Plane) -> Option<Vector3D>{

        let mut line_vector = self.starting_point - self.ending_point;
        let collsion_point = plane.find_vector_interception(&Point::new(self.ending_point, Color::white()), &mut line_vector).unwrap().location; // could fuck me in theorie when plane runs parralel to vector
        let connection_vector = self.starting_point - collsion_point;
        if !(connection_vector.occurence_length() > connection_vector.occurence_length() ||
           connection_vector.dot_product(Vector3D::new(1.0, 1.0, 1.0)) / connection_vector.dot_product(Vector3D::new(1.0, 1.0, 1.0)) < 0.0) {
            Some(collsion_point)
        } else {
            None
        }
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
#[derive(Clone)]
pub struct Plane{
    x: f32,
    y: f32,
    z: f32,
    value: f32
}

impl Plane{

    pub fn new (x: f32, y: f32, z: f32, value: f32) -> Result<Plane, String> {
        if !(x == 0.0 && y == 0.0 && z == 0.0) {
            return Ok(Plane {
                x,
                y,
                z,
                value,
            })
        }
        Err("you tried to make a 0,0,0 Plane. wtf?".to_string())
    }

    pub fn new_from_points(location_1: Vector3D, location_2:Vector3D, location_3:Vector3D) -> Result<Plane, String> {
        if location_1 == location_2 || location_1 == location_3 || location_2 == location_3 {
            return Err("Two or more locations were duplicates, Plane not definitvly determinable".to_string());
        }
        let vector_1 = location_1 - location_2;
        let vector_2 = location_1 - location_3;

        let normal_vector = vector_1.get_orthagonal(&vector_2).unwrap();
        let plane_value = normal_vector.dot_product(location_1);
        let plane = Plane::new(normal_vector.x, normal_vector.y, normal_vector.z, plane_value).unwrap();

        Ok(plane)
    }

    pub fn get_any_point (&self) -> Vector3D {
        if self.value == 0.0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }
        if (self.x != 0.0) {
            return Vector3D::new(self.value / self.x, 0.0, 0.0);
        } else if (self.y != 0.0) {
            return Vector3D::new(0.0, self.value / self.y, 0.0);
        } else {
            return Vector3D::new(0.0, 0.0, self.value / self.z);
        }
    }

    pub fn check_point(&self, point: Vector3D) -> bool {
        let normal_vector = Vector3D::new(self.x, self.y, self.z);
        let point_value = point.dot_product(normal_vector);
        if (point_value * 1000.0).round() == (self.value * 1000.0).round() {
            return true;
        }
        println!("point: {:?} does not lie in plane with normal vector: {:?} \ndot product: {:?} not equal to value: {:?}", point, normal_vector, point_value, self.value);
        false
    }

    pub fn find_vector_interception(&self, vector_origin:&Point, vector: &mut Vector3D) -> Option<Point>{
        vector.normalize();
        let dot_product = vector.x * self.x + vector.y * self.y + vector.z * self.z;
        if dot_product == 0.0 {
            return None;
        }
        let collision_occurence_length = (self.value - vector_origin.location.x * self.x - vector_origin.location.y * self.y - vector_origin.location.z * self.z)
                                        / (vector.x * self.x + vector.y * self.y + vector.z * self.z);
        // println!("finding Vector interseption");
        // println!(" \nself: {:?} \nvector origin: {:?} \nvector : {:?} \ncalculation : \n({} - {} * {} - {} * {} - {} * {}) / ({} * {} + {} * {} + {} * {})", self, vector_origin, vector, self.value, vector_origin.location.x, self.x, vector_origin.location.y, self.y, vector_origin.location.z, self.z, vector.x, self.x,  vector.y, self.y, vector.z, self.z);
        // println!("collision occurence_length: {} \n", collision_occurence_length);
        Some(Point::new(vector_origin.location + *vector * collision_occurence_length, vector_origin.color))
    }

    pub fn find_plane_interception(&self, plane: Plane) -> Line {
        let normal_vector = Vector3D::new(self.x, self.y, self.z);
        let additional_vector = Vector3D::new(self.x, self.y, self.z + 0.1);
        let any_plane_point = Point::new(self.get_any_point(), Color::white());

        let mut plane_vector_1 = normal_vector.get_orthagonal(&additional_vector).unwrap();
        let mut plane_vector_2 = normal_vector.get_orthagonal(&plane_vector_1).unwrap();
        let mut plane_vector_3 = plane_vector_1 + plane_vector_2;

        let intercept_1 = self.find_vector_interception(&any_plane_point, &mut plane_vector_1);
        let intercept_2 = self.find_vector_interception(&any_plane_point, &mut plane_vector_2);
        let intercept_3 = self.find_vector_interception(&any_plane_point, &mut plane_vector_3);

        match intercept_1 {
            None => {return Line::new(intercept_2.unwrap().location, intercept_3.unwrap().location, Color::black())},
            _ => {}
        }
        match intercept_2 {
            None => {return Line::new(intercept_1.unwrap().location, intercept_3.unwrap().location, Color::black())},
            _ => {}
        }

        Line::new(intercept_1.unwrap().location, intercept_2.unwrap().location, Color::black())
    }
}

// origin of a SquareSurface is in the lower left corner
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
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
        // println!("created Plane: {:?}", output);
        output

    }

    pub fn locate_point(&self, point:Point, collision_distance:f32) -> Result<RayCollision, &str>{
        // println!("\nlocating point:");
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
            // println!("Point: {:?} \nmissed at: x:{}, y:{}", point, with, hight);
            return Ok(RayCollision::Miss(RelativScreenPosition{
                parent: &self,
                relativ_with: with / self.with,
                relativ_hight: hight / self.hight,
                color: self.color}, CollisionDistance { distance: collision_distance }));
        } else {
            // println!("Point: {:?} \nlocated at: x:{}, y:{}", point, with, hight);
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
        // println!("\ncreating screen point at relativ position x:{}, y:{} \nwith resolution x:{}, y:{} \nand pixel position x:{}, y:{}", self.relativ_with, self.relativ_hight, x_resolution, y_resolution, (self.relativ_with * x_resolution as f32).round(), ((1.0 - self.relativ_hight) * y_resolution as f32).round());
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
            // println!("((({} / ({} + 1)).asin()).cos() * {}).round() = {}", i, radius, radius, ((((i as f32 / (radius as f32 +1.0) as f32).asin() as f32).cos() as f32) * radius as f32).round() as i64);
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
        // let center = self.turn_into_screen_point(x_resolution, y_resolution);
        // println!("\nsizeing screen point at x:{}, y:{}", self.relativ_with, self.relativ_hight);

        let radius: u32 = scale(distance.distance);

        if distance.distance > 0.01 {

            output = self.turn_into_stat_sized_screen_point(x_resolution, y_resolution, radius);
        }
            
        // println!("started with center: {:?} \nand distance: {:?}, \nand turned it into dots: {:?} \nwith radius: {}", center, distance, output, radius);
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
#[derive(PartialEq)]
pub struct  ScreenPoint<'a>{
    pub parent: &'a SquareSurface,
    pub x: i64,
    pub y: i64,
    pub color: Color
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(PartialEq)]
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

    pub fn optimise_screen_point_collection<'a>(screen_point_collection: Vec<ScreenPoint<'a>>, screen_height: i64, screen_width: i64) -> Result<Vec<Vec<OptimisedScreenPoint>>, String>{
        let sorted_screen_point_collection = sort_screen_points(screen_point_collection);
        let filtered_screen_points = sorted_screen_point_collection.iter().filter_map(|list| if (list.iter().all(|element| (element.x < screen_width && element.y < screen_height) && element.y > 0)) {Some(list)} else {None}).collect::<Vec<&Vec<ScreenPoint>>>();
        let mut output = vec!();
        
        if screen_height < 1 || screen_width < 1 {
            return Err("optimisation of screen points failed because of an inapropiate screen height or width".to_string());
        }

        let mut recent_y = 0;
        let mut recent_x = None;
        let mut recent_color = None;
        for sp_list in filtered_screen_points.iter(){

            let mut current_y = match sp_list.first() {
                Some(value) => {value.y},
                _ => {continue;}
            };

            for _index in recent_y..(current_y - 1) {
                output.push(vec!());
            }

            let mut streak = 1;
            let mut current_line = vec!();

            for (index, pixel) in sp_list.iter().enumerate() {

                current_y = pixel.y;
                let current_x = pixel.x;
                let current_color = pixel.color;

                if index == 0 || current_y != recent_y {
                    recent_y = current_y;
                    recent_x = Some(current_x);
                    recent_color = Some(current_color);
                    continue;
                }

                if current_x == recent_x.unwrap() + streak &&
                   current_color == recent_color.unwrap() {
                    streak = streak + 1;
                    continue;
                }

                let recent_opt_pixel = OptimisedScreenPoint::new(recent_color.unwrap(), streak as u32, recent_x.unwrap() as u32);
                streak = 1;
                
                current_line.push(recent_opt_pixel);

                recent_y = current_y;
                recent_x = Some(current_x);
                recent_color = Some(current_color);
            }

            match recent_x {
                Some(x) => {current_line.push(OptimisedScreenPoint::new(recent_color.unwrap(), streak as u32, x as u32))},
                _ => {}
            }
            output.push(current_line);
        }

        for _index in recent_y..screen_height {
            output.push(vec!());
        }

        Ok(output)
    }

    pub fn layer(mut lower_layer: Vec<Vec<OptimisedScreenPoint>>, mut upper_layer: Vec<Vec<OptimisedScreenPoint>>) -> Result<Vec<Vec<OptimisedScreenPoint>>, Box<dyn std::error::Error>> {
        // println!("\n\nlayering lower_layer: \n{:?}\nand upper layer: \n{:?}\n\n", lower_layer, upper_layer);
        let mut combined_layer = vec!();
        if lower_layer.len() != upper_layer.len() {
            // println!("\nThe 2 layers are of uneqaul resolution, lower layer: {}, upper layer: {}", lower_layer.len(), upper_layer.len());

            let mut discrepency = upper_layer.len() - lower_layer.len();
            while discrepency > 0 {
                lower_layer.push(vec!());
                discrepency = discrepency - 1;
            }
            while discrepency < 0 {
                upper_layer.push(vec!());
                discrepency = discrepency + 1;
            }
        }

        // TO DO: write as while loop so repetition is possible because issues occure if 2 upper pixels are on 1 lower pixel

        for (index, line) in lower_layer.iter_mut().enumerate() {
            let upper_line = upper_layer.get_mut(index).unwrap();
            upper_line.sort_by(|a, b| a.x.cmp(&b.x));
            line.sort_by(|a, b| a.x.cmp(&b.x));
            if upper_line.len() == 0 {
                combined_layer.push(line.clone());
                continue;
            } else if line.len() == 0 {
                combined_layer.push(upper_line.clone());
                continue;
            } else {
                let mut combined_line = vec!();

                let mut lower_index = 0;

                while lower_index < line.len() {

                    let lower_area = line.get_mut(lower_index).unwrap();

                    let upper_area = match upper_line.first() {
                        Some(x) => {x},
                        _ => {combined_line.append(&mut line[lower_index..(line.len())].to_owned()); break;}
                    };

                    let lower_area_reach = lower_area.x + lower_area.occurence_length - 1;
                    let upper_area_reach = upper_area.x + upper_area.occurence_length - 1;

                    if upper_area_reach < lower_area.x {
                        // upper area before lower area

                        combined_line.push(upper_area.clone());
                        upper_line.remove(0);

                        continue;
                    }

                    if lower_area_reach < upper_area.x {
                        // lower area before upper area

                        combined_line.push(lower_area.clone());
                    }

                    else {
                        if lower_area_reach <= upper_area_reach {
                            if lower_area.x >= upper_area.x {
                                // lower area doesnt stick out

                                //combined_line.push(upper_area.clone());
                                //upper_line.remove(0);
                            }

                            else {
                                // lower sticks out first

                                let mut mod_lower_area = lower_area.clone();
                                mod_lower_area.occurence_length = upper_area.x - lower_area.x;
                                if mod_lower_area.occurence_length > 20 || mod_lower_area.occurence_length <= 0 {
                                    // println!("\nWARNING, something might not be well, heres your debug info dump :3 \nlower_area: {:?}, upper_area: {:?}, mod_lower_area: {:?}, identification Number: 1", lower_area, upper_area, mod_lower_area);
                                }

                                combined_line.push(mod_lower_area);
                            }
                        }

                        else if lower_area.x >= upper_area.x {
                            // lower sticks out last

                            lower_area.x = upper_area_reach + 1;
                            lower_area.occurence_length = lower_area_reach - lower_area.x + 1;
                            if lower_area.occurence_length > 20 || lower_area.occurence_length <= 0 {
                                // println!("\nWARNING, something might not be well, heres your debug info dump :3 \nlower_area: {:?}, upper_area: {:?}, mod_lower_area: {:?}, identification Number: 2", lower_area, upper_area, mod_lower_area);
                            }
                            
                            combined_line.push(upper_area.clone());
                            upper_line.remove(0);
                            
                            continue;
                        }

                        else {
                            // lower sticks out both ends

                            let mut mod_lower_area = lower_area.clone();
                            mod_lower_area.occurence_length = upper_area.x - lower_area.x;
                            if mod_lower_area.occurence_length > 20 || mod_lower_area.occurence_length <= 0 {
                                // println!("\nWARNING, something might not be well, heres your debug info dump :3 \nlower_area: {:?}, upper_area: {:?}, mod_lower_area: {:?}, identification Number: 3", lower_area, upper_area, mod_lower_area);
                            }

                            lower_area.x = upper_area_reach + 1;
                            lower_area.occurence_length = lower_area_reach - lower_area.x + 1;

                            combined_line.push(mod_lower_area);
                            combined_line.push(upper_area.clone());
                            upper_line.remove(0);

                            continue;
                        }
                    }

                    lower_index = lower_index + 1;
                }
                
                combined_line.append(upper_line);
                combined_layer.push(combined_line);
            }
        }
        Ok(combined_layer)
    }

    
}


// MIGHT ALL BE AVOIDABLE WITH TRAITS
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(PartialEq)]
pub struct ShadingOptimisedScreenPoints {
    shading_signature: u32,
    occurence_length: u32,
    x: u32
}

impl ShadingOptimisedScreenPoints {

    fn new(shading_signature: u32, occurence_length: u32, x: u32) -> ShadingOptimisedScreenPoints{
        ShadingOptimisedScreenPoints {
            shading_signature,
            occurence_length,
            x
        }
    }

    fn new_from_optimised(original: OptimisedScreenPoint, shading_signature: u32) -> ShadingOptimisedScreenPoints {
        ShadingOptimisedScreenPoints::new(shading_signature, original.occurence_length, original.x)
    }

    pub fn layer(lower_layer: Vec<Vec<ShadingOptimisedScreenPoints>>, mut upper_layer: Vec<Vec<ShadingOptimisedScreenPoints>>) -> Result<Vec<Vec<ShadingOptimisedScreenPoints>>, Box<dyn std::error::Error>> {
        let mut combined_layer: Vec<Vec<ShadingOptimisedScreenPoints>> = vec!();
        for (index, line) in lower_layer.iter().enumerate() {
            if upper_layer.get(index).unwrap().len() == 0 {
                combined_layer.push(line.clone());
                continue;
            } else {
                let mut combined_line: Vec<ShadingOptimisedScreenPoints> = vec!();
                for (area_index, lower_area) in line.iter().enumerate() {
                    match upper_layer.get(index).unwrap().first() {
                        Some(_) => {},
                        _ => {combined_line.push(lower_area.clone()); break;}
                    }
                    while lower_area.x > upper_layer.get(index).unwrap().first().unwrap().x + upper_layer.get(index).unwrap().first().unwrap().occurence_length {
                            combined_line.push(upper_layer.get(index).unwrap().first().unwrap().clone());
                            upper_layer.get_mut(index).unwrap().remove(0);
                    } if lower_area.x + lower_area.occurence_length < upper_layer.get(index).unwrap().first().unwrap().x {
                        combined_line.push(lower_area.clone());
                        continue;
                    } else {
                        if lower_area.x < upper_layer.get(index).unwrap().first().unwrap().x {
                            combined_line.push(ShadingOptimisedScreenPoints::new(lower_area.shading_signature, upper_layer.get(index).unwrap().first().unwrap().x - lower_area.x, lower_area.x));
                        } else if lower_area.x > upper_layer.get(index).unwrap().first().unwrap().x {
                            combined_line.push(upper_layer.get(index).unwrap().first().unwrap().clone());
                            combined_line.push(ShadingOptimisedScreenPoints::new(lower_area.shading_signature, lower_area.occurence_length + lower_area.x - upper_layer.get(index).unwrap().first().unwrap().x + upper_layer.get(index).unwrap().first().unwrap().occurence_length, upper_layer.get(index).unwrap().first().unwrap().x + upper_layer.get(index).unwrap().first().unwrap().occurence_length));
                            upper_layer.get_mut(index).unwrap().remove(0);
                        }
                    }
                }
                combined_line.append(upper_layer.get_mut(index).unwrap());
                combined_layer.push(combined_line);
            }
        }
        Ok(combined_layer)
    }
}

pub fn sort_screen_points (mut input_list: Vec<ScreenPoint>) -> Vec<Vec<ScreenPoint>>{
    
    input_list.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut orderd_vec_list: Vec<Vec<ScreenPoint>> = vec!();
    // println!("{:?}", inner_list);

    for value in &input_list{
        if orderd_vec_list.len() > 0{
            if orderd_vec_list[orderd_vec_list.len() -1].first().unwrap().y != value.y{
                orderd_vec_list.push(input_list.iter().map(|instance| *instance).filter(|instance| instance.y == value.y).collect());
            }
        } else {
            orderd_vec_list.push(input_list.iter().map(|instance| *instance).filter(|instance| instance.y == value.y).collect());
        }
        
    }

    // println!("{:?}", outer_list);

    for screen_point_list in orderd_vec_list.iter_mut(){

        screen_point_list.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        *screen_point_list = screen_point_list.iter().enumerate().filter_map({|(index, a)| if index == 0 {return Some(*a);} else if screen_point_list.get(index -1).unwrap() != a {Some(*a)} else {None}}).collect::<Vec<ScreenPoint>>();
    }

    orderd_vec_list

}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullReqeustRecvPackage {
    pub title: String,
    pub description: String,      //Karina says this would be beneficial
    pub resolution: (u64, u64),
    pub matrix: Vec<StringRotationMatrix>,
    pub theta: Vec<f32>,
    pub camera_position: Vector3D,
    pub focus_point: Vector3D,
    pub fov: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullReqeustSendPackage {
    pub title: String,
    pub description: String,      //Karina says this would be beneficial
    pub resolution: (u16, u16),
    pub color_list: Vec<u8>,
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
    // println!("parsing: {}", input_text);
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
        println!("translating Matrix");
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
                    Err(_) => return Err(format!("unable to translate &str into operation, key: '{}' is neither contained in hashmap nor parsable", cleaned_key.as_str()))
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
                    Err(_) => return Err(format!("unable to translate &str into operation, key: '{}' is neither contained in hashmap nor parsable", cleaned_key.as_str()))
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
                    Err(_) => return Err(format!("unable to translate &str into operation, key: '{}' is neither contained in hashmap nor parsable", cleaned_key.as_str()))
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




























#[derive(Clone)]
#[derive(Debug)]
pub struct Corner<'a> {
    pub position: Vector3D,
    pub connection_1: Option<Rc<RefCell<Corner<'a>>>>,
    pub connection_2: Option<Rc<RefCell<Corner<'a>>>>,
}

impl Corner<'_> {
    
    pub fn new<'a> (position: Vector3D, connection_1: Option<Rc<RefCell<Corner<'a>>>>, connection_2: Option<Rc<RefCell<Corner<'a>>>>) -> Corner<'a> {
        // Add conection check
        Corner {
            position,
            connection_1,
            connection_2
        }
    }
}

impl PartialEq for Corner<'_> {

    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleCorner {
    pub position: Vector3D,
    pub normal: Vector3D,
    pub texture_position: Vector2D
}

impl TriangleCorner {

    pub fn new (position: Vector3D, normal: Vector3D, texture_position: Vector2D) -> TriangleCorner {
        TriangleCorner {
            position,
            normal,
            texture_position
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TriangleCorner>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3
            }, wgpu::VertexAttribute {
                offset: std::mem::size_of::<Vector3D>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3
            }, wgpu::VertexAttribute {
                offset: (std::mem::size_of::<Vector3D>() * 2) as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x2
            }]
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Polygon<'a> {
    corners: Vec<Rc<RefCell<Corner<'a>>>>
}

impl <'a> Polygon<'a> {

    pub fn new (corners: Vec<Rc<RefCell<Corner<'a>>>>) -> Result<Polygon, String> {
        // println!("creating Triangle");

        // checking if Triangle  is valid by verifying each corner is connected to 2 other corners
        for (index_1, corner) in corners.iter().enumerate() {
            // println!("\nchecking validity of a corner");
            let mut connection_counter = 0;
            for (index_2, connection) in corners.iter().enumerate() {
                // println!("comparing corner to other corner");
                if index_1 == index_2 {

                } else if corner.borrow().connection_1.clone().unwrap().borrow().position == connection.borrow().position || corner.borrow().connection_2.clone().unwrap().borrow().position == connection.borrow().position {
                    // println!("a compareson was completed");
                    connection_counter += 1;
                }
            }
            if connection_counter != 2 {
                return Err("Not all corners are properly connected, cant make Triangle :(".to_string())
            }
        }
        if corners.len() < 3 {
            return Err("to few corners to Polygon :(".to_string())
        }
        Ok(Polygon {
            corners
        })
    }

    pub fn new_from_ordered (corners: Vec<Vector3D>) -> Result<Polygon<'a>, String> {
        // println!("creating Triangle");

        // checking if positions are planenar
        if corners.len() < 3 {
            return Err("to few corners for Triangle creation".to_string());
        }

        let mut corner_structs = vec!();
        for corner in corners {
            let corner_struct = Corner::new(corner, None, None);
            corner_structs.push(Rc::new(RefCell::new(corner_struct)));
        }

        for (index, corner_struct) in corner_structs.iter().enumerate() {
            let mut next_coner = corner_structs.get(index + 1);
            if next_coner == None {
                next_coner = corner_structs.first();
            }
            let mut previous_coner = None;
            if index > 0 {
                previous_coner = corner_structs.get(index - 1);
            }
            if previous_coner == None {
                previous_coner = corner_structs.last();
            }
            corner_struct.borrow_mut().connection_2 = Some(Rc::clone(next_coner.unwrap()));
            corner_struct.borrow_mut().connection_1 = Some(Rc::clone(previous_coner.unwrap()));
        }

        Ok(Polygon::new(corner_structs)?)
    }
}


#[derive(Clone)]
#[derive(Debug)]
pub struct Triangle<'a> {
    corners: [TriangleCorner; 3],
    texture: &'a str
}

impl <'a> Triangle<'a> {

    pub fn new (corners: Vec<TriangleCorner>, texture: &str) -> Result<Triangle, String> {
        // println!("creating Triangle");

        if corners.len() != 3 {
            return Err("to few or to many corners to make Triangle :(".to_string())
        }

        // checking if each corner is unique
        for corner in corners.iter() {

            let mut counter = 0;

            for corner_2 in corners.iter() {

                if corner == corner_2 {
                    counter = counter + 1;
                }
            }

            if counter > 1 {
                return Err("Not all corners are unique :(".to_string())
            }
        }

        let corner_array = [corners[0].clone(), corners[1].clone(), corners[2].clone()];
        
        Ok(Triangle{
            corners: corner_array,
            texture
        })
    }

    pub fn flatten(faces: Vec<Triangle>) -> (Vec<TriangleCorner>, Vec<u32>) {
        let mut indexes = vec!();
        let mut shader_triangles = vec!();

        for triangle in faces {

            let mut texture_reference: u32 = 0;

            shader_triangles.append(&mut triangle.corners.to_vec());
            
            let current_index_length = indexes.len();
            for index in 0..3 {
                indexes.push((current_index_length + index) as u32);
            }
        }

        //println!("flattend triangles: {:?}", (shader_triangles.clone(), indexes.clone()));

        (shader_triangles, indexes)
    }
 
    pub fn center_position (&self) -> Vector3D {
        let mut corner_sum = Vector3D::origin();
        for corner in self.corners.iter() {
            corner_sum = corner_sum + corner.position;
        }
        corner_sum * (1.0 /self.corners.len() as f32)
    }

    pub fn get_plane (&self) -> Result<Plane, Box<dyn std::error::Error>> {
        let normal = (self.center_position() - self.corners.get(0).unwrap().position).get_orthagonal(&(self.center_position() - self.corners.get(1).unwrap().position))?;
        let value = normal.dot_product(self.corners.get(0).unwrap().position);

        Ok(Plane {
            x: normal.x,
            y: normal.y,
            z: normal.z,
            value
        })
    }

    pub fn create_cown (&self, pyramid_peak: Vector3D) -> Result<Vec<Plane>, String> {

        let mut sides = vec!();
        
        for index in 0..3 {

            let current_corner = self.corners.get(index % 3).unwrap();
            let next_corner = self.corners.get((index + 1) % 3).unwrap();

            let spanning_vector_1 = (current_corner.position - pyramid_peak).normalize();
            let spanning_vector_2 = (next_corner.position - pyramid_peak).normalize();

            let side = Plane::new_from_points(pyramid_peak, pyramid_peak + spanning_vector_1, pyramid_peak + spanning_vector_2)?;
            sides.push(side);
        }
        
        Ok(sides)
    }

    // Will have to rework corners to work with shared ownership, could have had this though sooner lmao

    pub fn cut (self, plane: Plane) -> Result<Vec<Triangle<'a>>, String> {
        let self_plane = self.get_plane().unwrap();
        let cuting_line = self_plane.find_plane_interception(plane);
        let mut cut_triangles = vec!();

        // Placeholder corner for later use, connections dont mean shit
        let mut cuts = vec!();
        let mut severed_corners = vec!();
        
        for index in 0..3 {

            let current_corner = self.corners.get(index % 3).unwrap();
            let next_corner = self.corners.get((index + 1) % 3).unwrap();

            let connection = Line::new(current_corner.position, next_corner.position, Color::white());
            let collision = connection.find_intercept(&cuting_line);

            match collision {
                Some(location) => {

                    let relative_collision_length = (current_corner.position - location).occurence_length() / (current_corner.position - next_corner.position).occurence_length();
                    let texture_anchor_vector = current_corner.texture_position - next_corner.texture_position;
                    let cut_texture_anchor = next_corner.texture_position + texture_anchor_vector * relative_collision_length;

                    let cut_corner = TriangleCorner::new(location, current_corner.normal, cut_texture_anchor);
                    cuts.push(cut_corner);
                },
                _ => {}
            }

            if cuts.len() % 2 != 0 {

                severed_corners.push(next_corner);
            }
        }

        if cuts.len() % 2 != 0 {

            return Err("triangle was only partially cut, should be impossible".to_string());
        }

        let remaining_corners = severed_corners.iter().filter(|a| !severed_corners.contains(a)).map(|a| *a).collect::<Vec<&TriangleCorner>>();

        for corner_set in vec!(severed_corners, remaining_corners) {

            if corner_set.len() == 1 {

                let mut corners = vec!((**corner_set.first().unwrap()).clone());
                corners.append(&mut cuts);
                let triangle = Triangle::new(corners, self.texture)?;

                cut_triangles.push(triangle);
            } 

            else {

                // This might cause problems where the 2 triangles produced overlap, or it might not but I didnt take the time to think about it hard enough

                let corners_1 = vec!(corner_set[0].clone(), corner_set[1].clone(), cuts[0].clone());
                let corners_2 = vec!(corner_set[1].clone(), cuts[1].clone(), cuts[0].clone());

                for corners in vec!(corners_1, corners_2) {

                    let triangle = Triangle::new(corners, self.texture)?;
                    cut_triangles.push(triangle);
                }
            }
        }

        Err("under construction :3".to_string())
    }

    pub fn draw (&self, screen: &SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, color: Color) -> Result<Vec<Vec<OptimisedScreenPoint>>, String> {
        let mut screenpoints = vec!();

        for index in 0..3 {

            let current_corner = self.corners.get((index) % 3).unwrap();
            let next_corner = self.corners.get((index + 1) % 3).unwrap();

            let connection_line = Line::new(current_corner.position, next_corner.position, color);
            let mut new_screenpoints = connection_line.render_line(screen, camera_position, x_resolution, y_resolution, 1.0)?;
            screenpoints.append(&mut new_screenpoints);

        }

        let optimized_screenpoints = OptimisedScreenPoint::optimise_screen_point_collection(screenpoints, y_resolution as i64, x_resolution as i64).unwrap();

        Ok(optimized_screenpoints)
    }

    pub fn draw_full (&self, screen: &SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, color: Color) -> Result<Vec<Vec<OptimisedScreenPoint>>, String> {
        let mut optimized_screenpoints = self.draw(screen, camera_position, x_resolution, y_resolution, color)?;
        let mut filled_optimized_screenpoints = vec!();

        for line in optimized_screenpoints.iter_mut() {

            let mut filled_line = vec!();

            if line.len() == 1 {
                filled_line.push(line.first().unwrap().clone());
            } 
            
            else {

                if line.len() % 2 != 0 {

                    line.remove(1);
                    // return Err("cannot fill line with uneven number of boundaries".to_string());
                }

                for (index, pixel) in line.iter().enumerate() {

                    if (index % 2) == 0 {
                        continue;
                    }

                    let recent_pixel = match line.get(index - 1) {
                        Some(p) => {p},
                        _ => {return Err("Something went wrong during filled line creation event though it shouldnt have been able to".to_string());}
                    };

                    let pixel_bridge = OptimisedScreenPoint::new(color, pixel.x - recent_pixel.x + 1, recent_pixel.x);
                    filled_line.push(pixel_bridge);
                }
            }

            filled_optimized_screenpoints.push(filled_line);
        }

        Ok(filled_optimized_screenpoints)
    }

    pub fn shading_index_draw (&self, screen: &SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, shading_index: u32) -> Result<Vec<Vec<ShadingOptimisedScreenPoints>>, String> {
        let unshaded_result = self.draw(screen, camera_position, x_resolution, y_resolution, Color::black())?;
        let mut result = vec!();
        for line in unshaded_result {
            let mut converted_line = vec!();
            for entry in line {
                converted_line.push(ShadingOptimisedScreenPoints::new_from_optimised(entry, shading_index));
            }
            result.push(converted_line);
        }
        return Ok(result)
    }

    pub fn render (&self, screen: &SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32, color: Color, lightsource: Vector3D) -> Result<Vec<Vec<OptimisedScreenPoint>>, String> {
        let unshaded_render = self.draw(screen, camera_position, x_resolution, y_resolution, color);
        // let mut corner_lithing = vec!();

        let orientation_corner = self.corners.get(0).unwrap();
        let orientation_corner_point = Point::new(orientation_corner.position, color);
        let (orientation_corner_screenpoint, _) =  orientation_corner_point.draw(camera_position, screen, x_resolution, y_resolution)?;
        
        let connection_1 = self.corners.get(1).unwrap();
        let connection_1_point = Point::new(connection_1.position, color);
        let (connection_1_screenpoint, _) =  connection_1_point.draw(camera_position, screen, x_resolution, y_resolution)?;

        let connection_2 = self.corners.get(2).unwrap();
        let connection_2_point = Point::new(connection_2.position, color);
        let (connection_2_screenpoint, _) =  connection_2_point.draw(camera_position, screen, x_resolution, y_resolution)?;

        let connection_1_vector = (orientation_corner_screenpoint.x - connection_1_screenpoint.x, orientation_corner_screenpoint.y - connection_1_screenpoint.y);
        // let connection_1_lithingshift = lithing_function(orientation_corner.position, self.get_plane().unwrap(), color, lightsource) - lithing_function(orientation_corner.connection_1.clone().unwrap().borrow().position, self.get_plane().unwrap(), color, lightsource);
        

        // calculate each pixels location as vector combination of 3 connected corners, use vector combination to estimate result
        // works because simplification of sine at low values to linear function
        // THIS ONLY WORKS WHEN TriangleS ARE SMALL ENOUGH => Angles to lightsource only change slightly
        // nvm, just gonna make it so that we use triangles for everything, makes a lot of shit way easier

        Err("sorry, WIP :3".to_string())
    }
}

// pub fn triangualize_corners (corners: Vec<Corner>)
// need to find "save" planes, any plane including four or more corners

// assumes all Triangles are of simular size
// assumes all Triangles are properly cut and dont collide
// doesnt adress eitehr issue in the name of performance -> easier to calculate once during Model creation rather then during every render
pub fn determin_illumination<'a> (mut triangles: Vec<Triangle<'a>>, screen: &SquareSurface, camera_position: Vector3D, x_resolution: u32, y_resolution: u32) -> Result<(Vec<Triangle<'a>>, Vec<Triangle<'a>>), String> {
    triangles.sort_by({|a, b| (camera_position - a.center_position()).occurence_length().abs().total_cmp(&(camera_position - b.center_position()).occurence_length().abs())});
    triangles.reverse();

    let mut full_draw = triangles.first().unwrap().shading_index_draw(screen, camera_position, x_resolution, y_resolution, 0)?;
    for (index, triangle) in triangles.iter().enumerate() {
        full_draw = ShadingOptimisedScreenPoints::layer(full_draw, triangle.shading_index_draw(screen, camera_position, x_resolution, y_resolution, index as u32)?).unwrap();
    }

    let mut fused_full_draw = vec!();
    for line in full_draw.iter() {
        fused_full_draw.append(&mut line.clone());
    }
    fused_full_draw.sort_by({|a, b| a.shading_signature.cmp(&b.shading_signature)});

    let mut fully_lit_indexs = vec!();
    let mut fully_shadowed_indexs = vec!();
    let mut partially_shadowed_indexs: Vec<(u32, Vec<u32>)> = vec!();

    let mut overshadowed_triangles = vec!();
    let mut lit_triangles = vec!();

    for (shading_index, triangle) in triangles.iter().enumerate() {
        let mut final_entry_of_index = 0;
        let mut current_entry = match fused_full_draw.get(0) {
            Some(area) => {
                if area.shading_signature != shading_index as u32 {
                    fully_shadowed_indexs.push(shading_index);
                    continue;
                }
                area
            }
            _ => {
                fully_shadowed_indexs.push(shading_index);
                break;
            }
        };
        while shading_index as u32 == current_entry.shading_signature {
            current_entry = match fused_full_draw.get(final_entry_of_index + 1) {
                Some(area) => {
                    if area.shading_signature != shading_index as u32 {
                        break;
                    } else {
                        final_entry_of_index = final_entry_of_index + 1;
                    }
                    area
                }
                _ => {
                    break;
                }
            };
        }

        let shading_index_slice = &fused_full_draw[0..final_entry_of_index];
        let individual_triangle_draw = triangle.shading_index_draw(screen, camera_position, x_resolution, y_resolution, shading_index as u32)?;
        let mut fused_individual_triangle_draw = vec!();

        for line in individual_triangle_draw.iter() {
            fused_individual_triangle_draw.append(&mut line.clone());
        }

        if shading_index_slice == fused_individual_triangle_draw {
            fully_lit_indexs.push(shading_index);
        } else {
            let mut relevant_range = (None, None);
            for (index, layer) in individual_triangle_draw.iter().enumerate() {
                if layer.is_empty() {
                    if relevant_range.0 == None {
                        continue;
                    } else {
                        relevant_range.1 = Some(index);
                        break;
                    }
                } else if relevant_range.0 == None {
                    relevant_range.0 = Some(index)
                }
            }
            let top_layerd_slice = ShadingOptimisedScreenPoints::layer(full_draw[relevant_range.0.unwrap()..relevant_range.1.unwrap()].to_vec(), individual_triangle_draw[relevant_range.0.unwrap()..relevant_range.1.unwrap()].to_vec()).unwrap();
            let default_slice = full_draw[relevant_range.0.unwrap()..relevant_range.1.unwrap()].to_vec();

            let mut fused_top_layerd_slice = vec!();
            for line in top_layerd_slice.iter() {
                fused_top_layerd_slice.append(&mut line.clone());
            }

            let mut fused_default_slice = vec!();
            for line in default_slice.iter() {
                fused_default_slice.append(&mut line.clone());
            }

            let mut overshadowing_indexs: Vec<u32> = fused_default_slice.iter().filter({|a| fused_top_layerd_slice.contains(a) || a.shading_signature == shading_index as u32}).map({|a| a.shading_signature}).collect();
            overshadowing_indexs.sort_by({|a, b| a.cmp(b)});
            overshadowing_indexs = overshadowing_indexs.iter().enumerate().filter({|(index, a)| if *index == 0 {true} else {overshadowing_indexs.get(index - 1).unwrap() != *a}}).map({|(_index, a)| *a}).collect();
            partially_shadowed_indexs.push((shading_index as u32, overshadowing_indexs));
        }

        for (triangle_index, shadow_indexs) in partially_shadowed_indexs.iter() {
            let mut triangles_in_light = vec!(triangles.get(*triangle_index as usize).unwrap().clone()); //should be guaranted to work (I think)
            let mut new_overshadowed_triangles = vec!();
            for shadow_index in shadow_indexs.iter() {
                let overshadowing_triangle = triangles.get(*shadow_index as usize).unwrap();
                let overshadowing_triangle_center = overshadowing_triangle.center_position();
                let shadow_cone_sides = overshadowing_triangle.create_cown(camera_position)?;
                let mut cut_lit_triangles = vec!();
                for triangle in triangles_in_light.iter() {
                    for side in shadow_cone_sides.iter() {
                        cut_lit_triangles.append(&mut triangle.clone().cut(side.clone())?);
                    }
                }
                let mut new_lit_triangles = vec!();
                for triangle in cut_lit_triangles.iter() {
                    let connection_line = Line::new(triangle.center_position(), overshadowing_triangle_center, Color::white());
                    for side in shadow_cone_sides.iter() {
                        let collision = connection_line.find_intercept_plane(side);
                        if collision == None {
                            new_lit_triangles.push(triangle.clone());
                        } else {
                            new_overshadowed_triangles.push(triangle.clone());
                        }
                    }
                }
                triangles_in_light = new_lit_triangles;
            }
            overshadowed_triangles.append(&mut new_overshadowed_triangles);
            lit_triangles.append(&mut triangles_in_light);
        }
    }
    lit_triangles.append(&mut triangles.clone().iter().enumerate().filter_map(|(index, a)| if fully_lit_indexs.contains(&index) {Some(a.clone())} else {None}).collect());
    overshadowed_triangles.append(&mut triangles.iter().enumerate().filter_map(|(index, a)| if fully_shadowed_indexs.contains(&index) {Some(a.clone())} else {None}).collect());

    Ok((lit_triangles, overshadowed_triangles))
}

// shading should happen via shading signature and following comparison of individual drawings with the combined one
// not in the combined one -> completly overshadowed
// partially in the combinded one -> precise shadow collision calculations with overlappers
// completly identical -> completly visible


// WIP, this part would be one of the first to write for GPU
// in theory a lot can be done here to make lithing better
pub fn lithing_function (position: Vector3D, plane: Plane, plane_color: Color, lightsource: Vector3D) -> Color {
    let lightray = position - lightsource;
    let normal = Vector3D::new(plane.x, plane.y, plane.z);

    let factor = (1.0 - lightray.find_angle(normal).cos()).clamp(0.2, 1.0);
    let result = plane_color * factor;
    result
}



pub fn create_sphere<'a>(position: Vector3D, size: u32, x_layers: u32, y_layers: u32) -> Vec<Triangle<'a>> {

    let mut verteces = vec!();
    let mut faces = vec!();

    for y_layer in 0..y_layers {

        let mut layer_verteces = vec!();

        let center = position + Vector3D::new(0.0, 1.0, 0.0) * (((y_layers as f32 / 2.0) - y_layer as f32)) * (1.0 / y_layers as f32) * size as f32 * 2.0;
        let radius = (y_layer as f32 / y_layers as f32 * 3.141).sin() * size as f32;


        for x_layer in 0..x_layers {

            let current_rotation = x_layer as f32 / x_layers as f32 * 2.0 * 3.141;
            let position_vector = Vector3D::new(
                current_rotation.sin(),
                0.0,
                current_rotation.cos(),
            ) * radius;


            let vertece = center + position_vector;
            layer_verteces.push(vertece);
        }

        verteces.push(layer_verteces);
    }

    for (index, layer) in verteces.iter().enumerate() {

        let mut current_layer = layer;
        let mut next_layer = match verteces.get(index + 1) {
            Some(i) => {i},
            _ => {break;}
        };

        let mut iterartion = false;

        for _iter in 0..2 {
            
            for (v_index, vertece) in current_layer.iter().enumerate() {

                let mut opposing_vertece = next_layer.get(v_index).unwrap();

                if iterartion {
                    opposing_vertece = match next_layer.get(v_index + 1) {
                        Some(i) => {i},
                        _ => {next_layer.first().unwrap()}
                    };
                }

                let other_vertece = match current_layer.get(v_index + 1) {
                    Some(i) => {i},
                    _ => {current_layer.first().unwrap()}
                };

                let triangle = match Triangle::new(vec!(
                    TriangleCorner::new(*vertece, Vector3D::origin(), Vector2D::origin()),
                    TriangleCorner::new(*opposing_vertece, Vector3D::origin(), Vector2D::origin()),
                    TriangleCorner::new(*other_vertece, Vector3D::origin(), Vector2D::origin())
                ), "playceholder :3"){
                    Ok(x) => {x},
                    _ => {continue;}
                };

                faces.push(triangle);
            }

            let swapper = current_layer;
            current_layer = next_layer;
            next_layer = swapper;
            iterartion = true;
        }
    }

    faces
}







// struct Texture {
//     pixels: Vec<Vec<ScreenPoint>>
// }