use std::collections::HashMap;
use std::path::Path;

use image::{EncodableLayout, GenericImageView, ImageError};
use sdl3::pixels::PixelFormat;
use sdl3::rect::Rect;
use sdl3::render::{
    Canvas, RenderTarget, ScaleMode, Texture, TextureCreator, TextureValueError, UpdateTextureError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::char::Char437;

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTable {
    /// A table of values stored in icon name and offset pairs. Note: The
    /// offsets are tile sized, not pixel sized.
    pub data : HashMap<String, (u32, u32)>,
}

pub struct Font<'tex> {
    pub glyph_height : u32,
    pub glyph_width :  u32,

    /// A font atlas in codepage 437 format.
    pub font_atlas : Texture<'tex>,

    pub extensions : HashMap<String, (LookupTable, Texture<'tex>)>,
}

impl<'tex> Font<'tex> {
    pub fn new<T>(
        texture_creator : &'tex TextureCreator<T>,
        path : impl AsRef<Path>,
    ) -> Result<Self, FontCreationError> {
        let im = image::open(path)?;

        let (w, h) = im.dimensions();

        if !(w % 16 == 0 && h % 16 == 0) {
            return Err(FontCreationError::BadlySized);
        }

        let mut font_atlas : Texture<'tex> =
            texture_creator.create_texture_static(PixelFormat::RGB24, w, h)?;
        font_atlas.update(
            Rect::new(0, 0, w, h),
            im.into_rgb8().as_bytes(),
            w as usize * 3,
        )?;
        font_atlas.set_scale_mode(ScaleMode::Nearest);

        Ok(Self {
            glyph_height : w,
            glyph_width : h,
            font_atlas,
            extensions : HashMap::new(),
        })
    }

    pub fn put<T : RenderTarget>(&self, canvas : &mut Canvas<T>, key : impl Into<FontKey>) {}
}

pub enum FontKey {
    Char(Char437),
    Icon(String, String),
}

#[derive(Debug, Error)]
pub enum FontCreationError {
    #[error(transparent)]
    ImageError(#[from] ImageError),

    #[error(transparent)]
    TextureValueError(#[from] TextureValueError),

    #[error(transparent)]
    UpdateTextureError(#[from] UpdateTextureError),

    #[error("Badly sized font atlas")]
    BadlySized,
}
