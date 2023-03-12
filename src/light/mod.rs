use crate::utils::{
    color::Color,
    math::{point::Point, vector::Vector},
};

use self::{directional::DirectionalLight, point::PointLight, spot::SpotLight};

pub trait Emittable {
    fn intensity(&self, point: Point) -> Color;
    fn direction(&self, point: Point) -> Vector;
    fn distance(&self, point: Point) -> f64;
}

pub mod directional;
pub mod point;
pub mod spot;

#[derive(Debug, Clone)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

impl Emittable for Light {
    fn intensity(&self, point: Point) -> Color {
        match self {
            Light::Directional(directional) => directional.intensity(point),
            Light::Point(point_light) => point_light.intensity(point),
            Light::Spot(spot) => spot.intensity(point),
        }
    }

    fn direction(&self, point: Point) -> Vector {
        match self {
            Light::Directional(directional) => directional.direction(point),
            Light::Point(point_light) => point_light.direction(point),
            Light::Spot(spot) => spot.direction(point),
        }
    }

    fn distance(&self, point: Point) -> f64 {
        match self {
            Light::Directional(directional) => directional.distance(point),
            Light::Point(point_light) => point_light.distance(point),
            Light::Spot(spot) => spot.distance(point),
        }
    }
}
