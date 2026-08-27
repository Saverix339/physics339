#[Derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(glium::Vertex, position);