use std::collections::HashMap;

use sdl3::render::Texture;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTable {
    /// A table of values stored in icon name and offset pairs
    pub data : HashMap<String, (u32, u32)>,
}

pub struct Font<'tex> {
    pub glyph_height : u32,
    pub glyph_width :  u32,

    /// A font atlas in codepage 437 format.
    pub font_atlas : Texture<'tex>,

    pub extensions : HashMap<String, (LookupTable, Texture<'tex>)>,
}
