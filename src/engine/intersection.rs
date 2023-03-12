use crate::{
    material::Material,
    objects::{Intersectable, Object},
    utils::math::{point::Point, vector::Vector},
};

use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub point: Point,
    pub obj: Object,
    pub material: Material,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(distance: f64, ray: &Ray, obj: Object) -> Intersection {
        let material = obj
            .clone()
            .material_at(&(*&ray.origin + ray.direction * distance));
        let normal = obj
            .clone()
            .normal(&(*&ray.origin + ray.direction * distance));

        Intersection {
            distance,
            point: ray.origin + ray.direction * distance,
            obj,
            material,
            normal,
        }
    }
}
