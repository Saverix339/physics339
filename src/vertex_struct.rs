use glium::implement_vertex;

pub const  VERTEX_SHADER_GLSL: &str = r#"#version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;
pub const  FRAGMENT_SHADER_GLSL: &str = r#"#version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

#[derive(Copy, Clone)]
pub struct Vert {
    pub position: [f32; 2],
}
implement_vertex!(Vert, position);

