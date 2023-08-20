use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2::sys::SDL_Point;
use crate::{event_handling, rendering};

pub(crate) fn run_app_loop(canvas: &mut WindowCanvas, event_pump: &mut EventPump) {
    let mut ball_pos: (i32, i32) = (0, 0);
    let mut ball_speed: i32 = 0;
    let mut frame_count: u32 = 0;

    let mut is_laggy = false;
    let mut sine_points: Vec<SDL_Point> = Vec::new();

    'running: loop {
        rendering::render(&mut *canvas, &mut sine_points, &mut ball_pos, &mut ball_speed, frame_count, is_laggy);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => { event_handling::handle_event(event, &mut is_laggy) }
            }
        }

        frame_count += 1;
        // 60 fps
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}