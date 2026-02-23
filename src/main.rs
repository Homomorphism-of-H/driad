use std::error::Error;
use std::time::Duration;

use driad::Driad;
use driad::font::Font;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;

/// Main Entrypoint to the program.
fn main() -> Result<(), Box<dyn Error>> {
    let mut driad = Driad::new()?;

    driad.load_plugin("plugins/test")?;
    driad.load_plugin("plugins/other")?;

    driad.init_plugins()?;

    let window = driad
        .video
        .window("Driad", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.clone().into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = driad.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let font = Font::new(
        &texture_creator,
        "assets/Alloy_curses_12x12.png",
        Some([255, 0, 255].into()),
    )?;

    let mut pos_x = 12;
    let mut pos_y = 12;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        font.put_str(&mut canvas, "Hello World!", (2, 2))?;

        font.put(&mut canvas, '@', (pos_x, pos_y))?;

        for event in event_pump.poll_iter() {
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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
