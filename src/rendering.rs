use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::sys::SDL_Point;
use crate::{HEIGHT, WIDTH};

pub(crate) fn render(canvas: &mut WindowCanvas, sine_points: &mut Vec<SDL_Point>, ball_pos: &mut (i32, i32), ball_speed: &mut i32, frame_count: u32, is_laggy: bool) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    render_ball(canvas, ball_pos, ball_speed, frame_count, is_laggy);
    add_sine_point(sine_points, ball_pos);
    render_sine_points(canvas, sine_points);
    canvas.present();
}

fn render_ball(canvas: &mut WindowCanvas, ball_pos: &mut (i32, i32), ball_speed: &mut i32, frame_count: u32, is_laggy: bool) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    update_speed(ball_pos, ball_speed, frame_count, is_laggy);
    get_new_ball_pos(ball_pos, ball_speed);
    canvas.fill_rect(sdl2::rect::Rect::new(ball_pos.0, ball_pos.1 + (HEIGHT * 10 - 50) as i32, 100, 100)).unwrap();
}

fn update_speed(ball_pos: &mut (i32, i32), ball_speed: &mut i32, frame_count: u32, is_laggy: bool) {
    if frame_count % 3 != 0 {
        return;
    }
    // 33% chance to cancel frame to simulate lag
    if is_laggy && rand::random::<f64>() < 0.33 {
        return;
    }
    let current_height = ball_pos.1 as f64; // Would get current location of entity or something in spigot plugin
    let time = frame_count as f64 / 60.0; // Would be 20 tps in spigot plugin
    let tick_duration = 0.05;
    let speed = 1.5;
    let magnitude = 300.0; // A lot bigger than it would be in spigot plugin
    let expected_height = (time * speed + tick_duration).sin() * magnitude;
    let mut new_speed = ((expected_height - current_height)/(1.0/tick_duration)) as i32;
    if new_speed > 20 { // Cap speed so lag doesn't make it go too fast
        new_speed = 20;
    }
    if new_speed < -20 {
        new_speed = -20;
    }
    *ball_speed = new_speed;
}

fn get_new_ball_pos(ball_pos: &mut (i32, i32), ball_speed: &mut i32) {
    ball_pos.1 += *ball_speed;
}

fn add_sine_point(sine_points: &mut Vec<SDL_Point>, ball_pos: &mut (i32, i32)) {
    let new_point = SDL_Point { x: ball_pos.0, y: ball_pos.1 };
    sine_points.push(new_point);
    if sine_points.len() > 600 {
        sine_points.remove(0);
    }
    for point in sine_points {
        point.x += 3;
    }
}

fn render_sine_points(canvas: &mut WindowCanvas, sine_points: &mut Vec<SDL_Point>) {
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    // Draw lines between points
    for i in 0..sine_points.len() - 1 {
        let point1 = sine_points[i];
        let point2 = sine_points[i + 1];
        canvas.draw_line(sdl2::rect::Point::new(point1.x, point1.y + (HEIGHT * 10) as i32), sdl2::rect::Point::new(point2.x, point2.y + (HEIGHT * 10) as i32)).unwrap();
    }
}