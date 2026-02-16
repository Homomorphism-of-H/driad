use std::{error::Error, time::Duration};

use driad::Driad;
use mlua::{Function, chunk};
use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    rect::Rect,
    render::FRect,
};

fn blamo(a: i32, b: i32) -> Result<i32, mlua::Error> {
    Ok(a + b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let driad = Driad::new()?;

    let lua = mlua::Lua::new();

    let func = lua
        .load(chunk! {
        local function alpha(a, b)
            return a + b
        end
        })
        .into_function()
        .unwrap();

    let out = func.call::<i32>((10i32, 5i32)).unwrap();

    println!("{out}");

    Function::wrap(blamo);

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

    let mut texture = texture_creator.create_texture_static(PixelFormat::RGB24, 2, 2)?;

    texture.update(
        Rect::new(0, 0, 2, 2),
        &[255, 155, 150, 0, 0, 0, 100, 100, 100, 150, 180, 250],
        3,
    )?;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, Some(FRect::new(300., 300., 32., 32.)))?;

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
