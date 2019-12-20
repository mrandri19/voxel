extern crate nalgebra_glm as glm;

use gl::types::*;

mod debug_message_callback;
mod program;
mod shader;
mod vertex;

use vertex::Vertex;

fn cube() -> Vec<Vertex> {
    return vec![
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]),
        Vertex::new([0.5, -0.5, -0.5], [1.0, 0.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]),
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0]),
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0]),
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
    ];
}

const CHUNK_X_BLOCKS: usize = 64;
const CHUNK_Y_BLOCKS: usize = 64;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * glm::pi::<f32>() / 180.
}

fn main() {
    use glutin::event::{Event, WindowEvent};
    use glutin::event_loop::{ControlFlow, EventLoop};
    use glutin::window::WindowBuilder;
    use glutin::ContextBuilder;

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
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

    windowed_context
        .window()
        .set_fullscreen(Some(glutin::window::Fullscreen::Exclusive(video_mode)));

    windowed_context.window().set_cursor_visible(false);
    windowed_context
        .window()
        .set_cursor_position(glutin::dpi::LogicalPosition::new(960., 540.))
        .unwrap();
    windowed_context.window().set_cursor_grab(true).unwrap();

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(debug_message_callback::callback), std::ptr::null())
    }

    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let mut vbo = 0;
    unsafe { gl::CreateBuffers(1, &mut vbo) };
    let mut vao = 0;
    unsafe { gl::CreateVertexArrays(1, &mut vao) };
    unsafe { gl::BindVertexArray(vao) };

    let program = program::Program::new().unwrap();
    program.use_();

    let texture_image = image::open("textures/mossy_cobblestone.png").unwrap();
    let image = texture_image.to_rgb();
    let width = image.width();
    let height = image.height();

    let mut texture = 0;
    unsafe {
        gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TextureStorage2D(texture, 4, gl::RGB8, width as GLsizei, height as GLsizei);
        gl::TextureSubImage2D(
            texture,
            0,
            0,
            0,
            width as GLsizei,
            height as GLsizei,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            image.into_raw().as_ptr() as *const GLvoid,
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

    let mut camera_pos = glm::vec3(0., -5., 2.);

    let mut last_x = 960.;
    let mut last_y = 540.;

    let mut yaw = 0.;
    let mut pitch = 0.;

    let mut camera_front_saved = glm::vec3(0., 1., 0.);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                windowed_context.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
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
                camera_front_saved = camera_front;

                let view: glm::Mat4 = glm::look_at(
                    &camera_pos,                  // eye: position of the camera
                    &(camera_pos + camera_front), // center: position where the camera is looking at
                    &glm::vec3(0., 0., 1.),       // up: normalized up vector
                );

                let (width, height): (f64, f64) = windowed_context
                    .window()
                    .inner_size()
                    .to_physical(windowed_context.window().hidpi_factor())
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
                vertices.reserve(cube().len() * CHUNK_X_BLOCKS * CHUNK_Y_BLOCKS);
                offsets.reserve(CHUNK_X_BLOCKS * CHUNK_Y_BLOCKS);
                for y in 0..CHUNK_Y_BLOCKS {
                    for x in 0..CHUNK_X_BLOCKS {
                        offsets.push(glm::vec3(x as f32 - 8., y as f32 - 8., 0.));
                        vertices.extend(cube());
                    }
                }

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
                    gl::GenBuffers(1, &mut instance_vbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (offsets.len() * std::mem::size_of::<glm::Vec3>()) as GLsizeiptr,
                        offsets.as_ptr() as *const GLvoid,
                        gl::STATIC_DRAW,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    // layout (location = 2) in vec3 view_offset;
                    gl::EnableVertexAttribArray(2);
                    gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
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
                        (CHUNK_X_BLOCKS * CHUNK_Y_BLOCKS) as GLsizei,
                    );
                }
                windowed_context.swap_buffers().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(logical_size),
                ..
            } => {
                let (width, height): (u32, u32) = logical_size
                    .to_physical(windowed_context.window().hidpi_factor())
                    .into();
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
                    position.to_physical(windowed_context.window().hidpi_factor());

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
                    let mut direction = camera_front_saved;
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos + direction;
                }
                glutin::event::VirtualKeyCode::A => {
                    let mut direction = glm::cross(&camera_front_saved, &glm::vec3(0., 0., 1.));
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::S => {
                    let mut direction = camera_front_saved;
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos - direction;
                }
                glutin::event::VirtualKeyCode::D => {
                    let mut direction = glm::cross(&camera_front_saved, &glm::vec3(0., 0., 1.));
                    direction.z = 0.;
                    let direction = glm::normalize(&direction);
                    camera_pos = camera_pos + direction;
                }
                glutin::event::VirtualKeyCode::Space => {
                    camera_pos = camera_pos + glm::vec3(0., 0., 1.);
                }
                glutin::event::VirtualKeyCode::LControl => {
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
