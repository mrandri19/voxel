// TODO(Andrea):
// - Skybox far plane clipping
// - Cazzo di compute shader

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
use texture::{Texture2D, TextureCubeMap};
use vertex::{cube, skybox_cube, Vertex, VertexUVNormal};

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

    skybox_vao: GLuint,
    skybox_bo: GLuint,

    mossy_cobblestone_texture: &Texture2D,

    sky_cubemap_texture: &TextureCubeMap,

    textured_phong_cube_program: &Program,
    skybox_program: &Program,

    chunk: &Chunk,
    time: f64,
) {
    // ************************************************************************
    // Clear screen and depth buffer
    unsafe {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    // *************************************************************************
    // Create model, view, projection matrices

    let model: glm::Mat4 = glm::identity();

    let view: glm::Mat4 = glm::look_at(
        &camera_pos,                // eye: position of the camera
        &(camera_pos + camera_ray), // center: position where the camera is looking at
        &up,                        // up: normalized up vector
    );

    let fov = glm::half_pi();
    let aspect_ratio = (width / height) as f32;
    let projection: glm::Mat4 = glm::perspective(aspect_ratio, fov, NEAR_DISTANCE, FAR_DISTANCE);

    let light_position = glm::vec3(
        20.0 + 10.0 * (2.0 * 3.14 * 5e-3 * time).cos() as f32,
        20.0 + 10.0 * (2.0 * 3.14 * 5e-3 * time).sin() as f32,
        20.0 + 1.0 * (2.0 * 3.14 * 1e-2 * time).cos() as f32,
    );

    // *************************************************************************
    // Use raycasting to figure out which cubes to display
    let mut offsets = raycast(aspect_ratio, fov, camera_pos, camera_ray, &up, chunk);

    // *************************************************************************
    // Add an additional cube to draw the light
    // TODO(Andrea): use a different program to draw this
    offsets.push(light_position.into());

    // Skybox Program
    {
        unsafe { gl::DepthMask(gl::FALSE) };

        skybox_program.use_();

        let model = glm::translate(
            &glm::scale(
                &glm::identity(),
                &glm::vec3(FAR_DISTANCE * 1.0, FAR_DISTANCE * 1.0, FAR_DISTANCE * 1.0), // will be a 64x64x64 cube centered at (0,0,0)
            ),
            &glm::vec3(1.0 / 4., 1.0 / 4., 0.0),
        );

        skybox_program.set_uniform_mat4(0, &model);
        let view_no_translate = glm::mat3_to_mat4(&glm::mat4_to_mat3(&view));

        skybox_program.set_uniform_mat4(1, &view_no_translate);
        skybox_program.set_uniform_mat4(2, &projection);

        let sky_cubemap_texture_unit = 7;
        sky_cubemap_texture.bind(sky_cubemap_texture_unit);
        skybox_program.set_uniform_sampler(3, sky_cubemap_texture_unit);

        let cube = skybox_cube();
        unsafe {
            gl::NamedBufferSubData(
                skybox_bo,
                0,
                (cube.len() * std::mem::size_of::<VertexUVNormal>()) as GLsizeiptr,
                cube.as_ptr() as *const GLvoid,
            )
        };

        unsafe { gl::BindVertexArray(skybox_vao) };
        Vertex::vertex_specification(skybox_vao, skybox_bo);

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, cube.len() as GLsizei);
        };

        unsafe { gl::DepthMask(gl::TRUE) };
    }

    // Phong Cube Program
    {
        // *************************************************************************
        // Use the program
        textured_phong_cube_program.use_();

        // *************************************************************************
        // Pass the MVP matrices as uniforms to the shaders
        textured_phong_cube_program.set_uniform_mat4(0, &model);
        textured_phong_cube_program.set_uniform_mat4(1, &view);
        textured_phong_cube_program.set_uniform_mat4(2, &projection);

        // *************************************************************************
        // Bind textures and pass them to shaders

        let mossy_cobblestone_texture_unit = 12;
        mossy_cobblestone_texture.bind(mossy_cobblestone_texture_unit);
        textured_phong_cube_program.set_uniform_sampler(3, mossy_cobblestone_texture_unit);

        // ************************************************************************
        // Pass additional data to shaders

        textured_phong_cube_program.set_uniform_vec3(4, &camera_pos);
        textured_phong_cube_program.set_uniform_vec3(5, &light_position);

        // *************************************************************************
        // Add cube vertices to their vbo and vao descriptor, then bind the VAO and
        // set it up

        let cube = cube();
        unsafe {
            gl::NamedBufferSubData(
                cube_bo,
                0,
                (cube.len() * std::mem::size_of::<VertexUVNormal>()) as GLsizeiptr,
                cube.as_ptr() as *const GLvoid,
            )
        };

        unsafe { gl::BindVertexArray(cube_vao) };
        VertexUVNormal::vertex_specification(cube_vao, cube_bo);

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
}

fn main() {
    let chunk = Chunk::new();

    // *************************************************************************
    // Setup window

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

    // *************************************************************************
    // Setup OpenGL
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(debug_message_callback::callback), std::ptr::null())
    }

    // Enable depth testing
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { gl::DepthFunc(gl::LESS) };

    // Enable back face culling
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::FrontFace(gl::CCW) };
    unsafe { gl::CullFace(gl::BACK) };

    // *************************************************************************
    // Create VBOs for a single cube's vertices
    let mut cube_bo = 0;
    unsafe {
        gl::CreateBuffers(1, &mut cube_bo);
        gl::NamedBufferData(
            cube_bo,
            (cube().len() * std::mem::size_of::<VertexUVNormal>()) as GLsizeiptr,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        )
    };

    let mut skybox_bo = 0;
    unsafe {
        gl::CreateBuffers(1, &mut skybox_bo);
        gl::NamedBufferData(
            skybox_bo,
            (cube().len() * std::mem::size_of::<VertexUVNormal>()) as GLsizeiptr,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        )
    };

    // *************************************************************************
    // Create VAOs
    let mut cube_vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut cube_vao) };

    let mut skybox_vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut skybox_vao) };

    // *************************************************************************
    // Create and use shader program
    let textured_phong_cube_program = {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/cube/cube.vert.glsl"
            )))
            .unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();
        let fragment_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/cube/cube.frag.glsl"
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

    let skybox_program = {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/skybox/skybox.vert.glsl"
            )))
            .unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();
        let fragment_shader = Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/skybox/skybox.frag.glsl"
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

    // *************************************************************************
    // Create textures
    let mossy_cobblestone_texture = Texture2D::new(
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/mossy_cobblestone.png"
        ))
        .unwrap()
        .to_rgb(),
    );

    // Ignore the rotations and the names, like this is works and that's it
    let sky_cubemap_texture = TextureCubeMap::new([
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/front.jpg"
        ))
        .unwrap()
        .rotate270()
        .to_rgb(),
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/back.jpg"
        ))
        .unwrap()
        .rotate90()
        .to_rgb(),
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/right.jpg"
        ))
        .unwrap()
        .rotate180()
        .to_rgb(),
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/left.jpg"
        ))
        .unwrap()
        .to_rgb(),
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/top.jpg"
        ))
        .unwrap()
        .rotate270()
        .to_rgb(),
        image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/textures/skybox/bottom.jpg"
        ))
        .unwrap()
        .rotate270()
        .to_rgb(),
    ]);

    // *************************************************************************
    // Camera, event handling, and main loop

    let up = glm::vec3(0., 0., 1.);
    let mut last_camera_pos = glm::vec3(3., 3., 15.);
    let mut last_camera_ray = glm::vec3(0., 1., 0.);
    let mut last_yaw = 0.;
    let mut last_pitch = 0.;
    let mut last_x = 0.;
    let mut last_y = 0.;
    let mut last_width = INITIAL_WIDTH;
    let mut last_height = INITIAL_HEIGHT;

    let mut time = 0.0;

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
                skybox_vao,
                skybox_bo,
                &mossy_cobblestone_texture,
                &sky_cubemap_texture,
                &textured_phong_cube_program,
                &skybox_program,
                &chunk,
                time,
            );
        });

        time += 1.0;

        window.swap_buffers()
    }
}
