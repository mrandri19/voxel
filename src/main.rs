use gl::types::*;

extern crate nalgebra_glm as glm;

use glfw::{Action, Context, Key};

mod chunk;
mod debug_message_callback;
mod program;
mod shader;
mod vertex;

use chunk::{Chunk, CHUNK_BLOCKS, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, COBBLESTONE};
use program::Program;
use shader::Shader;
use vertex::{cube, Vertex};

use std::ffi::CString;

const MOUSE_SENSITIVITY: f32 = 0.1;
const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1920;

const NEAR_DISTANCE: f32 = 0.1;
const FAR_DISTANCE: f32 = 200.;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * glm::pi::<f32>() / 180.
}

fn draw(
    camera_ray: &glm::Vec3,
    camera_pos: &glm::Vec3,
    width: f64,
    height: f64,
    vao: GLuint,
    cube_vertices_vbo: GLuint,
    texture: GLuint,
    drawing_program: &Program,
    chunk: &Chunk,
) {
    let up: glm::TVec3<GLfloat> = glm::vec3(0., 0., 1.);
    // ************************************************************************
    // Clear screen and depth buffer

    unsafe {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    // ************************************************************************
    // Create model, view, projection matrices

    let model: glm::Mat4 = glm::identity();
    let model = glm::translate(&model, &glm::vec3(0., 0., 0.));

    let view: glm::Mat4 = glm::look_at(
        &camera_pos,                // eye: position of the camera
        &(camera_pos + camera_ray), // center: position where the camera is looking at
        &up,                        // up: normalized up vector
    );

    let fov = glm::half_pi();
    let aspect_ratio = (width / height) as f32;
    let projection: glm::Mat4 = glm::perspective(aspect_ratio, fov, NEAR_DISTANCE, FAR_DISTANCE);

    let mvp = projection * view * model;

    // Pass the matrices as uniforms
    let model_uniform_location = 0;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            drawing_program.get_id(),
            model_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&mvp).as_ptr(),
        )
    };

    // ************************************************************************
    // Bind textures

    let texture_unit = 0;
    unsafe { gl::BindTextureUnit(texture_unit, texture) };
    unsafe { gl::ProgramUniform1i(drawing_program.get_id(), 3, texture_unit as i32) };

    // ************************************************************************
    // Add cube vertices to their vbo and vao descriptor

    let cube_vertices = cube();
    unsafe {
        gl::NamedBufferSubData(
            cube_vertices_vbo,
            0,
            (cube_vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            cube_vertices.as_ptr() as *const GLvoid,
        )
    };

    Vertex::vertex_specification(vao, cube_vertices_vbo);

    // ************************************************************************
    // Use raycasting to figure out which cubes to display

    let mut offsets: Vec<[GLfloat; 3]> = Vec::with_capacity(CHUNK_BLOCKS);

    let mut block_used: [bool; CHUNK_BLOCKS] = [false; CHUNK_BLOCKS];

    let far_height = 2. * ((fov) / 2.).tan() * FAR_DISTANCE;
    let far_width = aspect_ratio * far_height;

    let right = glm::cross(&camera_ray, &up).normalize();
    let camera_up = glm::cross(&right, &camera_ray).normalize();
    let fc = camera_pos + camera_ray * FAR_DISTANCE;
    let fbl = fc - (camera_up * far_height / 2.) - (right * far_width / 2.);

    let max_u = 320;
    let max_v = 180;
    for v in 0..max_v {
        for u in 0..max_u {
            let du = u as GLfloat / max_u as GLfloat;
            let dv = v as GLfloat / max_v as GLfloat;
            // initialization step

            // ray starting position
            let ray_start = camera_pos;
            // ray direction
            let ray_direction =
                (fbl + camera_up * far_height * dv + right * far_width * du) - ray_start;
            let ray_direction = ray_direction.normalize();
            // voxel on which the ray origin is found
            let mut ray_voxel = glm::floor(&camera_pos);
            // how much to increment as we cross voxel boundaries
            let step = glm::sign(&ray_direction);
            // the value of t at which the ray crosses the fist vertical voxel boundary
            let mut t_max = ((ray_voxel + step) - ray_start)
                .component_div(&ray_direction.add_scalar(0.0000001));
            // how far along the ray we must move for each component of such movement to equal the width of a voxel
            let t_delta = (glm::vec3(1., 1., 1.)
                .component_div(&ray_direction.add_scalar(0.0000001)))
            .component_mul(&step);

            // traversal step
            for _ in 0..(FAR_DISTANCE * (3.0f32).sqrt() + 1.) as u32 {
                if t_max.x < t_max.y {
                    if t_max.x < t_max.z {
                        ray_voxel.x += step.x;
                        t_max.x += t_delta.x;
                    } else {
                        ray_voxel.z += step.z;
                        t_max.z += t_delta.z;
                    }
                } else {
                    if t_max.y < t_max.z {
                        ray_voxel.y += step.y;
                        t_max.y += t_delta.y;
                    } else {
                        ray_voxel.z += step.z;
                        t_max.z += t_delta.z;
                    }
                }

                if ray_voxel.x >= (CHUNK_X_SIZE as f32) {
                    break;
                }
                if ray_voxel.y >= (CHUNK_Y_SIZE as f32) {
                    break;
                }
                if ray_voxel.z >= (CHUNK_Z_SIZE as f32) {
                    break;
                }
                if ray_voxel.x < 0. {
                    break;
                }
                if ray_voxel.y < 0. {
                    break;
                }
                if ray_voxel.z < 0. {
                    break;
                }

                let x_ = ray_voxel.x as GLuint;
                let y_ = ray_voxel.y as GLuint;
                let z_ = ray_voxel.z as GLuint;
                let x = ray_voxel.x;
                let y = ray_voxel.y;
                let z = ray_voxel.z;

                if chunk.get(x_, y_, z_) == COBBLESTONE
                    && !block_used
                        [(z_ * CHUNK_Y_SIZE * CHUNK_X_SIZE + y_ * CHUNK_X_SIZE + x_) as usize]
                {
                    block_used
                        [(z_ * CHUNK_Y_SIZE * CHUNK_X_SIZE + y_ * CHUNK_X_SIZE + x_) as usize] =
                        true;
                    offsets.push([x, y, z]);
                    break;
                }
            }
        }
    }

    // ************************************************************************
    // Create cubes offsets buffer and bind them to be used for instanced drawing
    let mut offsets_buffer = 0;

    unsafe {
        gl::CreateBuffers(1, &mut offsets_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, offsets_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (offsets.len() * std::mem::size_of::<[GLfloat; 3]>()) as GLsizeiptr,
            offsets.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );

        let location = 2;
        // layout (location = 2) in vec4 view_offset;
        gl::EnableVertexArrayAttrib(vao, location);
        gl::VertexAttribPointer(
            location,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as GLsizei,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::VertexAttribDivisor(location, 1);
    }

    // ************************************************************************
    // Draw Instanced

    drawing_program.use_();

    unsafe {
        gl::DrawArraysInstanced(
            gl::TRIANGLES,
            0,
            cube_vertices.len() as GLsizei,
            offsets.len() as GLsizei,
        );

        gl::DeleteBuffers(1, &mut offsets_buffer);
    }
}

fn main() {
    let up = glm::vec3(0., 0., 1.);

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(true));
    let (mut window, events) = glfw
        .with_primary_monitor(|g, m| {
            g.create_window(
                INITIAL_WIDTH,
                INITIAL_HEIGHT,
                "Hello this is window",
                m.map_or(glfw::WindowMode::Windowed, |m| {
                    glfw::WindowMode::FullScreen(m)
                }),
            )
        })
        .expect("Failed to create GLFW window.");
    window.set_all_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(debug_message_callback::callback), std::ptr::null())
    }

    let chunk = Chunk::new();

    // Enable depth testing
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { gl::DepthFunc(gl::LESS) };

    // Enable back face culling
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::FrontFace(gl::CCW) };
    unsafe { gl::CullFace(gl::BACK) };

    // Create vbo for a single cube's vertices
    let mut cube_vertices_vbo = 0;
    unsafe {
        gl::CreateBuffers(1, &mut cube_vertices_vbo);
        gl::NamedBufferData(
            cube_vertices_vbo,
            (cube().len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        )
    };

    // Create Vertex Array Object
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };
    unsafe { gl::BindVertexArray(vao) };

    // Create and use shader program

    let drawing_program = {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.vert"
            )))
            .unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();
        let fragment_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.frag"
            )))
            .unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
        program::Program::new(vec![
            (vertex_shader, gl::VERTEX_SHADER),
            (fragment_shader, gl::FRAGMENT_SHADER),
        ])
        .unwrap()
    };

    // Load texture
    let texture_image = image::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/textures/mossy_cobblestone.png"
    ))
    .unwrap()
    .to_rgb();
    let texture_width = texture_image.width();
    let texture_height = texture_image.height();

    // Create texture
    let mut texture = 0;
    unsafe {
        gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TextureStorage2D(
            texture,
            4,
            gl::RGB8,
            texture_width as GLsizei,
            texture_height as GLsizei,
        );
        gl::TextureSubImage2D(
            texture,
            0,
            0,
            0,
            texture_width as GLsizei,
            texture_height as GLsizei,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture_image.into_raw().as_ptr() as *const GLvoid,
        );

        gl::GenerateTextureMipmap(texture);
        gl::TextureParameteri(texture, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint);
        gl::TextureParameteri(texture, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint);
        gl::TextureParameteri(
            texture,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_NEAREST as GLint,
        );
        gl::TextureParameteri(texture, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
    };

    let mut last_camera_pos = glm::vec3(3., 3., 15.);
    let mut last_camera_ray = glm::vec3(0., 1., 0.);
    let mut last_yaw = 0.;
    let mut last_pitch = 0.;
    let mut last_x = 0.;
    let mut last_y = 0.;
    let mut last_width = INITIAL_WIDTH;
    let mut last_height = INITIAL_HEIGHT;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Size(width, height) => {
                    last_width = width as u32;
                    last_height = height as u32;
                    unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei) };
                }
                glfw::WindowEvent::Close => window.set_should_close(true),
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = x as f32;
                    let y = y as f32;

                    let xoffset = (x - last_x) * MOUSE_SENSITIVITY;
                    let yoffset = (y - last_y) * MOUSE_SENSITIVITY;

                    last_yaw += xoffset;
                    last_pitch -= yoffset;

                    if last_pitch > 89.9 {
                        last_pitch = 89.9;
                    }
                    if last_pitch < -89.9 {
                        last_pitch = -89.9;
                    }

                    last_x = x;
                    last_y = y;

                    let camera_ray = glm::vec3(
                        degrees_to_radians(last_pitch).cos() * degrees_to_radians(last_yaw).sin(),
                        degrees_to_radians(last_pitch).cos() * degrees_to_radians(last_yaw).cos(),
                        degrees_to_radians(last_pitch).sin(),
                    );
                    let camera_ray = glm::normalize(&camera_ray);
                    *last_camera_ray = *camera_ray;
                }
                glfw::WindowEvent::Key(key, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    match key {
                        Key::W => {
                            let mut direction = last_camera_ray;
                            direction.z = 0.;
                            let direction = glm::normalize(&direction);
                            last_camera_pos += direction;
                        }
                        Key::A => {
                            let mut right = glm::cross(&last_camera_ray, &up);
                            right.z = 0.;
                            let direction = glm::normalize(&right);
                            last_camera_pos -= direction;
                        }
                        Key::S => {
                            let mut direction = last_camera_ray;
                            direction.z = 0.;
                            let direction = glm::normalize(&direction);
                            last_camera_pos -= direction;
                        }
                        Key::D => {
                            let mut right = glm::cross(&last_camera_ray, &up);
                            right.z = 0.;
                            let direction = glm::normalize(&right);
                            last_camera_pos += direction;
                        }
                        Key::Space => {
                            last_camera_pos += up;
                        }
                        Key::C => {
                            last_camera_pos -= up;
                        }
                        Key::Escape => window.set_should_close(true),
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        unsafe {
            let mut query = 0;
            gl::GenQueries(1, &mut query);
            gl::BeginQuery(gl::TIME_ELAPSED, query);

            use std::time::Instant;
            let now = Instant::now();

            draw(
                &last_camera_ray,
                &last_camera_pos,
                last_width as f64,
                last_height as f64,
                vao,
                cube_vertices_vbo,
                texture,
                &drawing_program,
                &chunk,
            );

            let cpu_time = now.elapsed().as_micros() as f32 / 1000.;

            gl::EndQuery(gl::TIME_ELAPSED);
            let mut gpu_time = 0;
            gl::GetQueryObjectiv(query, gl::QUERY_RESULT, &mut gpu_time);
            gl::DeleteQueries(1, &query as *const _);

            println!(
                "CPU: {:.2} ms, GPU: {:.2} ms",
                cpu_time,
                gpu_time as f64 / 1_000_000.0
            );
        }

        window.swap_buffers()
    }
}
