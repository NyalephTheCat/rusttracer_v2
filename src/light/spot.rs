use crate::utils::{
    color::Color,
    math::{point::Point, vector::Vector},
};

use super::{Emittable, Light};

#[derive(Debug, Clone)]
pub struct SpotLight {
    pub position: Point,
    pub direction: Vector,
    pub color: Color,
    pub intensity: f64,
    pub cutoff: f64,
    pub falloff: f64,
}

impl SpotLight {
    pub fn new(
        position: Point,
        direction: Vector,
        color: Color,
        intensity: f64,
        cutoff: f64,
        falloff: f64,
    ) -> SpotLight {
        SpotLight {
            position,
            direction,
            color,
            intensity,
            cutoff,
            falloff,
        }
    }

    pub fn default() -> SpotLight {
        SpotLight {
            position: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, -1.0),
            color: Color::from((1.0, 1.0, 1.0)),
            intensity: 1.0,
            cutoff: 0.0,
            falloff: 0.0,
        }
    }

    pub fn with_position(&self, position: Point) -> SpotLight {
        let mut light = self.clone();
        light.position = position;
        light
    }

    pub fn with_direction(&self, direction: Vector) -> SpotLight {
        let mut light = self.clone();
        light.direction = direction;
        light
    }

    pub fn with_color(&self, color: Color) -> SpotLight {
        let mut light = self.clone();
        light.color = color;
        light
    }

    pub fn with_intensity(&self, intensity: f64) -> SpotLight {
        let mut light = self.clone();
        light.intensity = intensity;
        light
    }

    pub fn with_cutoff(&self, cutoff: f64) -> SpotLight {
        let mut light = self.clone();
        light.cutoff = cutoff;
        light
    }

    pub fn with_falloff(&self, falloff: f64) -> SpotLight {
        let mut light = self.clone();
        light.falloff = falloff;
        light
    }
}

impl Emittable for SpotLight {
    fn intensity(&self, point: Point) -> Color {
        let distance = self.position.distance(&point);
        let intensity = self.intensity / (distance * distance);
        let direction = (self.position - point).normalize();
        let angle = self.direction.dot(&direction);
        let angle = if angle < self.cutoff { 0.0 } else { angle };
        let angle = if angle > self.falloff { 1.0 } else { angle };
        self.color * intensity * angle
    }

    fn direction(&self, point: Point) -> Vector {
        (self.position - point).normalize()
    }

    fn distance(&self, point: Point) -> f64 {
        self.position.distance(&point)
    }
}

impl From<SpotLight> for Light {
    fn from(spot_light: SpotLight) -> Light {
        Light::Spot(spot_light)
    }
}

impl Default for SpotLight {
    fn default() -> SpotLight {
        SpotLight::default()
    }
}
