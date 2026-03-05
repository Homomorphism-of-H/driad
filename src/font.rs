use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

use image::{EncodableLayout, GenericImage, GenericImageView, ImageError, Rgb, RgbImage, SubImage};
use log::trace;
use sdl3::pixels::PixelFormat;
use sdl3::rect::Rect;
use sdl3::render::{Canvas, RenderTarget, TextureValueError, UpdateTextureError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::char::Char437;
use crate::color::{Color, Palette};

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
    font_atlas : RgbImage,

    extensions : HashMap<String, (LookupTable, RgbImage)>,
}

impl Font {
    pub fn new(
        path : impl AsRef<Path>,
        palette : impl Into<Palette>,
    ) -> Result<Self, FontCreationError> {
        let mut im = image::open(path)?;

        let (w, h) = im.dimensions();

        if !(w % 16 == 0 && h % 16 == 0) {
            return Err(FontCreationError::BadlySized);
        }

        trace!("Creating font with glyph size {} x {}", w / 16, h / 16);

        let palette = palette.into();

        for (x, y, p) in im.clone().pixels() {
            match p {
                fg if Color::from(fg) == palette.fg => {
                    im.put_pixel(x, y, Color::new(255, 255, 255).into())
                },
                bg if Color::from(bg) == palette.bg => {
                    im.put_pixel(x, y, Color::new(0, 0, 0).into())
                },
                _ => return Err(FontCreationError::BadPalette),
            }
        }

        Ok(Self {
            glyph_height : w / 16,
            glyph_width :  h / 16,
            font_atlas :   im.into_rgb8(),
            extensions :   HashMap::new(),
        })
    }

    pub fn put<T : RenderTarget>(
        &self,
        canvas : &mut Canvas<T>,
        key : impl Into<FontKey>,
        pos : impl Into<(i32, i32)>,
        palette : impl Into<Palette>,
    ) -> Result<(), PutGlyphError> {
        let sub_image = self.lookup_glyph(key).ok_or(PutGlyphError::MissingEntry)?;

        let mut sub_image = sub_image.to_image();

        let palette = palette.into();

        for pix in sub_image.pixels_mut() {
            if *pix == Rgb([255, 255, 255]) {
                *pix = palette.fg.into();
            }
        }

        let mut texture = canvas.create_texture_static(
            PixelFormat::RGB24,
            self.glyph_width,
            self.glyph_height,
        )?;

        texture.update(None, sub_image.as_bytes(), 3 * self.glyph_width as usize)?;

        let (x, y) = pos.into();

        canvas.copy(
            &texture,
            None,
            Rect::new(
                x * self.glyph_width as i32,
                y * self.glyph_height as i32,
                self.glyph_width,
                self.glyph_height,
            ),
        )?;

        // We can do this safely, because by passing a refrence to a canvas into this
        // function we ensure it lives at least the lifetime of this function.
        unsafe { texture.destroy() };

        Ok(())
    }

    pub fn put_char<T : RenderTarget>(
        &self,
        canvas : &mut Canvas<T>,
        key : char,
        pos : impl Into<(i32, i32)>,
    ) -> Result<(), PutGlyphError> {
        let sub_image = self.lookup_char(
            key.try_into()
                .map_err(|_| PutGlyphError::IntoChar437Error)?,
        );

        let mut texture = canvas.create_texture_static(
            PixelFormat::RGB24,
            self.glyph_width,
            self.glyph_height,
        )?;

        texture.update(
            None,
            sub_image.to_image().as_bytes(),
            3 * self.glyph_width as usize,
        )?;

        let (x, y) = pos.into();

        canvas.copy(
            &texture,
            None,
            Rect::new(
                x * self.glyph_width as i32,
                y * self.glyph_height as i32,
                self.glyph_width,
                self.glyph_height,
            ),
        )?;

        // We can do this safely, because by passing a refrence to a canvas into this
        // function we ensure it lives at least the lifetime of this function.
        unsafe { texture.destroy() };

        Ok(())
    }

    pub fn put_str<T : RenderTarget>(
        &self,
        canvas : &mut Canvas<T>,
        text : &str,
        pos : impl Into<(i32, i32)>,
    ) -> Result<(), PutGlyphError> {
        let (x, y) = pos.into();
        text.chars()
            .enumerate()
            .try_for_each(|(idx, c)| self.put_char(canvas, c, (x + idx as i32, y)))
    }

    /// Looks up a glyph texture based upon some type that can be converted into
    /// a key.
    pub fn lookup_glyph(&self, key : impl Into<FontKey>) -> Option<SubImage<&RgbImage>> {
        match key.into() {
            FontKey::Char(chr) => Some(self.lookup_char(chr)),
            FontKey::Icon(ext, key) => {
                let (tab, image) = self.extensions.get(&ext)?;
                let (x, y) = tab.get(&key)?;
                Some(image.view(
                    *x * self.glyph_width,
                    *y * self.glyph_height,
                    self.glyph_width,
                    self.glyph_height,
                ))
            },
        }
    }

    /// Looks up the texture associated with a `Char437`. Always returns, making
    /// it more practical than `lookup_glyph` if you know you are only using
    /// chars.
    pub fn lookup_char(&self, chr : Char437) -> SubImage<&RgbImage> {
        let (x, y) = chr.offset();
        self.font_atlas.view(
            Into::<u32>::into(x) * self.glyph_width,
            Into::<u32>::into(y) * self.glyph_height,
            self.glyph_width,
            self.glyph_height,
        )
    }

    // No longer storing a texture, no need for these
    // pub fn offset_to_local<T1 : Into<i32>, T2 : Into<i32>>(&self, offset : (T1,
    // T2)) -> Rect {     Rect::new(
    //         offset.0.into() * self.glyph_width as i32,
    //         offset.1.into() * self.glyph_height as i32,
    //         self.glyph_width,
    //         self.glyph_height,
    //     )
    // }

    // pub fn try_offset_to_local<T1 : TryInto<i32>, T2 : TryInto<i32>>(
    //     &self,
    //     offset : (T1, T2),
    // ) -> Option<Rect> {
    //     Some(Rect::new(
    //         offset.0.try_into().ok()? * self.glyph_width as i32,
    //         offset.1.try_into().ok()? * self.glyph_height as i32,
    //         self.glyph_width,
    //         self.glyph_height,
    //     ))
    // }
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

    #[error("Palette provided does not match the image loaded")]
    BadPalette,
}

#[derive(Debug, Error)]
pub enum PutGlyphError {
    #[error(transparent)]
    SdlError(#[from] sdl3::Error),

    #[error(transparent)]
    UpdateTextureError(#[from] UpdateTextureError),

    #[error("Encountered a non CP437 char in printing")]
    IntoChar437Error,

    #[error("The provided key does not exist in this font")]
    MissingEntry,

    #[error(transparent)]
    TextureValueError(#[from] TextureValueError),
}
