use std::{time::Instant, vec};

use glium::{DrawParameters, IndexBuffer, Program, Surface, backend::glutin::SimpleWindowBuilder, glutin::{config::ConfigTemplateBuilder, surface::WindowSurface}, uniform, winit::{application::ApplicationHandler, event::WindowEvent, 
        event_loop::{ActiveEventLoop, ControlFlow}, window::Window
    }
};
use nalgebra::{Matrix4, Perspective3, Translation3, Rotation3, Vector3};

mod vertex_struct;
mod rigidbody;
use vertex_struct::{Vert, VERTEX_SHADER_GLSL, FRAGMENT_SHADER_GLSL};
use rigidbody::cube;

use crate::rigidbody::{Mesh, GpuMesh};

#[derive(Default)]
pub struct Scene {
    meshes: Vec<Mesh>,
    points: Vec<Vert>,

    gpu_meshes: Vec<GpuMesh>,
}

struct App {
    window: Window,
    display: glium::Display<WindowSurface>,
    vertex_buffer: glium::VertexBuffer<Vert>,
    indices: IndexBuffer<u16>,
    program: Program,
    draw_parameters: DrawParameters<'static>,
    t: Instant, //time passed

    scene: Scene,
}

impl App{
    
}

fn compute_mvp(w_ratio: f32, t: f32) -> [[f32; 4]; 4]{
    let proj: Matrix4<f32> = Perspective3::new(w_ratio, 50.0_f32.to_radians(), 0.1, 100.0).to_homogeneous();
    let view = Translation3::new(0.0, 0.0, -3.0).to_homogeneous();
    let model = Rotation3::from_axis_angle(&Vector3::y_axis(), t*2.0_f32).to_homogeneous();
    (proj * view * model).data.0
}



impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(), //closing window function
            WindowEvent::Resized(window_size) => { //resizing 
                self.display.resize(window_size.into());
            },
            WindowEvent::RedrawRequested => { //called each frame
                /* let x_offs = self.t.sin() * 0.3;
                let y_offs = self.t.cos() * 0.3; */
                let elapsed = self.t.elapsed().as_secs_f32(); //tempo totale trascorso dall'avvio
                let mut frame = self.display.draw();
                frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                let (w, h): (f32, f32) = self.window.inner_size().into();
                let aspect= if h > 0.0 { w as f32 / h as f32 } else { return; };
                /* frame.draw(
                    &self.vertex_buffer, 
                    &self.indices, 
                    &self.program, 
                    &uniform!{ mvp: compute_mvp(aspect, elapsed) },
                    &self.draw_parameters
                ).unwrap(); */
                self.scene.gpu_meshes.iter().for_each(|m| {
                    m.draw(&mut frame, &self.program, &self.draw_parameters, compute_mvp(aspect, elapsed));
                });
                frame.finish().expect("frame");
            },
            _ => (),
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // self.step_physics();
        self.window.request_redraw();
    }

    
}



fn triangle_at(pos: [f32; 2]) -> Vec<Vert>{
    let shape = vec![
        Vert { position: [0.0 + pos[0], 0.5 + pos[1], 0.0], color: [1.0, 0.0, 0.0, 1.0]},
        Vert { position: [0.5 + pos[0], -0.5 + pos[1], 0.0], color: [0.0, 1.0, 0.0, 1.0]},
        Vert { position: [-0.5 + pos[0], -0.5 + pos[1], 0.0], color: [0.0, 0.0, 1.0, 1.0]},
    ];
    shape
}

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().expect("eventloop");
    let (window, display) = SimpleWindowBuilder::new().with_config_template_builder(ConfigTemplateBuilder::new().with_alpha_size(0)).build(&event_loop);
    let mut scene = Scene {
        meshes : Vec::new(),
        points : Vec::new(),
        gpu_meshes : Vec::new(),
    };
    let cube1 = cube(1.0, [1.0, 0.5, 0.0, 1.0]);
    let cube2 = cube1.clone();
    scene.meshes.push(&cube1);

    scene.meshes.push(&cube2);
    scene.gpu_meshes = scene.meshes.iter().map(|m| GpuMesh::load(&display, m)).collect();

    let gpu_buffer: Vec<Vert> = scene.gpu_meshes.iter().flat_map(|m| m.get_vertex_buffer().read().unwrap().to_vec()).collect::<Vec<_>>();
    let vertex_buffer = glium::VertexBuffer::dynamic(&display, &gpu_buffer).unwrap();
    let gpu_indices: Vec<u16> = scene.gpu_meshes.iter().flat_map(|m| m.get_indices().read().unwrap().to_vec()).collect::<Vec<_>>();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &gpu_indices).unwrap();
    let draw_parameters = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        ..Default::default()
    },
    ..Default::default()
    };
    let program = glium::Program::from_source(&display, VERTEX_SHADER_GLSL, FRAGMENT_SHADER_GLSL, None).unwrap();
    let t = Instant::now();

    let mut app = App {window, display, vertex_buffer, indices, program, draw_parameters, t, scene: scene};
    
    event_loop.run_app(&mut app).expect("run_app");
}

