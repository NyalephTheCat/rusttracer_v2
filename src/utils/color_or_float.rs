use std::ops::{Add, Mul, Sub};

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub enum ColorOrFloat {
    Color(Color),
    Float(f64),
}

impl ColorOrFloat {
    pub fn to_color(&self) -> Color {
        match self {
            ColorOrFloat::Color(color) => *color,
            ColorOrFloat::Float(float) => Color::from((*float, *float, *float)),
        }
    }
}

impl From<Color> for ColorOrFloat {
    fn from(color: Color) -> ColorOrFloat {
        ColorOrFloat::Color(color)
    }
}

impl From<f64> for ColorOrFloat {
    fn from(float: f64) -> ColorOrFloat {
        ColorOrFloat::Float(float)
    }
}

impl From<ColorOrFloat> for Color {
    fn from(color_or_float: ColorOrFloat) -> Color {
        color_or_float.to_color()
    }
}

impl From<ColorOrFloat> for f64 {
    fn from(color_or_float: ColorOrFloat) -> f64 {
        color_or_float.to_color().to_grayscale()
    }
}

impl Add<f64> for ColorOrFloat {
    type Output = ColorOrFloat;

    fn add(self, other: f64) -> ColorOrFloat {
        match self {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(color + other),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(float + other),
        }
    }
}

impl Add<ColorOrFloat> for f64 {
    type Output = ColorOrFloat;

    fn add(self, other: ColorOrFloat) -> ColorOrFloat {
        match other {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(self + color),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(self + float),
        }
    }
}

impl Add<Color> for ColorOrFloat {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        match self {
            ColorOrFloat::Color(color) => color + other,
            ColorOrFloat::Float(float) => float + other,
        }
    }
}

impl Add<ColorOrFloat> for Color {
    type Output = Color;

    fn add(self, other: ColorOrFloat) -> Color {
        match other {
            ColorOrFloat::Color(color) => self + color,
            ColorOrFloat::Float(float) => self + Color::from((float, float, float)),
        }
    }
}

impl Sub for ColorOrFloat {
    type Output = ColorOrFloat;

    fn sub(self, other: ColorOrFloat) -> ColorOrFloat {
        match self {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(color - other.to_color()),
            ColorOrFloat::Float(float) => {
                ColorOrFloat::Float(float - other.to_color().to_grayscale())
            }
        }
    }
}

impl Sub<f64> for ColorOrFloat {
    type Output = ColorOrFloat;

    fn sub(self, other: f64) -> ColorOrFloat {
        match self {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(color - other),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(float - other),
        }
    }
}

impl Sub<ColorOrFloat> for f64 {
    type Output = ColorOrFloat;

    fn sub(self, other: ColorOrFloat) -> ColorOrFloat {
        match other {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(self - color),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(self - float),
        }
    }
}

impl Sub<Color> for ColorOrFloat {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        match self {
            ColorOrFloat::Color(color) => color - other,
            ColorOrFloat::Float(float) => float - other,
        }
    }
}

impl Sub<ColorOrFloat> for Color {
    type Output = Color;

    fn sub(self, other: ColorOrFloat) -> Color {
        match other {
            ColorOrFloat::Color(color) => self - color,
            ColorOrFloat::Float(float) => self - Color::from((float, float, float)),
        }
    }
}

impl Mul for ColorOrFloat {
    type Output = ColorOrFloat;

    fn mul(self, other: ColorOrFloat) -> ColorOrFloat {
        match self {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(color * other.to_color()),
            ColorOrFloat::Float(float) => {
                ColorOrFloat::Float(float * other.to_color().to_grayscale())
            }
        }
    }
}

impl Mul<f64> for ColorOrFloat {
    type Output = ColorOrFloat;

    fn mul(self, other: f64) -> ColorOrFloat {
        match self {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(color * other),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(float * other),
        }
    }
}

impl Mul<ColorOrFloat> for f64 {
    type Output = ColorOrFloat;

    fn mul(self, other: ColorOrFloat) -> ColorOrFloat {
        match other {
            ColorOrFloat::Color(color) => ColorOrFloat::Color(self * color),
            ColorOrFloat::Float(float) => ColorOrFloat::Float(self * float),
        }
    }
}

impl Mul<Color> for ColorOrFloat {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        match self {
            ColorOrFloat::Color(color) => color * other,
            ColorOrFloat::Float(float) => float * other,
        }
    }
}

impl Mul<ColorOrFloat> for Color {
    type Output = Color;

    fn mul(self, other: ColorOrFloat) -> Color {
        match other {
            ColorOrFloat::Color(color) => self * color,
            ColorOrFloat::Float(float) => self * Color::from((float, float, float)),
        }
    }
}
