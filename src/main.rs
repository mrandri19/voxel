use gl::types::*;

extern crate nalgebra_glm as glm;

use glfw::{Action, Context, Key};

mod chunk;
mod constants;
mod debug_message_callback;
mod measure_elapsed;
mod program;
mod raycasting;
mod shader;
mod texture;
mod vertex;

use chunk::Chunk;
use constants::*;
use measure_elapsed::measure_elapsed;
use program::Program;
use raycasting::raycast;
use shader::Shader;
use texture::Texture;
use vertex::{cube, Vertex};

use std::ffi::CString;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * glm::pi::<f32>() / 180.
}

fn draw(
    camera_ray: &glm::Vec3,
    camera_pos: &glm::Vec3,
    up: &glm::Vec3,
    width: f64,
    height: f64,
    cube_vao: GLuint,
    cube_bo: GLuint,
    texture: &Texture,
    drawing_program: &Program,
    chunk: &Chunk,
) {
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

    // Pass the matrices as uniforms
    let model_uniform_location = 0;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            drawing_program.get_id(),
            model_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&model).as_ptr(),
        )
    };

    let view_uniform_location = 1;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            drawing_program.get_id(),
            view_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&view).as_ptr(),
        )
    };

    let projection_uniform_location = 2;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            drawing_program.get_id(),
            projection_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&projection).as_ptr(),
        )
    };

    let camera_position_uniform_location = 4;
    unsafe {
        gl::ProgramUniform3fv(
            drawing_program.get_id(),
            camera_position_uniform_location,
            1,
            glm::value_ptr(&camera_pos).as_ptr(),
        )
    };

    // ************************************************************************
    // Bind textures

    let texture_uniform_location = 3;
    let texture_unit = 0;
    unsafe { gl::BindTextureUnit(texture_unit, texture.name()) };
    unsafe {
        gl::ProgramUniform1i(
            drawing_program.get_id(),
            texture_uniform_location,
            texture_unit as i32,
        )
    };

    // ************************************************************************
    // Add cube vertices to their vbo and vao descriptor

    let cube = cube();
    unsafe {
        gl::NamedBufferSubData(
            cube_bo,
            0,
            (cube.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            cube.as_ptr() as *const GLvoid,
        )
    };

    Vertex::vertex_specification(cube_vao, cube_bo);

    // ************************************************************************
    // Use raycasting to figure out which cubes to display
    let offsets = raycast(aspect_ratio, fov, camera_pos, camera_ray, &up, chunk);

    // ************************************************************************
    // Create cubes offsets buffer and bind them to be used for instanced drawing
    let mut offsets_bo = 0;

    unsafe {
        gl::CreateBuffers(1, &mut offsets_bo);
        gl::BindBuffer(gl::ARRAY_BUFFER, offsets_bo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (offsets.len() * std::mem::size_of::<[GLfloat; 3]>()) as GLsizeiptr,
            offsets.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );

        let location = 3;
        // layout (location = 3) in vec4 view_offset;
        gl::EnableVertexArrayAttrib(cube_vao, location);
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
            cube.len() as GLsizei,
            offsets.len() as GLsizei,
        );

        gl::DeleteBuffers(1, &mut offsets_bo);
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(true));
    let (mut window, events) = glfw
        .with_connected_monitors_mut(|g, _monitors| {
            g.create_window(
                INITIAL_WIDTH,
                INITIAL_HEIGHT,
                "Hello this is window",
                glfw::WindowMode::Windowed,
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
    let mut cube_bo = 0;
    unsafe {
        gl::CreateBuffers(1, &mut cube_bo);
        gl::NamedBufferData(
            cube_bo,
            (cube().len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        )
    };

    // Create Vertex Array Object
    let mut cube_vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut cube_vao) };
    unsafe { gl::BindVertexArray(cube_vao) };

    // Create and use shader program
    let drawing_program = {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.vert.glsl"
            )))
            .unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();
        let fragment_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.frag.glsl"
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

    // Create texture
    let mossy_cobblestone_texture = Texture::new(
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/mossy_cobblestone.png"
        ))
        .unwrap()
        .to_rgb(),
    );

    let up = glm::vec3(0., 0., 1.);
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

        measure_elapsed(|| {
            draw(
                &last_camera_ray,
                &last_camera_pos,
                &up,
                last_width as f64,
                last_height as f64,
                cube_vao,
                cube_bo,
                &mossy_cobblestone_texture,
                &drawing_program,
                &chunk,
            );
        });

        window.swap_buffers()
    }
}
