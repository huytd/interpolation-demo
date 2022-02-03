use std::f32::consts::PI;
use config::{WIDTH, HEIGHT, BG_PIXEL, FG_PIXEL, RED_PIXEL, TRACE_PIXEL};
use minifb::{Window, WindowOptions, Key, Scale};
use text::{draw_char_at, draw_text, DOT_CHAR};

mod text;
mod config;

enum DemoMode {
    Lerp,
    Cosine,
    Acceleration,
    Deceleration
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a * (1.0 - t) + b * t;
}

fn cosine(a: f32, b: f32, t: f32) -> f32 {
    let t2 = (1.0 - (t * PI).cos()) / 2.0;
    return lerp(a, b, t2);
}

fn acceleration(a: f32, b: f32, t: f32) -> f32 {
    let t2 = t * t;
    return lerp(a, b, t2);
}

fn deceleration(a: f32, b: f32, t: f32) -> f32 {
    let t2 = 1.0 - (1.0 - t) * (1.0 - t);
    return lerp(a, b, t2);
}

fn main() {
    let mut window = Window::new(
        "Interpolation Demo",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        }
    ).unwrap();

    let mut mode = DemoMode::Lerp;

    let mut buffer = [BG_PIXEL; WIDTH * HEIGHT];
    let points = [100.0, 20.0];
    let start = points[0];
    let end = points[1];

    window.limit_update_rate(Some(std::time::Duration::from_micros(41600)));

    let mut ta = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Key1) {
            mode = DemoMode::Lerp;
        }
        if window.is_key_down(Key::Key2) {
            mode = DemoMode::Cosine;
        }
        if window.is_key_down(Key::Key3) {
            mode = DemoMode::Acceleration;
        }
        if window.is_key_down(Key::Key4) {
            mode = DemoMode::Deceleration;
        }

        buffer = [BG_PIXEL; WIDTH * HEIGHT];

        for t in 0..100 {
            let y = match mode {
                DemoMode::Lerp => lerp(start, end, t as f32 / 100.0),
                DemoMode::Cosine => cosine(start, end, t as f32 / 100.0),
                DemoMode::Acceleration => acceleration(start, end, t as f32 / 100.0),
                DemoMode::Deceleration => deceleration(start, end, t as f32 / 100.0),
            };
            let col = 40 + t * 2;
            let row = y as usize;
            buffer[row * WIDTH + col] = FG_PIXEL;
        }

        let a = (96, 31);
        draw_char_at(a.0, a.1, &'a', &mut buffer, RED_PIXEL);
        draw_char_at(a.0 + 1, a.1 + 6, &DOT_CHAR, &mut buffer, RED_PIXEL);

        let b = (16, 245);
        draw_char_at(b.0, b.1, &'b', &mut buffer, RED_PIXEL);
        draw_char_at(b.0 + 1, b.1 - 6, &DOT_CHAR, &mut buffer, RED_PIXEL);

        let mut tt = 0.0;
        while tt <= 1.0 {
            tt += 0.035;
            let col = match mode {
                DemoMode::Lerp => lerp(40.0, 245.0, tt),
                DemoMode::Cosine => cosine(40.0, 245.0, tt),
                DemoMode::Acceleration => acceleration(40.0, 245.0, tt),
                DemoMode::Deceleration => deceleration(40.0, 245.0, tt),
            } as usize;
            let row = 114;
            draw_char_at(row, col, &DOT_CHAR, &mut buffer, TRACE_PIXEL);
        }

        // animation
        if ta <= 1.0 {
            ta += 0.035;
            let col = match mode {
                DemoMode::Lerp => lerp(40.0, 245.0, ta),
                DemoMode::Cosine => cosine(40.0, 245.0, ta),
                DemoMode::Acceleration => acceleration(40.0, 245.0, ta),
                DemoMode::Deceleration => deceleration(40.0, 245.0, ta),
            } as usize;
            let row = 114;
            draw_char_at(row, col, &DOT_CHAR, &mut buffer, RED_PIXEL);
        } else {
            ta = 0.0;
        }

        draw_text(10, 10,
                  match mode {
                      DemoMode::Lerp => "Linear Interpolation",
                      DemoMode::Cosine => "Cosine Interpolation",
                      DemoMode::Acceleration => "Acceleration",
                      DemoMode::Deceleration => "Deceleration"
                  }, &mut buffer, RED_PIXEL);
        draw_text(17, 10, "Use 1/2/3/4 keys to switch mode", &mut buffer, TRACE_PIXEL);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

