use gl::types::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct VertexUVNormal {
    position: [GLfloat; 3],
    texture_uv: [GLfloat; 2],
    normal: [GLfloat; 3],
}
impl VertexUVNormal {
    pub fn new(position: [GLfloat; 3], texture_uv: [GLfloat; 2], normal: [GLfloat; 3]) -> Self {
        Self {
            position,
            texture_uv,
            normal,
        }
    }

    pub fn vertex_specification(vao: GLuint, vbo: GLuint) {
        // See
        // https://docs.google.com/presentation/d/13t-x_HWZOip8GWLAdlZu6_jV-VnIb0-FQBTVnLIsRSw/edit#slide=id.g75eed9a1c_0_67
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

            // layout (location = 2) in vec3 in_normal;
            let offset = (5 * std::mem::size_of::<GLfloat>()) as GLuint;
            let location = 2;
            gl::EnableVertexArrayAttrib(vao, location);
            gl::VertexArrayAttribFormat(vao, location, 3, gl::FLOAT, gl::FALSE, offset);
            gl::VertexArrayAttribBinding(vao, location, 0);
        }
    }
}

pub fn cube() -> Vec<VertexUVNormal> {
    return vec![
        // Back face
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 0.0], [0.0, 0.0, -1.0]), // Bottom-left
        VertexUVNormal::new([0.5, 0.5, -0.5], [1.0, 1.0], [0.0, 0.0, -1.0]),   // top-right
        VertexUVNormal::new([0.5, -0.5, -0.5], [1.0, 0.0], [0.0, 0.0, -1.0]),  // bottom-right
        VertexUVNormal::new([0.5, 0.5, -0.5], [1.0, 1.0], [0.0, 0.0, -1.0]),   // top-right
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 0.0], [0.0, 0.0, -1.0]), // bottom-left
        VertexUVNormal::new([-0.5, 0.5, -0.5], [0.0, 1.0], [0.0, 0.0, -1.0]),  // top-left
        // Front face
        VertexUVNormal::new([-0.5, -0.5, 0.5], [0.0, 0.0], [0.0, 0.0, 1.0]), // bottom-left
        VertexUVNormal::new([0.5, -0.5, 0.5], [1.0, 0.0], [0.0, 0.0, 1.0]),  // bottom-right
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 1.0], [0.0, 0.0, 1.0]),   // top-right
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 1.0], [0.0, 0.0, 1.0]),   // top-right
        VertexUVNormal::new([-0.5, 0.5, 0.5], [0.0, 1.0], [0.0, 0.0, 1.0]),  // top-left
        VertexUVNormal::new([-0.5, -0.5, 0.5], [0.0, 0.0], [0.0, 0.0, 1.0]), // bottom-left
        // Left face
        VertexUVNormal::new([-0.5, 0.5, 0.5], [1.0, 0.0], [-1.0, 0.0, 0.0]), // top-right
        VertexUVNormal::new([-0.5, 0.5, -0.5], [1.0, 1.0], [-1.0, 0.0, 0.0]), // top-left
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 1.0], [-1.0, 0.0, 0.0]), // bottom-left
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 1.0], [-1.0, 0.0, 0.0]), // bottom-left
        VertexUVNormal::new([-0.5, -0.5, 0.5], [0.0, 0.0], [-1.0, 0.0, 0.0]), // bottom-right
        VertexUVNormal::new([-0.5, 0.5, 0.5], [1.0, 0.0], [-1.0, 0.0, 0.0]), // top-right
        // Right face
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 0.0], [1.0, 0.0, 0.0]), // top-left
        VertexUVNormal::new([0.5, -0.5, -0.5], [0.0, 1.0], [1.0, 0.0, 0.0]), // bottom-right
        VertexUVNormal::new([0.5, 0.5, -0.5], [1.0, 1.0], [1.0, 0.0, 0.0]), // top-right
        VertexUVNormal::new([0.5, -0.5, -0.5], [0.0, 1.0], [1.0, 0.0, 0.0]), // bottom-right
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 0.0], [1.0, 0.0, 0.0]), // top-left
        VertexUVNormal::new([0.5, -0.5, 0.5], [0.0, 0.0], [1.0, 0.0, 0.0]), // bottom-left
        // Bottom face
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 1.0], [0.0, -1.0, 0.0]), // top-right
        VertexUVNormal::new([0.5, -0.5, -0.5], [1.0, 1.0], [0.0, -1.0, 0.0]),  // top-left
        VertexUVNormal::new([0.5, -0.5, 0.5], [1.0, 0.0], [0.0, -1.0, 0.0]),   // bottom-left
        VertexUVNormal::new([0.5, -0.5, 0.5], [1.0, 0.0], [0.0, -1.0, 0.0]),   // bottom-left
        VertexUVNormal::new([-0.5, -0.5, 0.5], [0.0, 0.0], [0.0, -1.0, 0.0]),  // bottom-right
        VertexUVNormal::new([-0.5, -0.5, -0.5], [0.0, 1.0], [0.0, -1.0, 0.0]), // top-right
        // Top face
        VertexUVNormal::new([-0.5, 0.5, -0.5], [0.0, 1.0], [0.0, 1.0, 0.0]), // top-left
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 0.0], [0.0, 1.0, 0.0]),   // bottom-right
        VertexUVNormal::new([0.5, 0.5, -0.5], [1.0, 1.0], [0.0, 1.0, 0.0]),  // top-right
        VertexUVNormal::new([0.5, 0.5, 0.5], [1.0, 0.0], [0.0, 1.0, 0.0]),   // bottom-right
        VertexUVNormal::new([-0.5, 0.5, -0.5], [0.0, 1.0], [0.0, 1.0, 0.0]), // top-left
        VertexUVNormal::new([-0.5, 0.5, 0.5], [0.0, 0.0], [0.0, 1.0, 0.0]),  // bottom-left
    ];
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    position: [GLfloat; 3],
}
impl Vertex {
    pub fn new(position: [GLfloat; 3]) -> Self {
        Self { position }
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
        }
    }
}

pub fn skybox_cube() -> Vec<Vertex> {
    // The vertices in 1 -> 3 -> 2 rather than 1 -> 2 -> 3, and the normals are
    // inverted. We always are inside the skybox cube so the backface culling
    // would delete everything if we kept the same order as a regular cube
    return vec![
        // Front face (+X)
        Vertex::new([0.5, 0.5, 0.5]),
        Vertex::new([0.5, 0.5, -0.5]),
        Vertex::new([0.5, -0.5, -0.5]),
        Vertex::new([0.5, -0.5, -0.5]),
        Vertex::new([0.5, -0.5, 0.5]),
        Vertex::new([0.5, 0.5, 0.5]),
        // Back face (-X)
        Vertex::new([-0.5, 0.5, 0.5]),
        Vertex::new([-0.5, -0.5, -0.5]),
        Vertex::new([-0.5, 0.5, -0.5]),
        Vertex::new([-0.5, -0.5, -0.5]),
        Vertex::new([-0.5, 0.5, 0.5]),
        Vertex::new([-0.5, -0.5, 0.5]),
        // Left (+Y)
        Vertex::new([-0.5, 0.5, -0.5]),
        Vertex::new([0.5, 0.5, -0.5]),
        Vertex::new([0.5, 0.5, 0.5]),
        Vertex::new([0.5, 0.5, 0.5]),
        Vertex::new([-0.5, 0.5, 0.5]),
        Vertex::new([-0.5, 0.5, -0.5]),
        // Right (-Y)
        Vertex::new([-0.5, -0.5, -0.5]),
        Vertex::new([0.5, -0.5, 0.5]),
        Vertex::new([0.5, -0.5, -0.5]),
        Vertex::new([0.5, -0.5, 0.5]),
        Vertex::new([-0.5, -0.5, -0.5]),
        Vertex::new([-0.5, -0.5, 0.5]),
        // Top (+Z)
        Vertex::new([-0.5, -0.5, 0.5]),
        Vertex::new([0.5, 0.5, 0.5]),
        Vertex::new([0.5, -0.5, 0.5]),
        Vertex::new([0.5, 0.5, 0.5]),
        Vertex::new([-0.5, -0.5, 0.5]),
        Vertex::new([-0.5, 0.5, 0.5]),
        // Bottom (-Z)
        Vertex::new([-0.5, -0.5, -0.5]),
        Vertex::new([0.5, -0.5, -0.5]),
        Vertex::new([0.5, 0.5, -0.5]),
        Vertex::new([0.5, 0.5, -0.5]),
        Vertex::new([-0.5, 0.5, -0.5]),
        Vertex::new([-0.5, -0.5, -0.5]),
    ];
}
