use gl::types::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    position: [GLfloat; 3],
    texture_uv: [GLfloat; 2],
}
impl Vertex {
    pub fn new(position: [GLfloat; 3], texture_uv: [GLfloat; 2]) -> Self {
        Self {
            position,
            texture_uv,
        }
    }

    pub fn vertex_specification(vao: GLuint, vbo: GLuint) {
        unsafe {
            // Bind vao and vbo together
            gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, std::mem::size_of::<Self>() as GLint);

            // layout (location = 0) in vec3 in_position;
            let offset = 0;
            let location = 0;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 3, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);

            // layout (location = 1) in vec2 in_texture_uv;
            let offset = (3 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 1;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 2, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);
        }
    }
}

pub fn cube() -> Vec<Vertex> {
    return vec![
        // Back face
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]), // Bottom-left
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),   // top-right
        Vertex::new([0.5, -0.5, -0.5], [1.0, 0.0]),  // bottom-right
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),   // top-right
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]), // bottom-left
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),  // top-left
        // Front face
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]), // bottom-left
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),  // bottom-right
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),   // top-right
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),   // top-right
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0]),  // top-left
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]), // bottom-left
        // Left face
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]), // top-right
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0]), // top-left
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // bottom-left
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // bottom-left
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]), // bottom-right
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]), // top-right
        // Right face
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]), // top-left
        Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]), // bottom-right
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]), // top-right
        Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]), // bottom-right
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]), // top-left
        Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0]), // bottom-left
        // Bottom face
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // top-right
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0]),  // top-left
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),   // bottom-left
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),   // bottom-left
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),  // bottom-right
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]), // top-right
        // Top face
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]), // top-left
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),   // bottom-right
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),  // top-right
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),   // bottom-right
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]), // top-left
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0]),  // bottom-left
    ];
}
