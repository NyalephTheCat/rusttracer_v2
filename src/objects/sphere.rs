use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::{
        texture::{Texturable, Texture},
        Material,
    },
    utils::math::{point::Point, vector::Vector},
};

use super::{Intersectable, Object};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, texture: Texture) -> Sphere {
        Sphere {
            center,
            radius,
            texture,
        }
    }

    pub fn with_texture(&self, texture: Texture) -> Sphere {
        let mut sphere = self.clone();
        sphere.texture = texture;
        sphere
    }

    pub fn with_position(&self, position: Point) -> Sphere {
        let mut sphere = self.clone();
        sphere.center = position;
        sphere
    }

    pub fn with_radius(&self, radius: f64) -> Sphere {
        let mut sphere = self.clone();
        sphere.radius = radius;
        sphere
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }

        let t = if t1 < 0.0 {
            t2
        } else if t2 < 0.0 {
            t1
        } else {
            t1.min(t2)
        };

        Some(Intersection::new(t, ray, self.into()))
    }

    fn normal(&self, point: &Point) -> Vector {
        (*point - self.center).normalize()
    }

    fn material_at(&self, point: &Point) -> Material {
        let normal = self.normal(point);
        let phi = normal.z.atan2(normal.x);
        let theta = normal.y.asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;

        self.texture.value(u, v)
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            max: Point::new(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        }
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            texture: Texture::default(),
        }
    }
}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Object {
        Object::Sphere(sphere)
    }
}

impl From<&Sphere> for Object {
    fn from(sphere: &Sphere) -> Object {
        Object::Sphere(sphere.clone())
    }
}
