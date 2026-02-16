use std::ops::{Deref, DerefMut};

use sdl3::{Sdl, VideoSubsystem};
use thiserror::Error;

pub struct Driad {
    pub sdl: Sdl,
    pub video: VideoSubsystem,
}

impl Driad {
    pub fn new() -> Result<Self, DriadNewError> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        Ok(Self { sdl, video })
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DriadNewError {
    #[error(transparent)]
    SDL3Error(sdl3::Error),
}

impl From<sdl3::Error> for DriadNewError {
    fn from(v: sdl3::Error) -> Self {
        Self::SDL3Error(v)
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
