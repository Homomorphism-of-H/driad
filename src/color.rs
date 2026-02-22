use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use image::{Rgb, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Palete {
    pub fg :      Color,
    pub bg :      Color,
    pub accent1 : Option<Color>,
    pub accent2 : Option<Color>,
    pub accent3 : Option<Color>,
    pub accent4 : Option<Color>,
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
            r : self.r + rhs.r,
            g : self.g + rhs.g,
            b : self.b + rhs.b,
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
            r : self.r - rhs.r,
            g : self.g - rhs.g,
            b : self.b - rhs.b,
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

impl From<sdl3::pixels::Color> for Color {
    fn from(value : sdl3::pixels::Color) -> Self {
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

impl From<Color> for (u8, u8, u8) {
    fn from(value : Color) -> Self {
        (value.r, value.g, value.b)
    }
}

impl From<Color> for [u8; 3] {
    fn from(value : Color) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<Color> for sdl3::pixels::Color {
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
