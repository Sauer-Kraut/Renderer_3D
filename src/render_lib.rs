use crate::calc_structs::*;
use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;
use std::sync::{Arc, Mutex};











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
pub struct Triangle<'a> {
    pub corners: [TriangleCorner; 3],
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

            let mut triangle_indexes = vec!();

            for corner in triangle.corners {

                let mut finished = false;

                for (inner_index, entered_corner) in shader_triangles.iter().enumerate() {
                    
                    if &corner == entered_corner {
                        triangle_indexes.push(inner_index as u32 + 0);
                        finished = true;
                        break;
                    }
                }

                if !finished {

                    triangle_indexes.push(shader_triangles.len() as u32 + 0);
                    shader_triangles.push(corner);
                }
            }

            // triangle_indexes.sort_by(|a, b| a.cmp(b));
            // triangle_indexes.reverse();

            indexes.append(&mut triangle_indexes);
        }

        // println!("flattend triangles: {:?}", (shader_triangles.clone(), indexes.clone()));

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

        Ok(Plane::new(
            normal.x,
            normal.y,
            normal.z,
            value
        )?)
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

    // Will have to rework corners to work with shared ownership, could have had this thought sooner lmao

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










pub struct DynObject<'a> {
    parts: Vec<(Model<'a>, Vector3D, Arc<Mutex<dyn Fn(Model, Vector3D, f32) -> (Model, Vector3D)>>)>,   // Vector 3D represents the Model position relativ to the Object origin
    position: Vector3D                                                                            // Vector 3D represents the Object origin
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Object<'a> {
    parts: Vec<(Model<'a>, Vector3D)>,   // Vector 3D represents the Model position relativ to the Object origin
    position: Vector3D              // Vector 3D represents the Object origin
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Model<'a> {
    pub faces: Vec<Triangle<'a>>,
    texture_path: String
}


impl <'a> DynObject<'a> {

    pub fn new(parts: Vec<(Model<'a>, Vector3D, Arc<Mutex<dyn Fn(Model, Vector3D, f32) -> (Model, Vector3D)>>)>, position: Vector3D) -> DynObject<'a> {
        DynObject { 
            parts, 
            position 
        }
    }

    pub fn generate_object(&self, t: f32) -> Object<'a> {

        let mut transformed_models = vec!();

        for (model, placement, tranformation_fn) in self.parts.iter() {

            let locked_transformation_fn = tranformation_fn.lock().unwrap();
            let transformation = locked_transformation_fn(model.clone(), *placement, t);

            transformed_models.push(transformation);
        }

        Object {
            parts: transformed_models,
            position: self.position,
        }
    } 
}






impl <'a> Model<'a> {
    
    pub fn new(faces: Vec<Triangle<'a>>, texture_path: String) -> Model {
        Model { 
            faces,
            texture_path
        }
    }

    pub fn import_obj(file_path: &str) -> Result<Model<'a>, Box<dyn std::error::Error>> {
        let mut faces = vec!();
        let mut positions = vec!();
        let mut normals = vec!();
        let mut texture_positions = vec!();

        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.get(0) {
                Some(&"v") => {
                    let position = Vector3D::new(
                        parts[1].parse()?,
                        parts[2].parse()?,
                        parts[3].parse()?,
                    );
                    positions.push(position);
                }
                Some(&"vn") => {
                    let normal = Vector3D::new(
                        parts[1].parse()?,
                        parts[2].parse()?,
                        parts[3].parse()?,
                    );
                    normals.push(normal);
                }
                Some(&"vt") => {
                    let texture_position = Vector2D::new(
                        parts[1].parse()?,
                        parts[2].parse()?,
                    );
                    texture_positions.push(texture_position);
                }
                Some(&"f") => {
                    let mut face_corners = vec!();
                    for i in 1..parts.len() {
                        let indices: Vec<&str> = parts[i].split('/').collect();
                        let position_index: usize = indices[0].parse()?;
                        let texture_index: usize = indices.get(1).unwrap_or(&"0").parse().unwrap_or(0);
                        let normal_index: usize = indices.get(2).unwrap_or(&"0").parse().unwrap_or(0);

                        let position = positions[position_index - 1];
                        let normal = if normal_index > 0 { normals[normal_index - 1] } else { Vector3D::origin() };
                        let texture_position = if texture_index > 0 { texture_positions[texture_index - 1] } else { Vector2D::origin() };

                        face_corners.push(TriangleCorner::new(position, normal, texture_position));
                    }
                    if face_corners.len() == 3 {
                        faces.push(Triangle::new(face_corners, "default_texture")?);
                    } else if face_corners.len() == 4 {
                        faces.push(Triangle::new(vec![face_corners[0].clone(), face_corners[1].clone(), face_corners[2].clone()], "default_texture")?);
                        faces.push(Triangle::new(vec![face_corners[0].clone(), face_corners[2].clone(), face_corners[3].clone()], "default_texture")?);
                    }
                }
                _ => {}
            }
        }

        Ok(Model::new(faces, "default_texture".to_string()))
    }

    pub fn flatten(self) -> (Vec<TriangleCorner>, Vec<u32>) {
        Triangle::flatten(self.faces)
    }

    pub fn displace(&mut self, displacement: Vector3D) {

        for face in self.faces.iter_mut() {

            for corner in face.corners.iter_mut() {

                corner.position = corner.position + displacement;
            }
        }
    }

    pub fn combine(self, model2: Model<'a>) -> Model<'a> {
        let mut combined_faces = self.faces.clone();
        combined_faces.extend(model2.faces.clone());
        Model::new(combined_faces, "combined_texture".to_string())
    }
}






impl <'a> Object<'a> {

    pub fn new(parts: Vec<(Model<'a>, Vector3D)>, position: Vector3D) -> Object<'a> {
        Object { 
            parts, 
            position 
        }
    }

    pub fn flatten(self) -> (Vec<TriangleCorner>, Vec<u32>) {
        let mut face_output = vec!();
        let mut index_output = vec!();

        for (mut model, model_position) in self.parts {

            model.displace(model_position + self.position);
            let mut flattend_model = model.flatten();
            
            face_output.append(&mut flattend_model.0);
            index_output.append(&mut flattend_model.1);
        }

        (face_output, index_output)
    }

    pub fn into_model(self) -> Model<'a> {
        let mut faces = vec!();

        for (mut model, model_position) in self.parts {
            model.displace(model_position + self.position);
            faces.append(&mut model.faces);
        }

        Model::new(faces, "default_texture".to_string())
    }
}