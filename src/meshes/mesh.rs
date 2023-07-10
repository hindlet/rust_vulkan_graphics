//! generic code for meshes in 3D space
#![allow(dead_code)]
use itertools::Itertools;
use vulkano::buffer::Subbuffer;
use vulkano_util::context::VulkanoContext;
use super::*;
use maths::Vector3;




/// Data for a mesh in 3D space
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<PositionVertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}


impl Mesh {
    pub fn new(
        vertices: Vec<PositionVertex>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            vertices, normals: Vec::new(), indices
        }
    }

    pub const EMPTY: Mesh = Mesh {
        vertices: Vec::new(),
        normals: Vec::new(),
        indices: Vec::new()
    };

    // sets the normals of the given mesh and returns a reference
    pub fn set_normals(
        &mut self,
        normals: Vec<Normal>,
    ) -> &mut Mesh{
        self.normals = normals;
        self
    }

    // recalculates the normals of the given mesh, smooth shaded
    pub fn recalculate_normals(&mut self) -> &mut Mesh{
        // init normals
        let mut normals: Vec<Vector3> = Vec::new();
        for _i in 0..self.vertices.len() {
            normals.push([0.0, 0.0, 0.0].into())
        }
        
        // create normals
        for i in (0..self.indices.len()).step_by(3) {
            let dir_one: Vector3 = {
                let dir: Vector3 = self.vertices[self.indices[i] as usize].position.into();
                dir - self.vertices[self.indices[i + 2] as usize].position.into()
            };
            let dir_two: Vector3 = {
                let dir: Vector3 = self.vertices[self.indices[i + 1] as usize].position.into();
                dir - self.vertices[self.indices[i + 2] as usize].position.into()
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

    /// returns a flat shaded version of the mesh called on
    pub fn flat_shaded(&self) -> Mesh {
        let mut new_verts: Vec<PositionVertex> = Vec::new();
        let mut new_normals: Vec<Normal> = Vec::new();
    
        for i in (0..self.indices.len()).step_by(3) {
            let v_one: Vector3 = self.vertices[self.indices[i as usize + 0] as usize].position.into();
            let v_two: Vector3 = self.vertices[self.indices[i as usize + 1] as usize].position.into();
            let v_thr: Vector3 = self.vertices[self.indices[i as usize + 2] as usize].position.into();
    
            let normal = (v_one - v_thr).cross(v_two - v_thr).normalised();
    
            new_verts.push(PositionVertex::from(v_one));
            new_verts.push(PositionVertex::from(v_two));
            new_verts.push(PositionVertex::from(v_thr));
            new_normals.push(Normal::from(normal));
            new_normals.push(Normal::from(normal));
            new_normals.push(Normal::from(normal));
        }
    
        let indices = (0..(new_verts.len()) as u32).collect_vec();
        Mesh::new(new_verts, indices).set_normals(new_normals).clone()
    }

    /// sets the current mesh to be flat shaded
    /// 
    /// NOT CURRENTLY REVERSIBLE
    pub fn flat_shade(&mut self) {
        let new = self.flat_shaded();
        self.vertices = new.vertices;
        self.normals = new.normals;
        self.indices = new.indices;
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


    pub fn components(&self) -> (Vec<PositionVertex>, Vec<Normal>, Vec<u32>){
        (self.vertices.clone(), self.normals.clone(), self.indices.clone())
    }

    pub fn get_buffers(&self, context: VulkanoContext) -> (Subbuffer<[PositionVertex]>, Subbuffer<[Normal]>, Subbuffer<[u32]>) {
        (
            create_shader_data_buffer(self.vertices.clone(), &context, BufferType::Vertex),
            create_shader_data_buffer(self.normals.clone(), &context, BufferType::Normal),
            create_shader_data_buffer(self.indices.clone(), &context, BufferType::Index),
        )
    }
}