use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;

use mlua::{Function, Lua, Table};
use serde::Deserialize;
use thiserror::Error;

use self::version::Version;

pub mod version {
    use std::fmt;
    use std::num::ParseIntError;
    use std::str::FromStr;

    use serde::Deserialize;
    use serde::de::{self, Visitor};
    use thiserror::Error;

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

    impl fmt::Display for Version {
        fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }

    #[derive(Debug, Error)]
    pub enum ParseVersionError {
        #[error(transparent)]
        ParseIntError(#[from] ParseIntError),

        #[error("Version is not 3 entries long")]
        WrongLength,
    }
}

/// A plugin
pub struct Plugin {
    pub metadata : Metadata,

    // Eventually move over to a dynamic option
    api : LuaPluginApi,
}

impl DerefMut for Plugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api
    }
}

impl Deref for Plugin {
    type Target = LuaPluginApi;

    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

pub trait PluginApi {
    type Err;

    fn init(&self) -> Option<Result<(), Self::Err>> {
        None
    }

    fn draw_pass(&self) -> Option<Result<DrawCommand, Self::Err>> {
        None
    }
}

pub struct LuaPluginApi {
    init :      Option<Function>,
    draw_pass : Option<Function>,
}

impl LuaPluginApi {
    pub fn new(table : Table) -> Self {
        Self {
            init :      table.get("init").ok(),
            draw_pass : table.get("draw_pass").ok(),
        }
    }
}

impl PluginApi for LuaPluginApi {
    type Err = mlua::Error;

    fn init(&self) -> Option<Result<(), Self::Err>> {
        self.init.as_ref().map(|init| init.call(()))
    }

    fn draw_pass(&self) -> Option<Result<DrawCommand, Self::Err>> {
        let out = self
            .draw_pass
            .as_ref()
            .map(|draw_pass| draw_pass.call::<Table>(()))?;

        Some(out.and_then(|tab| -> Result<DrawCommand, Self::Err> {
            Ok(DrawCommand {
                x :     tab.get("x")?,
                y :     tab.get("y")?,
                glyph : tab.get("glyph")?,
            })
        }))
    }
}

pub struct DrawCommand {
    pub x :     i32,
    pub y :     i32,
    pub glyph : char,
}

impl Plugin {
    #[must_use]
    #[inline(always)]
    pub fn new_from_parts(metadata : Metadata, functions : Table) -> Self {
        Self {
            metadata,
            api : LuaPluginApi::new(functions),
        }
    }

    pub fn load_from_path(path : impl AsRef<Path>, lua : &Lua) -> Result<Self, LoadPluginError> {
        let metadata = Self::fetch_metadata(&path)?;
        let functions = Self::load_lua_functions(lua, &path)?;
        Ok(Self {
            metadata,
            api : LuaPluginApi::new(functions),
        })
    }

    pub fn fetch_metadata(path : impl AsRef<Path>) -> Result<Metadata, FetchMetadataError> {
        let dir_reader = Path::read_dir(path.as_ref())?;

        let metadata = dir_reader
            .flatten()
            .find(|entry| {
                entry.file_name().eq_ignore_ascii_case("metadata.toml")
                    || entry.file_name().eq_ignore_ascii_case("meta.toml")
                    || entry.file_name().eq_ignore_ascii_case("data.toml")
            })
            .ok_or(FetchMetadataError::MetadataNotFound)?;

        let mut buf = String::new();

        {
            let mut file = File::open(metadata.path())?;
            file.read_to_string(&mut buf)?;
        }

        let metadata = toml::from_str::<Metadata>(&buf)?;

        Ok(metadata)
    }

    /// Warning, this function can execute arbitrary lua code, be warned.
    pub fn load_lua_functions(lua : &Lua, path : impl AsRef<Path>) -> Result<Table, FetchLuaError> {
        let dir_reader = Path::read_dir(path.as_ref())?;

        let functions = dir_reader
            .flatten()
            .find(|entry| entry.file_name().eq_ignore_ascii_case("main.lua"))
            .ok_or(FetchLuaError::LuaNotFound)?;

        let mut buf = String::new();

        {
            let mut file = File::open(functions.path())?;
            file.read_to_string(&mut buf)?;
        }

        lua.load(buf).eval::<Table>().map_err(From::from)
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name :    String,
    // Could this be a `Box<[String]>`
    pub authors : Vec<String>,
    pub version : Version,
}

impl Display for Metadata {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Name: {}\nAuthors:", self.name)?;

        for author in self.authors.clone() {
            writeln!(f, " - {author}")?;
        }

        write!(f, "Version {}", self.version)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum FetchMetadataError {
    #[error("Plugin folder lacks a metadata.toml file")]
    MetadataNotFound,

    #[error(transparent)]
    TomlParseError(#[from] toml::de::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum FetchLuaError {
    #[error(transparent)]
    LuaError(#[from] mlua::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("Plugin folder lackas a main.lua file")]
    LuaNotFound,
}

#[derive(Debug, Error)]
pub enum LoadPluginError {
    #[error(transparent)]
    IoError(io::Error),

    #[error(transparent)]
    FetchLuaError(#[from] FetchLuaError),

    #[error(transparent)]
    FetchMetadataError(#[from] FetchMetadataError),
}
