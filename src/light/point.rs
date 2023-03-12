use crate::utils::{
    color::Color,
    math::{point::Point, vector::Vector},
};

use super::{Emittable, Light};

#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Point, color: Color, intensity: f64) -> PointLight {
        PointLight {
            position,
            color,
            intensity,
        }
    }
    pub fn with_position(&self, position: Point) -> PointLight {
        let mut light = self.clone();
        light.position = position;
        light
    }

    pub fn with_color(&self, color: Color) -> PointLight {
        let mut light = self.clone();
        light.color = color;
        light
    }

    pub fn with_intensity(&self, intensity: f64) -> PointLight {
        let mut light = self.clone();
        light.intensity = intensity;
        light
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            position: Point::new(0.0, 0.0, 0.0),
            color: "#FFFFFF".into(),
            intensity: 10.0,
        }
    }
}

impl Emittable for PointLight {
    fn intensity(&self, point: Point) -> Color {
        let distance = (self.position - point).length();
        let intensity = self.intensity / distance;
        self.color * intensity
    }

    fn direction(&self, point: Point) -> Vector {
        (self.position - point).normalize()
    }

    fn distance(&self, point: Point) -> f64 {
        self.position.distance(&point)
    }
}

impl From<PointLight> for Light {
    fn from(point_light: PointLight) -> Light {
        Light::Point(point_light)
    }
}
