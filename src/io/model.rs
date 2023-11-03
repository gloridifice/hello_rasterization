use std::fs::File;
use std::io::BufReader;
use std::mem::{size_of, size_of_val};
use std::time::SystemTime;
use obj::{load_obj, Obj};
use crate::io::model::Status::{Loaded, Unload};
use crate::Vec3;
use anyhow::Result;
use colored::{ColoredString, Colorize};

enum Status{
    Loaded(Vec<Vertex>, Vec<Face>),
    Unload
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub normal: Vec3,
    pub position: Vec3,
}

impl Vertex {
    pub fn new(normal: Vec3, position: Vec3) -> Vertex {
        Vertex {
            normal,
            position,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Face{
    pub normal: Vec3,
    pub vertices_indices: [usize; 3]
}

impl Face {
    pub fn new(normal: Vec3, vertices_indices: [usize; 3]) -> Face{
        Face{
            normal, vertices_indices
        }
    }
}


pub struct Model<'a> {
    path: &'a str,
    status: Status
}

impl Model<'_> {
    pub fn new(path: &str) -> Model {
        Model{
            path,
            status: Unload
        }
    }

    pub fn new_load(path: &str) -> Result<Model>{
        let mut ret = Model::new(path);
        ret.load()?;
        Ok(ret)
    }
    pub fn load(&mut self) -> Result<()>{
        println!("Start loading model...");
        let start_time = SystemTime::now();

        let input = BufReader::new(File::open(self.path)?);
        let model: Obj = load_obj(input)?;

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        for i in 0..model.vertices.len() {
            let ver = &model.vertices[i];
            let mut normal = ver.normal;
            normal[1] = -normal[1];
            let mut position = ver.position;
            position[1] = -position[1];
            vertices.push(Vertex::new(Vec3::from(normal), Vec3::from(position)));
        }

        for index in (0..model.indices.len()).step_by(3) {
            let indices = (model.indices[index], model.indices[index + 1], model.indices[index + 2]);
            let normal: Vec3 = (vertices[indices.0 as usize].normal + vertices[indices.1 as usize].normal + vertices[indices.2 as usize].normal).normalize();

            faces.push(Face::new(normal, [indices.0 as usize, indices.1 as usize, indices.2 as usize]))
        }

        self.status = Loaded(vertices, faces);
        if let Ok(elapsed) = start_time.elapsed() {
            println!("Model loaded, using time: {}ms", elapsed.as_millis());
        };

        Ok(())
    }

    pub fn vertices(&self) -> Result<&Vec<Vertex>>{
        let error = format!("Try to get vertices but model[{}] not loaded!", self.path);
        match &self.status {
            Loaded(vertices, _) => Ok(vertices),
            _ => anyhow::bail!(error.red())
        }
    }
    pub fn faces(&self) -> Result<&Vec<Face>>{
        let error = format!("Try to get faces but model[{}] not loaded!", self.path);
        match &self.status {
            Loaded(_, faces) => Ok(faces),
            _ => anyhow::bail!(error.red())
        }
    }
}
