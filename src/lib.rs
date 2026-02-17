use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use mlua::Lua;
use sdl3::{Sdl, VideoSubsystem};
use thiserror::Error;

use crate::plugin::{LoadPluginError, Plugin};

pub mod plugin;

pub struct Driad {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
    pub lua: Lua,
}

impl Driad {
    pub fn new() -> Result<Self, DriadNewError> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        let lua = Lua::new();
        Ok(Self { sdl, video, lua })
    }

    pub fn load_plugin(&self, path: impl AsRef<Path>) -> Result<Plugin, LoadPluginError> {
        let metadata = Plugin::fetch_metadata(&path)?;
        let functions = Plugin::load_lua_functions(&self.lua, &path)?;

        println!("{metadata}");

        Ok(Plugin::new_from_parts(metadata, functions))
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

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DriadNewError {
    #[error(transparent)]
    SDL3Error(#[from] sdl3::Error),
}
