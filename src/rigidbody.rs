use glium::{IndexBuffer, backend::{Facade, glutin}, glutin::surface::WindowSurface, program::Uniform, Surface, uniform};
use nalgebra::{Matrix3, Vector3};
use std::sync::atomic::{AtomicUsize, Ordering};


use crate::vertex_struct::Vert;

type Display = glium::Display<WindowSurface>;


static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn get_next_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct Mesh {
    pub id: u32,

    pub verts: Vec<Vert>,
    pub indices: Vec<u16>,

    pub position: Vector3<f32>,
    pub rotation: Matrix3<f32>,

}
impl Mesh {
    fn translate(&mut self, direction: [f32; 3]){
        let tr = Vector3::from(direction);
        /* self.verts.iter_mut().for_each(|v| {
            v.position = (Vector3::from(v.position) + tr).into();
        }); */
    }
}

pub struct GpuMesh { 
    vertex_buffer: glium::VertexBuffer<Vert>,
    indices: IndexBuffer<u16>
}

impl GpuMesh {
    pub fn load(display: &Display, mesh: &Mesh) -> Self {
        //let context = display.get_context().clone();
        let vertex_buffer = glium::VertexBuffer::new(display, &mesh.verts)
        .expect("Vertex Buffer Load fail.");
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &mesh.indices,
        ).expect("Index Buffer Load fail.");
        Self {
            vertex_buffer,
            indices
        }
    }
    pub fn draw(&self, frame: &mut glium::Frame, program: &glium::Program, params : &glium::DrawParameters<'static>, mvp: [[f32; 4]; 4]) {
        frame.draw(
            &self.vertex_buffer,
            &self.indices,
            program,
            &uniform!(mvp: mvp),
            params,
        ).expect("Draw fail.");
    }
    pub fn get_vertex_buffer(&self) -> &glium::VertexBuffer<Vert> {
        &self.vertex_buffer
    }
    pub fn get_indices(&self) -> &IndexBuffer<u16> {
        &self.indices
    }
}

pub fn cube(h: f32, color: [f32; 4]) -> Mesh {
    let v = |x , y, z| Vert {position: [x * h, y * h, z * h,], color};
    let id = get_next_id();
    Mesh { verts: vec![
        v(-1.,-1.,-1.), v( 1.,-1.,-1.), v( 1., 1.,-1.), v(-1., 1.,-1.),
        v(-1.,-1., 1.), v( 1.,-1., 1.), v( 1., 1., 1.), v(-1., 1., 1.),], indices:
        vec![
            0,1,2, 0,2,3,  4,6,5, 4,7,6,  0,4,5, 0,5,1,
            3,2,6, 3,6,7,  0,3,7, 0,7,4,  1,5,6, 1,6,2,
        ],
        id: id as u32,
        position: Vector3::new(0.0, 0.0, 0.0), rotation: Matrix3::identity() 
    }
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