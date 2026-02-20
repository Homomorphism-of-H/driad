use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::str::FromStr;

use mlua::Lua;
use sdl3::{Sdl, VideoSubsystem};
use serde::Deserialize;
use serde::de::{self, Visitor};
use thiserror::Error;

use crate::plugin::{LoadPluginError, Plugin};

pub mod font;
pub mod char;
pub mod plugin;

pub struct Driad {
    pub sdl :   Sdl,
    pub video : VideoSubsystem,
    pub lua :   Lua,

    plugins_initialized : bool,
    pub plugins :         Vec<Plugin>,
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

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DriadNewError {
    #[error(transparent)]
    SDL3Error(#[from] sdl3::Error),
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct Version {
    major : u32,
    minor : u32,
    patch : u32,
}

struct VersionVisitor;

impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"a string tuple of u32s like "1.3.104""#)
    }

    fn visit_str<E>(self, v : &str) -> Result<Self::Value, E>
    where
        E : de::Error,
    {
        Version::from_str(v).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer : D) -> Result<Self, D::Error>
    where
        D : serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionVisitor)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let splits : Vec<&str> = s.split('.').collect();

        if splits.len() != 3 {
            return Err(ParseVersionError::WrongLength);
        }

        let mut splits = splits.iter();

        Ok(Self {
            major : splits.next().unwrap().parse()?,
            minor : splits.next().unwrap().parse()?,
            patch : splits.next().unwrap().parse()?,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Error)]
pub enum ParseVersionError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error("Version is not 3 long")]
    WrongLength,
}
