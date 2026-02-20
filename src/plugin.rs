use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use mlua::{Function, Lua, Table};
use serde::Deserialize;
use thiserror::Error;

use crate::Version;

/// A plugin
pub struct Plugin {
    pub metadata : Metadata,

    // Eventually move over to a dynamic option
    api : LuaPluginApi,
}

pub trait PluginApi {
    type Err;

    fn init(&self) -> Result<bool, Self::Err>;
}

pub struct LuaPluginApi {
    table : Table,
}

impl LuaPluginApi {
    pub fn new(table : Table) -> Self {
        Self { table }
    }
}

impl PluginApi for LuaPluginApi {
    type Err = mlua::Error;

    fn init(&self) -> Result<bool, Self::Err> {
        self.table
            .get::<Function>("init")
            .and_then(|init| init.call(()))
    }
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

    /// Wrapper around the `init` lua function
    pub fn call_init(&self) -> Result<bool, mlua::Error> {
        self.api.init()
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
