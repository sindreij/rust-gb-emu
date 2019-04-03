use std::{borrow::Cow, sync::Arc, thread};

use glium::{glutin, implement_vertex, Surface};
use parking_lot::Mutex;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn start_thread() -> (std::thread::JoinHandle<()>, Arc<Mutex<Vec<u8>>>) {
    let buffer = Arc::new(Mutex::new(vec![0; 160 * 144 * 3]));
    let handle = thread::spawn({
        let buffer = buffer.clone();
        move || {
            run(buffer);
        }
    });

    (handle, buffer)
}

pub fn run(buffer: Arc<Mutex<Vec<u8>>>) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let image: Vec<u8> = vec![
        0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0,
        0, 0, 0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255,
        255, 0, 0, 0,
    ];

    // let image = glium::texture::RawImage2d::from_raw_rgb(image, (4, 4));
    //let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut closed = false;
    while !closed {
        let texture = {
            let buffer = buffer.lock();
            let image = glium::texture::RawImage2d {
                data: Cow::Borrowed(&buffer),
                width: 160,
                height: 144,
                format: glium::texture::ClientFormat::U8U8U8,
            };
            glium::texture::Texture2d::new(&display, image).unwrap()
        };

        let target = display.draw();

        let (target_w, target_h) = target.get_dimensions();

        texture.as_surface().blit_whole_color_to(
            &target,
            &glium::BlitTarget {
                left: 0,
                bottom: target_h,
                width: target_w as i32,
                height: -(target_h as i32),
            },
            glium::uniforms::MagnifySamplerFilter::Nearest,
        );

        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });

        thread::sleep_ms(30);
    }
}
