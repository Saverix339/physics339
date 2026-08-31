use nalgebra::{Matrix3, Vector3};


use crate::vertex_struct::Vert;
pub struct Mesh {
    pub verts: Vec<Vert>,
    pub indices: Vec<u16>,
}

pub fn cube(h: f32, color: [f32; 4]) -> Mesh {
    let v = |x , y, z| Vert {position: [x * h, y * h, z * h,], color};
    Mesh { verts: vec![
        v(-1.,-1.,-1.), v( 1.,-1.,-1.), v( 1., 1.,-1.), v(-1., 1.,-1.),
        v(-1.,-1., 1.), v( 1.,-1., 1.), v( 1., 1., 1.), v(-1., 1., 1.),], indices:
        vec![
            0,1,2, 0,2,3,  4,6,5, 4,7,6,  0,4,5, 0,5,1,
            3,2,6, 3,6,7,  0,3,7, 0,7,4,  1,5,6, 1,6,2,
        ] }
}


struct Ray3 {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

struct RigidBody{
    mass: f32,
    loc_inverse_inertia: Matrix3<f32>,
    glo_inverse_inertia: Matrix3<f32>,

    loc_centroid: Vector3<f32>,
    glo_centroid: Vector3<f32>,
 
    position: Vector3<f32>,
    orientation: Matrix3<f32>,
    linear_velocity: Vector3<f32>,
    angular_velocity: Vector3<f32>,

    force_accumulator: Vector3<f32>,
    torque_accumulator: Vector3<f32>,
}

impl RigidBody{
    fn update_globalcentroid(&mut self){
        self.glo_centroid = self.orientation * self.loc_centroid + self.position;
    }

    fn update_globalposition(&mut self){
        self.position = self.orientation * (-self.loc_centroid) + self.glo_centroid;
    }
}