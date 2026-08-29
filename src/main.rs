use glium::{Program, Surface, glutin::surface::WindowSurface, index::NoIndices, winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow}, window::Window}};
mod vertex_struct;
use vertex_struct::{Vert, VERTEX_SHADER_GLSL, FRAGMENT_SHADER_GLSL};



struct App {
    window: Window,
    display: glium::Display<WindowSurface>,
    vertex_buffer: glium::VertexBuffer<Vert>,
    indices: NoIndices,
    program: Program,
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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let mut frame = self.display.draw();
                frame.clear_color(0.0, 1.0, 0.0, 1.0);

                frame.draw(&self.vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms,&Default::default()).unwrap();

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
        Vert { position: [0.0 + pos[0], 0.5 + pos[1]]},
        Vert { position: [0.5 + pos[0], -0.5 + pos[1]]},
        Vert { position: [-0.5 + pos[0], -0.5 + pos[1]]},
    ];
    shape
}

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().expect("eventloop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle_at([0.0, 0.0])).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, VERTEX_SHADER_GLSL, FRAGMENT_SHADER_GLSL, None).unwrap();

    
    
    let mut app = App {window, display, vertex_buffer, indices, program};

    
    
    event_loop.run_app(&mut app).expect("run_app");
}

