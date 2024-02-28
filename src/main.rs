extern crate glium;

use glium::{
    glutin::{self, dpi::PhysicalSize},
    Surface,
};
use std::time::Instant;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

glium::implement_vertex!(Vertex, position, uv);

fn make_plane() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [-1.0, -1.0],
        uv: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        position: [-1.0, 1.0],
        uv: [-1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [1.0, 1.0],
        uv: [1.0, 1.0],
    };
    let vertex4 = Vertex {
        position: [1.0, -1.0],
        uv: [1.0, -1.0],
    };

    vec![vertex1, vertex2, vertex3, vertex3, vertex4, vertex1]
}

fn main() {
    // Инициализация контекста и создание окна
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Simple Shader Viewer");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let plane = make_plane();

    let vertex_buffer = glium::VertexBuffer::new(&display, &plane).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 uv;

        out vec2 v_uv;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_uv = uv;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        uniform float t;
        uniform vec2 m;
        in vec2 v_uv;
        out vec4 color;
        void main() {
            vec2 uv = v_uv;
            float b = abs(sin(t/180.0));
            float l = length(uv - m);
            l = smoothstep(0.1, 0.2, fract(l+t/10.0));
            color = vec4(l, l, b, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;
    let mut last_frame_time = Instant::now(); // Время начала предыдущего кадра
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut phys_size = display.gl_window().window().inner_size();

    // Главный цикл отображения
    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let now = Instant::now();
        let elapsed = now.duration_since(last_frame_time); // Время, прошедшее с предыдущего кадра
        last_frame_time = now; // Обновление времени для следующего кадра

        let delta = elapsed.as_secs_f32();

        t += delta;

        let uniforms = glium::uniform! {
            t : t,
            m : [x, y]
        };
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    ()
                }
                glutin::event::WindowEvent::Resized(physical_size) => {
                    target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target.finish().unwrap();
                    display.gl_window().resize(physical_size);
                    println!("resize!");
                    println!("{:?}", physical_size);
                    phys_size = physical_size;
                    ()
                }
                glutin::event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    modifiers,
                } => {
                    println!("CursorMoved");
                    println!("{:?}", position);
                    x = position.x as f32 / phys_size.width as f32;
                    y = position.y as f32 / phys_size.height as f32;
                    ()
                }
                _ => (),
            },
            _ => (),
        }
    });
}
