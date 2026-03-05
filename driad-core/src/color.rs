use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use image::{Rgb, Rgba};
use sdl3::pixels;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
/// A RGB color.
pub struct Color {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

impl Color {
    #[must_use]
    #[inline]
    pub const fn new(r : u8, g : u8, b : u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Palette {
    pub fg :         Color,
    pub fg_accent1 : Option<Color>,
    pub fg_accent2 : Option<Color>,

    pub bg :         Color,
    pub bg_accent1 : Option<Color>,
    pub bg_accent2 : Option<Color>,
}

impl Palette {
    #[inline]
    #[must_use]
    pub fn bg_accent2(&self) -> Color {
        self.bg_accent2.unwrap_or_else(|| self.bg_accent1())
    }

    #[inline]
    #[must_use]
    pub fn bg_accent1(&self) -> Color {
        self.bg_accent1.unwrap_or(self.bg)
    }

    #[inline]
    #[must_use]
    pub fn fg_accent2(&self) -> Color {
        self.fg_accent2.unwrap_or_else(|| self.fg_accent1())
    }

    #[inline]
    #[must_use]
    pub fn fg_accent1(&self) -> Color {
        self.fg_accent1.unwrap_or(self.fg)
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            fg : Color {
                r : 255,
                g : 255,
                b : 255,
            },
            bg : Color {
                r : 0,
                g : 0,
                b : 0,
            },

            fg_accent1 : None,
            fg_accent2 : None,
            bg_accent1 : None,
            bg_accent2 : None,
        }
    }
}

impl Palette {
    #[must_use]
    pub const fn simple(fg : Color, bg : Color) -> Self {
        Self {
            fg,
            bg,
            fg_accent1 : None,
            fg_accent2 : None,
            bg_accent1 : None,
            bg_accent2 : None,
        }
    }
}

impl From<Color> for Palette {
    fn from(value : Color) -> Self {
        Self {
            fg : value,
            ..Default::default()
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs : Self) -> Self::Output {
        Self {
            r : self.r.saturating_add(rhs.r),
            g : self.r.saturating_add(rhs.g),
            b : self.r.saturating_add(rhs.b),
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs : Self) {
        *self = *self + rhs;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs : Self) -> Self::Output {
        Self {
            r : self.r.saturating_sub(rhs.r),
            g : self.r.saturating_sub(rhs.g),
            b : self.r.saturating_sub(rhs.b),
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs : Self) {
        *self = *self - rhs;
    }
}

impl<R : Into<u8>, G : Into<u8>, B : Into<u8>> From<(R, G, B)> for Color {
    fn from((r, g, b) : (R, G, B)) -> Self {
        Self {
            r : r.into(),
            g : g.into(),
            b : b.into(),
        }
    }
}

impl<C : Into<u8>> From<[C; 3]> for Color {
    fn from([r, g, b] : [C; 3]) -> Self {
        Self {
            r : r.into(),
            g : g.into(),
            b : b.into(),
        }
    }
}

impl From<pixels::Color> for Color {
    fn from(value : pixels::Color) -> Self {
        Self {
            r : value.r,
            g : value.g,
            b : value.b,
        }
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value : Rgb<u8>) -> Self {
        Self {
            r : value.0[0],
            g : value.0[1],
            b : value.0[2],
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(value : Rgba<u8>) -> Self {
        Self {
            r : value.0[0],
            g : value.0[1],
            b : value.0[2],
        }
    }
}

impl<R : From<u8>, G : From<u8>, B : From<u8>> From<Color> for (R, G, B) {
    fn from(value : Color) -> Self {
        (value.r.into(), value.g.into(), value.b.into())
    }
}

impl From<Color> for [u8; 3] {
    fn from(value : Color) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<Color> for pixels::Color {
    fn from(value : Color) -> Self {
        Self {
            r : value.r,
            g : value.g,
            b : value.b,
            a : 255,
        }
    }
}

impl<T : From<u8>> From<Color> for Rgb<T> {
    fn from(value : Color) -> Self {
        Self([value.r.into(), value.g.into(), value.b.into()])
    }
}

impl From<Color> for Rgba<u8> {
    fn from(value : Color) -> Self {
        Self([value.r, value.g, value.b, 255])
    }
}
