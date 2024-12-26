use std::ops::{Add, Mul, Sub};
use serde::{Deserialize, Serialize};
use bytemuck;











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
        // println!("getting orthagonal Vector");
        
        let cross = Vector3D::new(
            self.y * other.z - self.z * other.y, 
            self.z * other.x - self.x * other.z,  
            self.x * other.y - self.y * other.x, 
        );

        if self == &Vector3D::origin() || other == &Vector3D::origin() {

            Err("unable to find definitiv orthagonal")
        } else {

            Ok(cross)
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


        let mut orthangonal_plane = Plane::new(self_direction.x, self_direction.y, self_direction.z, self_direction.x * self.starting_point.x + self_direction.y* self.starting_point.y + self_direction.z * self.starting_point.z).unwrap();
        let other_line_point_start = orthangonal_plane.find_vector_interception(&other_line.starting_point, &mut (other_line.ending_point - other_line.starting_point))?;
        orthangonal_plane = Plane::new(self_direction.x, self_direction.y, self_direction.z, self_direction.x * self.ending_point.x + self_direction.y* self.ending_point.y + self_direction.z * self.ending_point.z).unwrap();
        let other_line_point_end = orthangonal_plane.find_vector_interception(&other_line.ending_point, &mut (other_line.ending_point - other_line.starting_point))?;

        let distance_start = (other_line_point_start - self.starting_point).occurence_length();
        let distance_end = (other_line_point_end - self.ending_point).occurence_length();

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
        let collsion_point = plane.find_vector_interception(&self.ending_point, &mut line_vector).unwrap(); // could fuck me in theorie when plane runs parralel to vector
        let connection_vector = self.starting_point - collsion_point;
        if !(connection_vector.occurence_length() > connection_vector.occurence_length() ||
           connection_vector.dot_product(Vector3D::new(1.0, 1.0, 1.0)) / connection_vector.dot_product(Vector3D::new(1.0, 1.0, 1.0)) < 0.0) {
            Some(collsion_point)
        } else {
            None
        }
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

    pub fn find_vector_interception(&self, vector_origin:&Vector3D, vector: &mut Vector3D) -> Option<Vector3D>{
        vector.normalize();
        let dot_product = vector.x * self.x + vector.y * self.y + vector.z * self.z;
        if dot_product == 0.0 {
            return None;
        }

        // not to big of a fan of this but it does make sense
        let collision_occurence_length = (self.value - vector_origin.x * self.x - vector_origin.y * self.y - vector_origin.z * self.z)
                                        / (vector.x * self.x + vector.y * self.y + vector.z * self.z);

        Some(*vector_origin + *vector * collision_occurence_length)
    }

    pub fn find_plane_interception(&self, plane: Plane) -> Line {
        let normal_vector = Vector3D::new(self.x, self.y, self.z);
        let additional_vector = Vector3D::new(self.x, self.y, self.z + 0.1);
        let any_plane_point = self.get_any_point();

        let mut plane_vector_1 = normal_vector.get_orthagonal(&additional_vector).unwrap();
        let mut plane_vector_2 = normal_vector.get_orthagonal(&plane_vector_1).unwrap();
        let mut plane_vector_3 = plane_vector_1 + plane_vector_2;

        let intercept_1 = plane.find_vector_interception(&any_plane_point, &mut plane_vector_1);
        let intercept_2 = plane.find_vector_interception(&any_plane_point, &mut plane_vector_2);
        let intercept_3 = plane.find_vector_interception(&any_plane_point, &mut plane_vector_3);

        match intercept_1 {
            None => {return Line::new(intercept_2.unwrap(), intercept_3.unwrap(), Color::black())},
            _ => {}
        }
        match intercept_2 {
            None => {return Line::new(intercept_1.unwrap(), intercept_3.unwrap(), Color::black())},
            _ => {}
        }

        Line::new(intercept_1.unwrap(), intercept_2.unwrap(), Color::black())
    }
}











// Thx ChatGPT
// Row based, each array one row
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub matrix: [[f32; 4]; 4],
}

impl Mat4 {
    // Create a new Mat4 with specified matrix
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Mat4 { matrix }
    }

    // Multiply the matrix with another Mat4
    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = (0..4).fold(0.0, |sum, k| sum + self.matrix[i][k] * other.matrix[k][j]);
            }
        }

        Mat4 { matrix: result }
    }

    pub fn to_collum_based(&self) -> [[f32; 4]; 4] {
        let mut output = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {

                output[i][j] = self.matrix[j][i];
            }
        }

        output
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        self.multiply(&rhs)
    }
}

impl std::ops::Mul<(Vector3D, f32)> for Mat4 {
    type Output = (Vector3D, f32);

    fn mul(self, rhs: (Vector3D, f32)) -> (Vector3D, f32) {
        let vec = rhs.0;
        let x = self.matrix[0][0] * vec.x + self.matrix[0][1] * vec.y + self.matrix[0][2] * vec.z + self.matrix[0][3] * rhs.1;
        let y = self.matrix[1][0] * vec.x + self.matrix[1][1] * vec.y + self.matrix[1][2] * vec.z + self.matrix[1][3] * rhs.1;
        let z = self.matrix[2][0] * vec.x + self.matrix[2][1] * vec.y + self.matrix[2][2] * vec.z + self.matrix[2][3] * rhs.1;
        let w = self.matrix[3][0] * vec.x + self.matrix[3][1] * vec.y + self.matrix[3][2] * vec.z + self.matrix[3][3] * rhs.1;

        return (Vector3D::new(x, y, z), w)
    }
}