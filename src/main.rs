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
    offsets_program: &Program,
    drawing_program: &Program,
    chunk: &Chunk,
) {
    unsafe {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    let model: glm::Mat4 = glm::identity();
    let model = glm::translate(&model, &glm::vec3(0., 0., 0.));

    let camera_front = glm::vec3(
        degrees_to_radians(pitch).cos() * degrees_to_radians(yaw).sin(),
        degrees_to_radians(pitch).cos() * degrees_to_radians(yaw).cos(),
        degrees_to_radians(pitch).sin(),
    );
    let camera_front = glm::normalize(&camera_front);
    *last_camera_front = camera_front;

    let view: glm::Mat4 = glm::look_at(
        &camera_pos,                  // eye: position of the camera
        &(camera_pos + camera_front), // center: position where the camera is looking at
        &glm::vec3(0., 0., 1.),       // up: normalized up vector
    );

    let (width, height): (f64, f64) = ctx
        .window()
        .inner_size()
        .to_physical(ctx.window().hidpi_factor())
        .into();
    let projection: glm::Mat4 =
        glm::perspective((width / height) as f32, glm::half_pi(), 0.1, 100.);

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

    let texture_unit = 0;
    unsafe { gl::BindTextureUnit(texture_unit, texture) };
    unsafe { gl::ProgramUniform1i(drawing_program.get_id(), 3, texture_unit as i32) };

    let cube_vertices = cube();
    let mut cubes_offsets =
        Vec::with_capacity(chunk.x_blocks() * chunk.y_blocks() * chunk.z_blocks());

    for z in 0..chunk.z_blocks() {
        for y in 0..chunk.y_blocks() {
            for x in 0..chunk.x_blocks() {
                if chunk.get(x, y, z) == COBBLESTONE {
                    cubes_offsets.push([x as f32, y as f32, z as f32, 1.0]);
                }
            }
        }
    }

    let mut offsets_program_ssbo = 0;
    let index_binding_point = 0;
    unsafe {
        gl::GenBuffers(1, &mut offsets_program_ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, offsets_program_ssbo);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (cubes_offsets.len() * std::mem::size_of::<[GLfloat; 4]>()) as GLsizeiptr,
            cubes_offsets.as_ptr() as *const GLvoid,
            gl::DYNAMIC_READ,
        );

        gl::BindBufferBase(
            gl::SHADER_STORAGE_BUFFER,
            index_binding_point,
            offsets_program_ssbo,
        );

        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    }

    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, offsets_program_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLfloat;
    //     println!("before");
    //     dbg!(std::slice::from_raw_parts(ptr, 9));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }

    // offsets_program.use_();

    // unsafe {
    //     // Run compute shader
    //     gl::DispatchCompute(cubes_offsets.len() as GLuint, 1, 1);

    //     // Accesses to shader storage blocks after the barrier
    //     // will reflect writes prior to the barrier.
    //     // Wait for the shader to run and write to its storage
    //     gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
    // };

    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, offsets_program_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLfloat;
    //     println!("after");
    //     dbg!(std::slice::from_raw_parts(ptr, 9));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }

    unsafe {
        gl::NamedBufferSubData(
            cube_vertices_vbo,
            0,
            (cube_vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            cube_vertices.as_ptr() as *const GLvoid,
        )
    };

    Vertex::vertex_specification(vao, cube_vertices_vbo);

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, offsets_program_ssbo);
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
        // we call glVertexAttribDivisor. This function tells OpenGL when to update the
        // content of a vertex attribute to the next element. Its first parameter is the
        // vertex attribute in question and the second parameter the attribute divisor.
        // By default the attribute divisor is 0 which tells OpenGL to update the content
        // of the vertex attribute each iteration of the vertex shader. By setting this
        // attribute to 1 we're telling OpenGL that we want to update the content of the
        // vertex attribute when we start to render a new instance. By setting it to 2 we'd
        // update the content every 2 instances and so on. By setting the attribute divisor
        // to 1 we're effectively telling OpenGL that the vertex attribute at attribute location 2
        // is an instanced array.
        gl::VertexAttribDivisor(location, 1);
    }

    drawing_program.use_();

    // With vertex attributes, each run of the vertex shader will cause GLSL to
    // retrieve the next set of vertex attributes that belong to the current vertex.
    // When defining a vertex attribute as an instanced array however, the vertex
    // shader only updates the content of the vertex attribute per instance instead
    // of per vertex. This allows us to use the standard vertex attributes for data
    // per vertex and use the instanced array for storing data that is unique per instance.
    unsafe {
        gl::DrawArraysInstanced(
            gl::TRIANGLES,
            0,
            cube_vertices.len() as GLsizei,
            cubes_offsets.len() as GLsizei,
        );

        gl::DeleteBuffers(1, &mut offsets_program_ssbo);
    }
}

fn main() {
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

    let offsets_program = {
        let basic_compute_shader = shader::Shader::from_source(
            &CString::new(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/basic.comp"
            )))
            .unwrap(),
            gl::COMPUTE_SHADER,
        )
        .unwrap();

        program::Program::new(vec![(basic_compute_shader, gl::COMPUTE_SHADER)]).unwrap()
    };

    // Load texture
    let texture_image = image::open("textures/mossy_cobblestone.png")
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
    let mut camera_pos = glm::vec3(0., -5., 2.);

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
                    &offsets_program,
                    &drawing_program,
                    &chunk,
                );
                println!("{} ms", now.elapsed().as_micros() as f32 / 1000.);
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
                    let mut direction = glm::cross(&last_camera_front, &glm::vec3(0., 0., 1.));
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::S => {
                    let mut direction = last_camera_front;
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::D => {
                    let mut direction = glm::cross(&last_camera_front, &glm::vec3(0., 0., 1.));
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos + direction;
                }
                glutin::event::VirtualKeyCode::Space => {
                    camera_pos = camera_pos + glm::vec3(0., 0., 1.);
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
