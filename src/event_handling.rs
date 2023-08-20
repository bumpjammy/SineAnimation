use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub(crate) fn handle_event(event: Event, is_laggy: &mut bool) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::L), .. } => {
            *is_laggy = !*is_laggy;
            if *is_laggy {
                println!("Laggy mode enabled");
            } else {
                println!("Laggy mode disabled");
            }
        },
        _ => {}
    }
}