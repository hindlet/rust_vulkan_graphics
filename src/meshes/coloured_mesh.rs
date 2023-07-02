//! generic code for meshes in 3D space
#![allow(dead_code)]
use itertools::Itertools;
use vulkano::buffer::Subbuffer;
use vulkano_util::context::VulkanoContext;
use super::*;
use maths::Vector3;




/// Data for a mesh in 3D space
#[derive(Debug, Clone)]
pub struct ColouredMesh {
    mesh: Mesh,
    pub colour: [f32; 4],
}


impl ColouredMesh {
    pub fn new(
        vertices: Vec<PositionVertex>,
        indices: Vec<u32>,
        colour: [f32; 4]
    ) -> Self {
        Self {
            mesh: Mesh::new(vertices, indices), colour
        }
    }

    pub const EMPTY: ColouredMesh = ColouredMesh {
        mesh: Mesh::EMPTY,
        colour: [1.0, 1.0, 1.0, 1.0]
    };

    // sets the normals of the given mesh and returns a reference
    pub fn set_normals(
        &mut self,
        normals: Vec<Normal>,
    ) -> &mut ColouredMesh{
        self.mesh.set_normals(normals);
        self
    }

    // recalculates the normals of the given mesh, smooth shaded
    pub fn recalculate_normals(&mut self) -> &mut ColouredMesh{
        self.mesh.recalculate_normals();
        self
    }

    /// returns a flat shaded version of the mesh called on
    pub fn flat_shaded(&self) -> ColouredMesh {
        let new_mesh = self.mesh.flat_shaded();

        ColouredMesh {mesh: new_mesh, colour: self.colour}
    }

    /// sets the current mesh to be flat shaded
    /// 
    /// NOT CURRENTLY REVERSIBLE
    pub fn flat_shade(&mut self) {
        self.mesh.flat_shade();
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
        self.mesh.components()
    }

    pub fn get_buffers(&self, context: VulkanoContext) -> (Subbuffer<[PositionVertex]>, Subbuffer<[Normal]>, Subbuffer<[u32]>) {
        self.mesh.get_buffers(context)
    }
}