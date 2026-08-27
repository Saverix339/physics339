use glium::{Surface, Vertex, glutin::{api::egl::display, surface::WindowSurface}, winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow}, window::Window}};
use Vertex;
struct App {
    window: Window,
    display: glium::Display<WindowSurface>,
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

fn triangle() -> Vec<Vertex>{

}

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().expect("eventloop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let mut app = App {window, display};
    
    event_loop.run_app(&mut app).expect("run_app");
}

