use crate::utils::{
    color::Color,
    math::{point::Point, vector::Vector},
};

use super::{Emittable, Light};

#[derive(Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vector, color: Color, intensity: f64) -> DirectionalLight {
        DirectionalLight {
            direction,
            color,
            intensity,
        }
    }

    pub fn with_direction(&self, direction: Vector) -> DirectionalLight {
        let mut light = self.clone();
        light.direction = direction;
        light
    }

    pub fn with_color(&self, color: Color) -> DirectionalLight {
        let mut light = self.clone();
        light.color = color;
        light
    }

    pub fn with_intensity(&self, intensity: f64) -> DirectionalLight {
        let mut light = self.clone();
        light.intensity = intensity;
        light
    }
}

impl Emittable for DirectionalLight {
    fn intensity(&self, _point: Point) -> Color {
        self.color * self.intensity
    }

    fn direction(&self, _point: Point) -> Vector {
        self.direction
    }

    fn distance(&self, _point: Point) -> f64 {
        f64::INFINITY
    }
}

impl From<DirectionalLight> for Light {
    fn from(directional: DirectionalLight) -> Light {
        Light::Directional(directional)
    }
}

impl Default for DirectionalLight {
    fn default() -> DirectionalLight {
        DirectionalLight {
            direction: Vector::new(0.0, 0.0, -1.0),
            color: Color::from((1.0, 1.0, 1.0)),
            intensity: 1.0,
        }
    }
}
