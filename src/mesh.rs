//! generic code for meshes in 3D space
#![allow(dead_code)]
use itertools::Itertools;
use vulkano::buffer::{Subbuffer, BufferContents};
use vulkano_util::context::VulkanoContext;
use super::*;
use maths::Vector3;
use std::{collections::BTreeMap, io::{BufReader, BufRead}, fs::File};



/// Data for a mesh in 3D space
#[derive(Debug, Clone)]
pub struct Mesh<T: Position + Clone + Copy + BufferContents> {
    pub vertices: Vec<T>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}


impl<T: Position + Clone + Copy + BufferContents> Mesh<T> {
    pub fn new(
        vertices: Vec<T>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            vertices, normals: Vec::new(), indices
        }
    }

    pub const EMPTY: Mesh<T> = Mesh {
        vertices: Vec::new(),
        normals: Vec::new(),
        indices: Vec::new()
    };

    // sets the normals of the given mesh and returns a reference
    pub fn set_normals(
        &mut self,
        normals: Vec<Normal>,
    ) -> &mut Mesh<T>{
        self.normals = normals;
        self
    }

    // recalculates the normals of the given mesh, smooth shaded
    pub fn recalculate_normals(&mut self) -> &mut Mesh<T>{
        // init normals
        let mut normals: Vec<Vector3> = Vec::new();
        for _i in 0..self.vertices.len() {
            normals.push([0.0, 0.0, 0.0].into())
        }
        
        // create normals
        for i in (0..self.indices.len()).step_by(3) {
            let dir_one: Vector3 = {
                let dir: Vector3 = self.vertices[self.indices[i] as usize].pos().into();
                dir - self.vertices[self.indices[i + 2] as usize].pos().into()
            };
            let dir_two: Vector3 = {
                let dir: Vector3 = self.vertices[self.indices[i + 1] as usize].pos().into();
                dir - self.vertices[self.indices[i + 2] as usize].pos().into()
            };
            let normal = dir_two.cross(dir_one);

            // println!("{:?}, {:?}, {:?} => {:?}", self.vertices[i], self.vertices[i + 1], self.vertices[i + 2], normal);

            normals[self.indices[i + 0] as usize] += normal;
            normals[self.indices[i + 1] as usize] += normal;
            normals[self.indices[i + 2] as usize] += normal;
        }

        // normalise normals
        let normals = {
            let mut final_norms: Vec<Normal> = Vec::new();
            for normal in normals.iter_mut() {
                // println!("{:?}, {:?}", normal, normal.normalised());
                final_norms.push(normal.normalised().into());
                
            }
            final_norms
        };

        self.set_normals(normals);
        self
    }

    pub fn invert_normals(&mut self) {
        for mut normal in self.normals.iter_mut() {
            normal.normal[0] *= -1.0;
            normal.normal[1] *= -1.0;
            normal.normal[2] *= -1.0;
        }
    }

    /// returns a flat shaded version of the smooth shaded mesh called on
    pub fn flat_shaded(&self) -> Mesh<T> {
        let mut new_verts: Vec<T> = Vec::new();
    
        for i in (0..self.indices.len()).step_by(3) {
            new_verts.push(self.vertices[self.indices[i as usize + 0] as usize]);
            new_verts.push(self.vertices[self.indices[i as usize + 1] as usize]);
            new_verts.push(self.vertices[self.indices[i as usize + 2] as usize]);
        }
    
        let indices = (0..(new_verts.len()) as u32).collect_vec();
        let mut new_mesh = Mesh::new(new_verts, indices);
        new_mesh.recalculate_normals();
        new_mesh
    }

    /// sets the current mesh to be flat shaded
    pub fn flat_shade(&mut self) {
        let new = self.flat_shaded();
        self.vertices = new.vertices;
        self.normals = new.normals;
        self.indices = new.indices;
    }


    /// returns a smooth shaded version of the flat shaded mesh called on
    pub fn smooth_shaded(&self) -> Mesh<T> {
        let mut map: BTreeMap<Vector3, u32> = BTreeMap::new();
        let mut new_indices = Vec::new();

        for i in 0..self.vertices.len() {
            map.insert(self.vertices[i].pos().into(), i as u32);
        }

        for index in self.indices.iter() {
            new_indices.push(map.get(&self.vertices[*index as usize].pos().into()).unwrap().clone())
        }

        let mut out = Mesh::new(self.vertices.clone(), new_indices);
        out.recalculate_normals();

        out
    }

     /// sets the current mesh to be smooth shaded
     pub fn smooth_shade(&mut self) {
        let new = self.smooth_shaded();
        self.vertices = new.vertices;
        self.normals = new.normals;
        self.indices = new.indices;
    }


    pub fn components(&self) -> (Vec<T>, Vec<Normal>, Vec<u32>){
        (self.vertices.clone(), self.normals.clone(), self.indices.clone())
    }

    pub fn get_buffers(&self, context: &VulkanoContext) -> (Subbuffer<[T]>, Subbuffer<[Normal]>, Subbuffer<[u32]>) {
        (
            create_shader_data_buffer(self.vertices.clone(), &context, BufferType::Vertex),
            create_shader_data_buffer(self.normals.clone(), &context, BufferType::Normal),
            create_shader_data_buffer(self.indices.clone(), &context, BufferType::Index),
        )
    }

    // combines two meshes, recalculates normals if there is an incorrect number of normals compared to vertices
    pub fn add(&mut self, mut other: Mesh<T>) {
        let vert_offset = self.vertices.len();
        self.vertices.append(&mut other.vertices);
        self.normals.append(&mut other.normals);
        for index in other.indices.iter_mut() {
            *index += vert_offset as u32;
        }
        self.indices.append(&mut other.indices);

        if self.vertices.len() != self.normals.len() {
            self.recalculate_normals();
        }
    }
}


// loads an obj file, currently only reads vertices, not materials
pub fn load_obj(path: &str) -> Vec<Mesh<PositionVertex>> {

    if !path.ends_with(".obj") {
        panic!("Cannot read files that are not obj");
    }

    let file = File::open(path).expect(&format!("Could not find file, \"{}\"", path));
    let buf = BufReader::new(file);

    let mut result: Vec<Mesh<PositionVertex>> = Vec::new();

    let mut temp_vertices: Vec<PositionVertex> = Vec::new();
    let mut temp_normals: Vec<Normal> = Vec::new();
    let mut vertex_indices: Vec<usize> = Vec::new();
    let mut normal_indices: Vec<usize> = Vec::new();
    let mut vertex_index_offset = 0;
    let mut normal_index_offset = 0;
    let mut flat_shaded: bool = true;

    for line in buf.lines() {
        if line.is_err() {break;}
        let line = line.unwrap();

        if line.starts_with("# ") {continue;}

        else if line.starts_with("o ") {
            if vertex_indices.len() != normal_indices.len() || vertex_indices.len() % 3 != 0 {
                panic!("Size Error");
            }
            if temp_vertices.len() == 0 {continue;}
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            for i in (0..vertex_indices.len()).step_by(3) {
                vertices.push(temp_vertices[vertex_indices[i + 0]]);
                vertices.push(temp_vertices[vertex_indices[i + 1]]);
                vertices.push(temp_vertices[vertex_indices[i + 2]]);

                normals.push(temp_normals[normal_indices[i + 0]]);
                normals.push(temp_normals[normal_indices[i + 1]]);
                normals.push(temp_normals[normal_indices[i + 2]]);
            }
            let indices: Vec<u32> = (0..vertices.len() as u32).collect();
            let mut mesh = Mesh::new(vertices, indices);
            mesh.set_normals(normals);
            if !flat_shaded { 
                result.push(mesh.smooth_shaded());
            } else {
                result.push(mesh);
            }

            vertex_index_offset += temp_vertices.len();
            normal_index_offset += temp_normals.len();
            temp_vertices = Vec::new();
            temp_normals = Vec::new();
            vertex_indices = Vec::new();
            normal_indices = Vec::new();
        }

        else if line.starts_with("v ") {
            let split: Vec<&str> = line.split(" ").collect();
            temp_vertices.push(PositionVertex {position: [split[1].parse::<f32>().unwrap(), split[2].parse::<f32>().unwrap(), split[3].parse::<f32>().unwrap()]});
        }

        else if line.starts_with("vn ") {
            let split: Vec<&str> = line.split(" ").collect();
            let normal = Vector3::new(split[1].parse::<f32>().unwrap(), split[2].parse::<f32>().unwrap(), split[3].parse::<f32>().unwrap());
            temp_normals.push(Normal {normal: normal.normalised().into()});
        }

        else if line.starts_with("f ") {
            let split: Vec<&str> = line.split(" ").collect();
            for i in 1..=3 {
                let subsplit: Vec<&str> = split[i].split("/").collect();
                vertex_indices.push(subsplit[0].parse::<usize>().unwrap() - 1 - vertex_index_offset);
                normal_indices.push(subsplit[2].parse::<usize>().unwrap() - 1 - normal_index_offset);
            }
        }

        else if line.starts_with("s ") {
            let split: Vec<&str> = line.split(" ").collect();
            flat_shaded = split[1] == "off";
        }
    }

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    // println!("{:?}", temp_vertices);
    // println!("{:?}", vertex_indices);
    // println!("{:?}", temp_normals);
    // println!("{:?}", normal_indices);
    // println!("{:?}", vertex_indices.len() / 3);
    for i in (0..vertex_indices.len()).step_by(3) {
        vertices.push(temp_vertices[vertex_indices[i + 0]]);
        vertices.push(temp_vertices[vertex_indices[i + 1]]);
        vertices.push(temp_vertices[vertex_indices[i + 2]]);

        normals.push(temp_normals[normal_indices[i + 2]]);
        normals.push(temp_normals[normal_indices[i + 0]]);
        normals.push(temp_normals[normal_indices[i + 1]]);
    }
    let indices: Vec<u32> = (0..vertices.len() as u32).collect();
    let mut mesh = Mesh::new(vertices, indices);
    mesh.set_normals(normals);
    if !flat_shaded {
        result.push(mesh.smooth_shaded());
    } else {
        result.push(mesh);
    }

    // println!("{:?}", result[0]);

    result
}


/// flat shades the components of a Mesh without ever needing a Mesh, 
/// functionally equivalent to calling flat_shaded() and then into()
pub fn flat_shade_components(in_verts: Vec<PositionVertex>, in_inds: Vec<u32>) -> (Vec<PositionVertex>, Vec<Normal>, Vec<u32>){
    let mut new_verts: Vec<PositionVertex> = Vec::new();
    let mut new_normals: Vec<Normal> = Vec::new();

    for i in (0..in_inds.len()).step_by(3) {
        let v_one: Vector3 = in_verts[in_inds[i as usize + 0] as usize].position.into();
        let v_two: Vector3 = in_verts[in_inds[i as usize + 1] as usize].position.into();
        let v_thr: Vector3 = in_verts[in_inds[i as usize + 2] as usize].position.into();

        let normal = (v_one - v_thr).cross(v_two - v_thr);

        new_verts.push(PositionVertex::from(v_one));
        new_verts.push(PositionVertex::from(v_two));
        new_verts.push(PositionVertex::from(v_thr));
        new_normals.push(Normal::from(normal));
        new_normals.push(Normal::from(normal));
        new_normals.push(Normal::from(normal));
    }

    let indices = (0..(new_verts.len()) as u32).collect_vec();
    (new_verts, new_normals, indices)
}

pub fn combine_meshes<T: Position + Clone + Copy + BufferContents>(meshes: &Vec<Mesh<T>>) -> Mesh<T> {
    let mut total_mesh = meshes[0].clone();
    for i in 1..meshes.len() {
        total_mesh.add(meshes[i].clone());
    };
    total_mesh
}