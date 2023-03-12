use std::ops::{Add, Div, Mul, Sub};

use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const GAMMA: f64 = 2.2;

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn zero() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        fn gamma_correct(color: f64) -> f64 {
            color.powf(1.0 / Color::GAMMA)
        }

        (
            (gamma_correct(self.r) * 255.0) as u8,
            (gamma_correct(self.g) * 255.0) as u8,
            (gamma_correct(self.b) * 255.0) as u8,
        )
    }

    pub fn to_grayscale(&self) -> f64 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    pub fn from_hex(hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Color::from((r, g, b))
    }

    pub fn relative_luminance(&self) -> f64 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    pub fn contrast(&self, color: Color) -> f64 {
        let l1 = self.relative_luminance();
        let l2 = color.relative_luminance();
        if l1 > l2 {
            (l1 + 0.05) / (l2 + 0.05)
        } else {
            (l2 + 0.05) / (l1 + 0.05)
        }
    }
}

impl From<&str> for Color {
    fn from(hex: &str) -> Color {
        Color::from_hex(hex)
    }
}

impl From<Rgb<u8>> for Color {
    fn from(rgb: Rgb<u8>) -> Color {
        Color::from((rgb[0], rgb[1], rgb[2]))
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(tuple: (u8, u8, u8)) -> Color {
        fn gamma_uncorrect(color: u8) -> f64 {
            let color = color as f64 / 255.0;
            color.powf(Color::GAMMA)
        }

        Color {
            r: gamma_uncorrect(tuple.0),
            g: gamma_uncorrect(tuple.1),
            b: gamma_uncorrect(tuple.2),
        }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8) {
        color.to_rgb()
    }
}

impl From<[u8; 3]> for Color {
    fn from(array: [u8; 3]) -> Color {
        let colors = (array[0], array[1], array[2]);
        Color::from(colors)
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        let (r, g, b) = color.to_rgb();
        [r, g, b]
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(tuple: (f64, f64, f64)) -> Color {
        Color {
            r: tuple.0,
            g: tuple.1,
            b: tuple.2,
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Rgb<u8> {
        let (r, g, b) = color.to_rgb();
        Rgb([r, g, b])
    }
}

impl From<Color> for (f64, f64, f64) {
    fn from(color: Color) -> (f64, f64, f64) {
        (color.r, color.g, color.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Add<f64> for Color {
    type Output = Color;

    fn add(self, other: f64) -> Color {
        Color {
            r: self.r + other,
            g: self.g + other,
            b: self.b + other,
        }
    }
}

impl Add<Color> for f64 {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self + other.r,
            g: self + other.g,
            b: self + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Sub<f64> for Color {
    type Output = Color;

    fn sub(self, other: f64) -> Color {
        Color {
            r: self.r - other,
            g: self.g - other,
            b: self.b - other,
        }
    }
}

impl Sub<Color> for f64 {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self - other.r,
            g: self - other.g,
            b: self - other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}
