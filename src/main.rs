use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

extern crate sdl2;

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video_subsystem = ctx.video()?;

    let window = video_subsystem
        .window("runner game", 1280, 720)
        .position_centered()
        .build()
        .expect("Could not create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let mut event_pump = ctx.event_pump()?;

    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Game Logic

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
