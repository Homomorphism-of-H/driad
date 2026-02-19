use std::error::Error;
use std::time::Duration;

use driad::Driad;
use image::EncodableLayout;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::{Color, PixelFormat};
use sdl3::rect::Rect;
use sdl3::render::ScaleMode;

/// Main Entrypoint to the program.
fn main() -> Result<(), Box<dyn Error>> {
    let driad = Driad::new()?;

    let plugin = driad.load_plugin("plugins/test")?;

    plugin.call_init()?;

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

    let mut font = texture_creator.create_texture_static(PixelFormat::RGB24, 192, 192)?;

    let dat = image::open("assets/Alloy_curses_12x12.png")?;

    font.update(Rect::new(0, 0, 192, 192), dat.into_rgb8().as_bytes(), 576)?;
    font.set_scale_mode(ScaleMode::Nearest);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(
            &font,
            Some(Rect::new(8 * 12, 4 * 12, 12, 12).into()),
            Some(Rect::new(0, 0, 24, 24).into()),
        )?;
        canvas.copy(
            &font,
            Some(Rect::new(9 * 12, 6 * 12, 12, 12).into()),
            Some(Rect::new(24, 0, 24, 24).into()),
        )?;

        for event in event_pump.poll_iter() {
            match event {
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
