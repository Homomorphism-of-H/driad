use std::error::Error;
use std::time::Duration;

use driad_core::color::{Color, Palette};
use driad_core::font::Font;
use driad_core::plugin::PluginApi;
use driad_core::{Driad, WindowProperties};
use log::{LevelFilter, warn};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use simplelog::{Config, SimpleLogger};

/// Main Entrypoint to the program.
fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Trace, Config::default())?;

    let font = Font::new(
        "assets/Alloy_curses_12x12.png",
        Palette::simple(
            Color {
                r : 255,
                g : 255,
                b : 255,
            },
            Color {
                r : 255,
                g : 0,
                b : 255,
            },
        ),
    )?;

    let mut driad = Driad::new(
        WindowProperties::default(),
        font,
        Vec::<String>::new(), 
        // vec!["plugins/test", "plugins/other"],
    )?;

    driad.init_plugins()?;

    driad.canvas.set_draw_color(Color::new(0, 255, 255));
    driad.canvas.clear();
    driad.canvas.present();

    let mut pos_x = 12;
    let mut pos_y = 12;

    'running: loop {
        driad.canvas.set_draw_color(Color::new(0, 0, 0));
        driad.canvas.clear();

        driad
            .font
            .put_str(&mut driad.canvas, "Hello World!", (2, 2))?;

        driad.font.put(
            &mut driad.canvas,
            '@',
            (pos_x, pos_y),
            Color::new(255, 255, 0),
        )?;

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

        for plugin in &driad.plugins {
            plugin.draw_pass().inspect(|a| {
                match a {
                    Ok(draw) => {
                        match driad
                            .font
                            .put_char(&mut driad.canvas, draw.glyph, (draw.x, draw.y))
                        {
                            Ok(()) => (),
                            Err(err) => warn!("{err}"),
                        }
                    },
                    Err(err) => warn!("{err}"),
                }
            });
        }

        driad.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
