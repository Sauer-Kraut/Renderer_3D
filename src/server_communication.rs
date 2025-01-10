use crate::calc_structs::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};









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
    pub timescale: f32,
    pub model: String
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