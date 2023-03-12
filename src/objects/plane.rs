use super::{Intersectable, Object};
use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::{
        texture::{Texturable, Texture},
        Material,
    },
    utils::math::{point::Point, vector::Vector},
};

#[derive(Debug, Clone)]
pub struct Plane {
    pub position: Point,
    pub normal: Vector,
    pub texture: Texture,
    pub scale: f64,
}

impl Plane {
    pub fn new(position: Point, normal: Vector, texture: Texture, scale: f64) -> Plane {
        Plane {
            position,
            normal,
            texture,
            scale,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let denominator = self.normal.dot(&ray.direction);
        if denominator > 0.0 {
            return None;
        }
        let numerator = self.normal.dot(&(ray.origin - self.position));
        let t = numerator / denominator;
        if t < 0.0 {
            return None;
        }
        println!("Plane intersection: {:?}", self.position);
        Some(Intersection::new(t, ray, self.into()))
    }

    fn normal(&self, _point: &Point) -> Vector {
        self.normal
    }

    fn material_at(&self, point: &Point) -> Material {
        let u = (point.x - self.position.x) / self.scale;
        let v = (point.z - self.position.z) / self.scale;
        self.texture.value(u, v)
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        }
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane {
            position: Point::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 1.0, 0.0),
            texture: Texture::default(),
            scale: 1.0,
        }
    }
}

impl From<&Plane> for Object {
    fn from(plane: &Plane) -> Object {
        Object::Plane(plane.clone())
    }
}
