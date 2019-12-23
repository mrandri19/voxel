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

use chunk::Chunk;
use vertex::Vertex;

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
    texture: GLuint,
    program: &program::Program,
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
            program.get_id(),
            model_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&model).as_ptr(),
        )
    };
    let view_uniform_location = 1;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            program.get_id(),
            view_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&view).as_ptr(),
        )
    };
    let projection_uniform_location = 2;
    unsafe {
        gl::ProgramUniformMatrix4fv(
            program.get_id(),
            projection_uniform_location,
            1,
            gl::FALSE,
            glm::value_ptr(&projection).as_ptr(),
        )
    };

    let texture_unit = 0;
    unsafe { gl::BindTextureUnit(texture_unit, texture) };
    unsafe { gl::ProgramUniform1i(program.get_id(), 3, texture_unit as i32) };

    let mut vertices = vec![];
    let mut offsets = vec![];
    vertices.reserve(cube().len() * chunk.x_blocks() * chunk.y_blocks() * chunk.z_blocks());
    offsets.reserve(chunk.x_blocks() * chunk.y_blocks() * chunk.z_blocks());

    for z in 0..chunk.z_blocks() {
        for y in 0..chunk.y_blocks() {
            for x in 0..chunk.x_blocks() {
                offsets.push(glm::vec3(x as f32, y as f32, z as f32));
                vertices.extend(cube());
            }
        }
    }

    // Create Vertex Buffer Object
    let mut vbo = 0;
    unsafe { gl::CreateBuffers(1, &mut vbo) };

    unsafe {
        gl::NamedBufferData(
            vbo,
            (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        )
    };

    Vertex::vertex_specification(vao, vbo);

    // TODO(andrea): use DSA api instead
    let mut instance_vbo = 0;
    unsafe {
        gl::CreateBuffers(1, &mut instance_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (offsets.len() * std::mem::size_of::<glm::Vec3>()) as GLsizeiptr,
            offsets.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // layout (location = 2) in vec3 view_offset;
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as GLsizei,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::VertexAttribDivisor(2, 1);
    }

    unsafe {
        gl::DrawArraysInstanced(
            gl::TRIANGLES,
            0,
            cube().len() as GLsizei,
            (chunk.x_blocks() * chunk.y_blocks() * chunk.z_blocks()) as GLsizei,
        );
    }

    unsafe { gl::DeleteBuffers(1, &instance_vbo) };
    unsafe { gl::DeleteBuffers(1, &vbo) };
}

fn main() {
    let (ctx, event_loop) = make_gl_ctx_and_event_loop();
    init_opengl(&ctx);

    // Enable depth testing
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { gl::DepthFunc(gl::LESS) };

    // Enable back face culling
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::FrontFace(gl::CCW) };
    unsafe { gl::CullFace(gl::BACK) };

    // Create Vertex Array Object
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };
    unsafe { gl::BindVertexArray(vao) };

    // Create and use shader program
    let program = program::Program::new().unwrap();
    program.use_();

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

    let chunk = Chunk::new();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                ctx.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                draw(
                    pitch,
                    yaw,
                    &mut last_camera_front,
                    camera_pos,
                    &ctx,
                    vao,
                    texture,
                    &program,
                    &chunk,
                );
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
