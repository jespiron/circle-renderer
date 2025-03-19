mod circle;
mod renderer;
use circle::generate_circles;
use minifb::{Window, WindowOptions};
use renderer::render_sequential::render_sequential;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const RENDER_MODE: RenderMode = RenderMode::Sequential;

enum RenderMode {
    Sequential,
    PerCircleLocks,
    PerCircleAtomics,
    PerCircleMessagePassing,
    PerSubimage,
}

fn main() {
    let width = 800;
    let height = 600;
    let mut circles = generate_circles(50);

    let mut window =
        Window::new("Circle Renderer", width, height, WindowOptions::default()).unwrap();

    let dt = 0.1;
    let mut frame = 0;
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let buffer = match RENDER_MODE {
            // One thread per circle
            RenderMode::Sequential => render_sequential(&mut circles, width, height, dt),
            _ => vec![],
        };

        window.update_with_buffer(&buffer, width, height).unwrap();
        //std::thread::sleep(Duration::from_millis(5));
        frame += 1;
    }
}
