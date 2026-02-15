use std::{error::Error, time::Duration};

use sdl3::{event::Event, keyboard::Keycode, pixels::Color, render::FPoint};

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl3::init()?;

    let video_subsytem = sdl.video()?;

    let window = video_subsytem
        .window("Beep Borp", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump()?;
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_point(FPoint::new(100., 100.))?;
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
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
