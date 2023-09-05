//! generic code for meshes in 3D space
#![allow(dead_code)]
use itertools::Itertools;
use vulkano::buffer::{Subbuffer, BufferContents};
use vulkano_util::context::VulkanoContext;
use super::*;
use maths::Vector3;
use std::collections::BTreeMap;


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
            let normal = dir_one.cross(dir_two);

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

/// flat shades the components of a Mesh without ever needing a Mesh
/// 
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