use std::path::Path;

use log::error;
use mlua::Lua;
use sdl3::render::Canvas;
use sdl3::video::{Window, WindowBuildError};
use sdl3::{EventPump, IntegerOrSdlError, Sdl, VideoSubsystem};
use thiserror::Error;

use crate::color::Palette;
use crate::font::{Font, FontCreationError};
use crate::plugin::{LoadPluginError, Plugin, PluginApi};

pub mod char;
pub mod color;
pub mod font;
pub mod plugin;
pub mod widget;

pub struct Driad {
    pub sdl :    Sdl,
    pub video :  VideoSubsystem,
    pub window : Window,
    pub canvas : Canvas<Window>,
    pub font :   Font,

    pub event_pump : EventPump,

    pub plugins_initialized : bool,
    pub lua :                 Lua,
    pub plugins :             Vec<Plugin>,
}

// Todo, rework this to be a window Builder
#[derive(Debug)]
pub struct WindowProperties {
    pub width :      u32,
    pub height :     u32,
    pub name :       &'static str,
    pub centered :   bool,
    pub borderless : bool,
}

impl Default for WindowProperties {
    fn default() -> Self {
        Self {
            width :      80,
            height :     60,
            name :       "Driad Window",
            centered :   true,
            borderless : false,
        }
    }
}

impl Driad {
    pub fn new<T : AsRef<Path>>(
        window_properties : WindowProperties,
        font_path : impl AsRef<Path>,
        font_palette: impl Into<Palette>,
        plugin_paths : Vec<T>,
    ) -> Result<Self, DriadNewError> {
        let sdl = sdl3::init()?;

        let video = sdl.video()?;
        let mut window = video.window(
            window_properties.name,
            window_properties.width,
            window_properties.height,
        );
        if window_properties.borderless {
            window.borderless();
        }
        if window_properties.centered {
            window.position_centered();
        }

        let mut window = window.build()?;

        let canvas = window.clone().into_canvas();

        let texture_creator = canvas.texture_creator();
        // Hardcoded until Color is better integrated into fonts
        let font = Font::new(&texture_creator, font_path, font_palette)?;

        window.set_size(
            window_properties.width * font.glyph_width,
            window_properties.height * font.glyph_height,
        )?;

        let event_pump = sdl.event_pump()?;

        let lua = Lua::new();

        let plugins = plugin_paths
            .iter()
            .filter_map(|plugin_path| {
                match Plugin::load_from_path(plugin_path, &lua) {
                    Ok(ok) => Some(ok),
                    Err(err) => {
                        error!("{err}");
                        None
                    },
                }
            })
            .collect();

        Ok(Self {
            sdl,
            video,
            window,
            canvas,
            font,
            event_pump,
            lua,
            plugins,
            plugins_initialized : false,
        })
    }

    #[inline]
    /// Attaches a plugin to the Driad runtime without calling initializer
    /// functions yet.
    pub fn load_plugin(&mut self, path : impl AsRef<Path>) -> Result<bool, LoadPluginError> {
        if !self.plugins_initialized {
            self.plugins.push(Plugin::load_from_path(path, &self.lua)?);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn init_plugins(&mut self) -> Result<bool, mlua::Error> {
        if !self.plugins_initialized {
            for plugin in &self.plugins {
                if let Some(res) = plugin.init() {
                    res?
                }
            }

            self.plugins_initialized = true;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn sdl(&self) -> &Sdl {
        &self.sdl
    }
}

#[derive(Debug, Error)]
pub enum DriadNewError {
    #[error(transparent)]
    SDL3Error(#[from] sdl3::Error),
    #[error(transparent)]
    WindowBuildError(#[from] WindowBuildError),
    #[error(transparent)]
    IntegerOrSdlError(#[from] IntegerOrSdlError),
    #[error(transparent)]
    FontCreationError(#[from] FontCreationError),
    #[error(transparent)]
    LoadPluginError(#[from] LoadPluginError),
}
