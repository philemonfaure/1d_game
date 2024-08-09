use std::f32::consts::PI;
use glium::{glutin, Surface, uniform};
use std::fs::read_to_string;
use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new().with_title("Shader Art");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = read_to_string(Path::new("src/shaders/vertex.glsl")).expect("Failed to read vertex shader");
    let fragment_shader_src = read_to_string(Path::new("src/shaders/fragment.glsl")).expect("Failed to read fragment shader");

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [ 1.0, -1.0] },
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
    ]).unwrap();

    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip, &[0u16, 1, 2, 3]).unwrap();

    let mut camera_position: [f32; 2] = [0.0, -2.0];
    let mut camera_orientation: f32 = 0.0;
    let mouse_sensitivity: f32 = 0.005;
    let mut movement = [false, false, false, false];
    let speed = 0.01;

    let target_fps: u32 = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);

    let start_time = Instant::now();
    let mut last_frame_time = Instant::now();

    display.gl_window().window().set_cursor_visible(false);
    display.gl_window().window().set_cursor_grab(true).expect("Failed to grab cursor");

    event_loop.run(move |event, _, control_flow| {

        let now = Instant::now();
        let elapsed = now - last_frame_time;

        if elapsed >= frame_duration {
            last_frame_time = now;

            let (width, height) = display.get_framebuffer_dimensions();

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target.draw(&vertex_buffer, &index_buffer, &program, &uniform! {
            time: start_time.elapsed().as_secs_f32(),
            resolution: [width as f32, height as f32],
            camera_position: camera_position,
            camera_orientation: camera_orientation
        }, &Default::default()).unwrap();
            target.finish().unwrap();
        }

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(last_frame_time + frame_duration);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x as f32;
                    let (width, height) = display.get_framebuffer_dimensions();
                    let (center_x, center_y) = (width as f32 / 2.0, height as f32 / 2.0);

                    let x_offset = x - center_x;

                    camera_orientation += x_offset * mouse_sensitivity;

                    display.gl_window().window().set_cursor_position(glutin::dpi::PhysicalPosition::new(center_x as f64, center_y as f64)).expect("Failed to set cursor position");
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glutin::event::VirtualKeyCode::Z) = input.virtual_keycode {
                        if input.state == glutin::event::ElementState::Pressed {
                            movement[0] = true;
                        } else if input.state == glutin::event::ElementState::Released {
                            movement[0] = false;
                        }
                    }
                    else if let Some(glutin::event::VirtualKeyCode::S) = input.virtual_keycode {
                        if input.state == glutin::event::ElementState::Pressed {
                            movement[1] = true;
                        } else if input.state == glutin::event::ElementState::Released {
                            movement[1] = false;
                        }
                    }
                    else if let Some(glutin::event::VirtualKeyCode::Q) = input.virtual_keycode {
                        if input.state == glutin::event::ElementState::Pressed {
                            movement[2] = true;
                        } else if input.state == glutin::event::ElementState::Released {
                            movement[2] = false;
                        }
                    }
                    else if let Some(glutin::event::VirtualKeyCode::D) = input.virtual_keycode {
                        if input.state == glutin::event::ElementState::Pressed {
                            movement[3] = true;
                        } else if input.state == glutin::event::ElementState::Released {
                            movement[3] = false;
                        }
                    }
                },
                _ => (),
            },
            _ => (),
        }

        if movement[0] == true
        {
            camera_position[0] += speed * (camera_orientation+PI/2.0).cos();
            camera_position[1] += speed * (camera_orientation+PI/2.0).sin();
        }
        if movement[1] == true
        {
            camera_position[0] -= speed * (camera_orientation+PI/2.0).cos();
            camera_position[1] -= speed * (camera_orientation+PI/2.0).sin();
        }
        if movement[2] == true
        {
            camera_position[0] -= speed * (camera_orientation).cos();
            camera_position[1] -= speed * (camera_orientation).sin();
        }
        if movement[3] == true
        {
            camera_position[0] += speed * (camera_orientation).cos();
            camera_position[1] += speed * (camera_orientation).sin();
        }
    });
}
