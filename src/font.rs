use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

use image::{EncodableLayout, GenericImage, GenericImageView, ImageError, Rgba};
use sdl3::pixels::PixelFormat;
use sdl3::rect::Rect;
use sdl3::render::{
    Canvas, RenderTarget, ScaleMode, Texture, TextureCreator, TextureValueError, UpdateTextureError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::char::Char437;
use crate::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTable {
    /// A table of values stored in icon name and offset pairs. Note: The
    /// offsets are tile sized, not pixel sized.
    pub data : HashMap<String, (u32, u32)>,
}

impl Deref for LookupTable {
    type Target = HashMap<String, (u32, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct Font {
    pub glyph_height : u32,
    pub glyph_width :  u32,

    /// A font atlas in codepage 437 format.
    pub font_atlas : Texture,

    pub extensions : HashMap<String, (LookupTable, Texture)>,
}

impl Font {
    pub fn new<T>(
        texture_creator : &TextureCreator<T>,
        path : impl AsRef<Path>,
        bg : impl Into<Option<Color>>,
    ) -> Result<Self, FontCreationError> {
        let mut im = image::open(path)?;

        let (w, h) = im.dimensions();

        if let Some(bg) = bg.into() {
            for (x, y, p) in im.clone().pixels() {
                if p == bg.into() {
                    im.put_pixel(x, y, Rgba::from([0, 0, 0, 255]));
                }
            }
        }

        if !(w % 16 == 0 && h % 16 == 0) {
            return Err(FontCreationError::BadlySized);
        }

        let mut font_atlas : Texture =
            texture_creator.create_texture_static(PixelFormat::RGB24, w, h)?;
        font_atlas.update(
            Rect::new(0, 0, w, h),
            im.into_rgb8().as_bytes(),
            w as usize * 3,
        )?;
        font_atlas.set_scale_mode(ScaleMode::Nearest);

        Ok(Self {
            glyph_height : w / 16,
            glyph_width : h / 16,
            font_atlas,
            extensions : HashMap::new(),
        })
    }

    pub fn put<T : RenderTarget>(
        &self,
        canvas : &mut Canvas<T>,
        key : impl Into<FontKey>,
        pos : (i32, i32),
    ) -> Result<(), sdl3::Error> {
        if let Some((texture, src)) = self.lookup_glyph(key) {
            canvas.copy(
                texture,
                src,
                Rect::new(
                    pos.0 * self.glyph_width as i32,
                    pos.1 * self.glyph_height as i32,
                    self.glyph_width,
                    self.glyph_height,
                ),
            )
        } else {
            Ok(())
        }
    }

    pub fn put_str<T : RenderTarget>(
        &self,
        canvas : &mut Canvas<T>,
        text : &str,
        pos : (i32, i32),
    ) -> Result<(), sdl3::Error> {
        text.chars()
            .enumerate()
            .try_for_each(|(idx, c)| self.put(canvas, c, (pos.0 + idx as i32, pos.1)))
    }

    pub fn lookup_glyph(&self, key : impl Into<FontKey>) -> Option<(&Texture, Rect)> {
        match key.into() {
            FontKey::Char(char437) => {
                Some((
                    &self.font_atlas,
                    self.try_offset_to_local(char437.offset())?,
                ))
            },
            FontKey::Icon(ext, key) => {
                let (lookup, texture) = self.extensions.get(&ext)?;
                let offset = self.try_offset_to_local(*lookup.get(&key)?)?;
                Some((texture, offset))
            },
        }
    }

    pub fn offset_to_local<T1 : Into<i32>, T2 : Into<i32>>(&self, offset : (T1, T2)) -> Rect {
        Rect::new(
            offset.0.into() * self.glyph_width as i32,
            offset.1.into() * self.glyph_height as i32,
            self.glyph_width,
            self.glyph_height,
        )
    }

    pub fn try_offset_to_local<T1 : TryInto<i32>, T2 : TryInto<i32>>(
        &self,
        offset : (T1, T2),
    ) -> Option<Rect> {
        Some(Rect::new(
            offset.0.try_into().ok()? * self.glyph_width as i32,
            offset.1.try_into().ok()? * self.glyph_height as i32,
            self.glyph_width,
            self.glyph_height,
        ))
    }
}

pub enum FontKey {
    Char(Char437),
    Icon(String, String),
}

impl From<Char437> for FontKey {
    fn from(v : Char437) -> Self {
        Self::Char(v)
    }
}

impl From<char> for FontKey {
    fn from(value : char) -> Self {
        Char437::try_from(value).unwrap().into()
    }
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
