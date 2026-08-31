use glium::implement_vertex;

pub const  VERTEX_SHADER_GLSL: &str = r#"#version 140

    in vec3 position;
    in vec4 color;      
    out vec3 vertex_color;
    out float alpha;

    uniform mat4 mvp;

    //uniform float x;
    //uniform float y;

    void main() {
        vertex_color = color.rgb;
        //vec3 pos = position;
        //pos.x += x;
        //pos.y += y;
        gl_Position = mvp * vec4(position, 1.0);
    }
    "#;
pub const  FRAGMENT_SHADER_GLSL: &str = r#"#version 140

    in vec3 vertex_color;
    in float alpha;
    out vec4 color;

    void main() {
        color = vec4(vertex_color, alpha);
    }
    "#;

#[derive(Copy, Clone)]
pub struct Vert {
    pub position: [f32; 3],
    pub color: [f32; 4]
}
implement_vertex!(Vert, position, color);

