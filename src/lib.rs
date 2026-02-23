use std::ops::{Deref, DerefMut};
use std::path::Path;

use mlua::Lua;
use sdl3::video::WindowBuildError;
use sdl3::{IntegerOrSdlError, Sdl, VideoSubsystem};
use thiserror::Error;

use crate::font::FontCreationError;
use crate::plugin::{LoadPluginError, Plugin};

pub mod char;
pub mod color;
pub mod font;
pub mod plugin;

pub struct Driad {
    pub sdl :   Sdl,
    pub video : VideoSubsystem,
    pub lua :   Lua,

    plugins_initialized : bool,
    pub plugins :         Vec<Plugin>,
}

#[derive(Debug)]
pub struct WindowProperties {
    pub width :      u32,
    pub height :     u32,
    pub name :       &'static str,
    pub centered :   bool,
    pub borderless : bool,
    pub fullscreen : bool,
}

impl Default for WindowProperties {
    fn default() -> Self {
        Self {
            width :      400,
            height :     300,
            name :       "Driad Window",
            centered :   true,
            borderless : false,
            fullscreen : false,
        }
    }
}

impl Driad {
    pub fn new() -> Result<Self, DriadNewError> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        let lua = Lua::new();
        Ok(Self {
            sdl,
            video,
            lua,
            plugins_initialized : false,
            plugins : Vec::new(),
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
                if let Some(res) = plugin.call_init() {
                    res?
                }
            }

            self.plugins_initialized = true;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl DerefMut for Driad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sdl
    }
}

impl Deref for Driad {
    type Target = Sdl;

    fn deref(&self) -> &Self::Target {
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
}

pub mod tileset {}
