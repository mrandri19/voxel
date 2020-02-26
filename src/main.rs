use gl::types::*;

extern crate nalgebra_glm as glm;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

mod chunk;
mod debug_message_callback;
mod program;
mod shader;
mod vertex;

use chunk::{Chunk, COBBLESTONE};
use program::Program;
use shader::Shader;
use vertex::Vertex;

use std::ffi::CString;

fn cube() -> Vec<Vertex> {
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

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * glm::pi::<f32>() / 180.
}

fn make_gl_ctx_and_event_loop() -> (ContextWrapper<PossiblyCurrent, Window>, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    // Select the first video mode
    let mut video_mode = windowed_context
        .window()
        .current_monitor()
        .video_modes()
        .next()
        .unwrap();
    for vm in windowed_context.window().current_monitor().video_modes() {
        if vm.size().width > video_mode.size().width
            && vm.refresh_rate() > video_mode.refresh_rate()
        {
            video_mode = vm;
        }
    }

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // windowed_context
    //     .window()
    //     .set_fullscreen(Some(glutin::window::Fullscreen::Exclusive(video_mode)));

    // windowed_context.window().set_cursor_visible(false);
    // windowed_context
    //     .window()
    //     .set_cursor_position(glutin::dpi::LogicalPosition::new(960., 540.))
    //     .unwrap();
    // windowed_context.window().set_cursor_grab(true).unwrap();

    return (windowed_context, event_loop);
}

fn init_opengl(ctx: &ContextWrapper<PossiblyCurrent, Window>) {
    gl::load_with(|s| ctx.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(debug_message_callback::callback), std::ptr::null())
    }
}

fn draw(
    pitch: f32,
    yaw: f32,
    last_camera_front: &mut glm::Vec3,
    camera_pos: glm::Vec3,
    ctx: &ContextWrapper<PossiblyCurrent, Window>,
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

    let camera_ray = glm::vec3(
        degrees_to_radians(pitch).cos() * degrees_to_radians(yaw).sin(),
        degrees_to_radians(pitch).cos() * degrees_to_radians(yaw).cos(),
        degrees_to_radians(pitch).sin(),
    );
    let camera_ray = glm::normalize(&camera_ray);
    *last_camera_front = camera_ray;

    let view: glm::Mat4 = glm::look_at(
        &camera_pos,                // eye: position of the camera
        &(camera_pos + camera_ray), // center: position where the camera is looking at
        &up,                        // up: normalized up vector
    );

    let (width, height): (f64, f64) = ctx
        .window()
        .inner_size()
        .to_physical(ctx.window().hidpi_factor())
        .into();
    let far_distance = 100.;
    let fov = glm::half_pi();
    let aspect_ratio = (width / height) as f32;
    let projection: glm::Mat4 = glm::perspective(aspect_ratio, fov, 0.1, far_distance);

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

    let mut offsets: Vec<[GLfloat; 4]> = vec![];

    let far_height = 1.3 * 2. * (fov / 2.).tan() * far_distance;
    let far_width = aspect_ratio * far_height;

    let right = glm::cross(&camera_ray, &up).normalize();
    let fc = camera_pos + camera_ray * far_distance;
    let fbl = fc - (up * far_height / 2.) - (right * far_width / 2.);

    let max_u = 71;
    let max_v = 40;
    for v in 0..max_v {
        for u in 0..max_u {
            let du = u as GLfloat / max_u as GLfloat;
            let dv = v as GLfloat / max_v as GLfloat;
            // initialization step

            // ray starting position
            let ray_start = camera_pos;
            // ray direction
            let ray_direction = fbl + up * far_height * dv + right * far_width * du;
            let ray_direction = ray_direction.normalize();
            // voxel on which the ray origin is found
            let mut ray_voxel = glm::trunc(&camera_pos);
            // how much to increment as we cross voxel boundaries
            let step = glm::sign(&ray_direction);
            // the value of t at which the ray crosses the fist vertical voxel boundary
            let mut t_max = ((ray_voxel + step) - ray_start)
                .component_div(&ray_direction.add_scalar(0.0000001));
            // how far along the ray we must move for each component of such movement to equal the width of a voxel
            let t_delta = glm::vec3(1., 1., 1.)
                .component_div(&ray_direction.component_mul(&step.add_scalar(0.0000001)));

            // traversal step
            loop {
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

                if ray_voxel.x >= chunk.x_blocks() as f32 {
                    break;
                }
                if ray_voxel.y >= chunk.y_blocks() as f32 {
                    break;
                }
                if ray_voxel.z >= chunk.z_blocks() as f32 {
                    break;
                }

                if ray_voxel.x < 0. || ray_voxel.y < 0. || ray_voxel.z < 0. {
                    continue;
                }

                let x = ray_voxel.x as GLuint;
                let y = ray_voxel.y as GLuint;
                let z = ray_voxel.z as GLuint;

                if chunk.get(x, y, z) == COBBLESTONE {
                    offsets.push([x as GLfloat, y as GLfloat, z as GLfloat, 1.]);
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
            (offsets.len() * std::mem::size_of::<[GLfloat; 4]>()) as GLsizeiptr,
            offsets.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );

        let location = 2;
        // layout (location = 2) in vec4 view_offset;
        gl::EnableVertexArrayAttrib(vao, location);
        gl::VertexAttribPointer(
            location,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * std::mem::size_of::<f32>()) as GLsizei,
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

    let (ctx, event_loop) = make_gl_ctx_and_event_loop();
    init_opengl(&ctx);

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

    // Initial camera position
    let mut camera_pos = glm::vec3(-1., -1., 0.);

    // Initial camera orientation
    let mut last_camera_front = glm::vec3(0., 1., 0.);

    // Initial yaw and pitch
    let mut yaw = 0.;
    let mut pitch = 0.;

    // Initial mouse position
    let (width, height): (f64, f64) = ctx
        .window()
        .inner_size()
        .to_physical(ctx.window().hidpi_factor())
        .into();
    let mut last_x = (width / 2.) as f32;
    let mut last_y = (height / 2.) as f32;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                ctx.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                unsafe {
                    let mut query = 0;
                    gl::GenQueries(1, &mut query);
                    gl::BeginQuery(gl::TIME_ELAPSED, query);
                    use std::time::Instant;
                    let now = Instant::now();
                    draw(
                        pitch,
                        yaw,
                        &mut last_camera_front,
                        camera_pos,
                        &ctx,
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
                ctx.swap_buffers().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(logical_size),
                ..
            } => {
                let (width, height): (u32, u32) =
                    logical_size.to_physical(ctx.window().hidpi_factor()).into();
                unsafe { gl::Viewport(0, 0, width as GLsizei, height as GLsizei) };
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let glutin::dpi::PhysicalPosition { x, y } =
                    position.to_physical(ctx.window().hidpi_factor());

                let mouse_sensitivity = 0.1;
                let xoffset = (x as f32 - last_x) * mouse_sensitivity;
                let yoffset = (y as f32 - last_y) * mouse_sensitivity;

                yaw += xoffset;
                pitch -= yoffset;

                // TODO(andrea): figure a way to move the mouse past the window's boundary
                // otherwise we cannot go turn over +- 90 degrees with a 1920 screen and 0.1 sens

                // dbg!(x, last_x);
                // dbg!(y, last_y);
                // dbg!(yaw, pitch);

                last_x = x as f32;
                last_y = y as f32;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::event::KeyboardInput {
                                state: glutin::event::ElementState::Pressed,
                                virtual_keycode: Some(virtual_keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => match virtual_keycode {
                glutin::event::VirtualKeyCode::W => {
                    let mut direction = last_camera_front;
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos + direction;
                }
                glutin::event::VirtualKeyCode::A => {
                    let mut right = glm::cross(&last_camera_front, &up);
                    right.z = 0.;
                    let direction = glm::normalize(&right);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::S => {
                    let mut direction = last_camera_front;
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::D => {
                    let mut right = glm::cross(&last_camera_front, &up);
                    right.z = 0.;
                    let direction = glm::normalize(&right);
                    camera_pos = camera_pos + direction;
                }
                glutin::event::VirtualKeyCode::Space => {
                    camera_pos = camera_pos + up;
                }
                glutin::event::VirtualKeyCode::C => {
                    camera_pos = camera_pos + glm::vec3(0., 0., -1.);
                }
                glutin::event::VirtualKeyCode::Escape => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => *control_flow = ControlFlow::Poll,
        }
    })
}
