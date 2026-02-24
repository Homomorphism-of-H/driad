use std::error::Error;
use std::time::Duration;

use driad::{Driad, WindowProperties};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;

/// Main Entrypoint to the program.
fn main() -> Result<(), Box<dyn Error>> {
    let mut driad = Driad::new(
        WindowProperties::default(),
        "assets/Alloy_curses_12x12.png",
        vec!["plugins/test", "plugins/other"],
    )?;

    driad.init_plugins()?;

    driad.canvas.set_draw_color(Color::RGB(0, 255, 255));
    driad.canvas.clear();
    driad.canvas.present();

    let mut pos_x = 12;
    let mut pos_y = 12;

    'running: loop {
        driad.canvas.set_draw_color(Color::RGB(0, 0, 0));
        driad.canvas.clear();

        driad.font.put_str(&mut driad.canvas, "Hello World!", (2, 2))?;

        driad.font.put(&mut driad.canvas, '@', (pos_x, pos_y))?;

        for event in driad.event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    pos_y -= 1;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    pos_y += 1;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    pos_x += 1;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    pos_x -= 1;
                },

                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {},
            }
        }

        driad.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
