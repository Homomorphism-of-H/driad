use std::path::Path;

use log::{error, trace, warn};
use mlua::Lua;
use sdl3::render::Canvas;
use sdl3::video::{Window, WindowBuildError};
use sdl3::{EventPump, IntegerOrSdlError, Sdl, VideoSubsystem};
use thiserror::Error;

use crate::font::{Font, FontCreationError};
use crate::plugin::{LoadPluginError, Plugin, PluginApi};

pub mod char;
pub mod color;
pub mod font;
pub mod plugin;
pub mod widget;

pub struct Driad {
    /// The Sdl Library
    pub sdl :    Sdl,
    pub video :  VideoSubsystem,
    pub window : Window,
    pub canvas : Canvas<Window>,
    pub font :   Font,

    pub event_pump : EventPump,

    pub plugins_initialized : bool,
    /// The Lua runtime
    pub lua :                 Lua,
    pub plugins :             Vec<Plugin>,
}

// TODO, rework this to be a window builder
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
    /// Creates a new `Driad` object
    pub fn new<T : AsRef<Path>>(
        window_properties : WindowProperties,
        font : Font,
        plugin_paths : Vec<T>,
    ) -> Result<Self, DriadNewError> {
        trace!("Initializing SDL 3");
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

        trace!("Initialized Driad");

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
    /// function yet.
    pub fn load_plugin(&mut self, path : impl AsRef<Path>) -> Result<bool, LoadPluginError> {
        if !self.plugins_initialized {
            self.plugins.push(Plugin::load_from_path(path, &self.lua)?);
            Ok(true)
        } else {
            warn!("Attempted to load a plugin after initialization");
            Ok(false)
        }
    }

    pub fn init_plugins(&mut self) -> Result<bool, mlua::Error> {
        if !self.plugins_initialized {
            for plugin in &self.plugins {
                trace!("Loading plugin: {}", plugin.metadata.name);
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
    SdlError(#[from] sdl3::Error),

    #[error(transparent)]
    WindowBuildError(#[from] WindowBuildError),

    #[error(transparent)]
    IntegerOrSdlError(#[from] IntegerOrSdlError),

    #[error(transparent)]
    FontCreationError(#[from] FontCreationError),

    #[error(transparent)]
    LoadPluginError(#[from] LoadPluginError),
}

// TODO
pub mod draw {
    use std::ops::{Deref, DerefMut};

    use crate::char::Char437;
    use crate::color::Color;

    #[derive(Debug, Default)]
    pub enum DrawCommand {
        PutChr {
            pos : (i32, i32),
            chr : Char437,
            col : Color,
        },
        #[default]
        None,
    }

    pub struct DrawPass {
        commands : Vec<DrawCommand>,
    }

    impl DerefMut for DrawPass {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.commands
        }
    }

    impl Deref for DrawPass {
        type Target = Vec<DrawCommand>;

        fn deref(&self) -> &Self::Target {
            &self.commands
        }
    }
}
