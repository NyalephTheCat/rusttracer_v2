use super::{Intersectable, Object};
use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::{
        texture::{Texturable, Texture},
        Material,
    },
    utils::math::{point::Point, vector::Vector},
};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Point; 3],
    pub texture: Texture,
}

impl Triangle {
    pub fn new(points: [Point; 3], texture: Texture) -> Triangle {
        Triangle { points, texture }
    }

    pub fn with_points(&self, points: [Point; 3]) -> Triangle {
        Triangle {
            points,
            ..self.clone()
        }
    }

    pub fn with_a(&self, a: Point) -> Triangle {
        let mut points = self.points;
        points[0] = a;
        Triangle {
            points,
            ..self.clone()
        }
    }

    pub fn with_b(&self, b: Point) -> Triangle {
        let mut points = self.points;
        points[1] = b;
        Triangle {
            points,
            ..self.clone()
        }
    }

    pub fn with_c(&self, c: Point) -> Triangle {
        let mut points = self.points;
        points[2] = c;
        Triangle {
            points,
            ..self.clone()
        }
    }

    pub fn with_texture(&self, texture: Texture) -> Triangle {
        Triangle {
            texture,
            ..self.clone()
        }
    }

    pub fn write_to_obj(&self, file: &mut std::fs::File) {
        for point in self.points.iter() {
            writeln!(file, "v {} {} {}", point.x, point.y, point.z).unwrap();
        }
        writeln!(file, "f {} {} {}", -3, -2, -1).unwrap();
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = ray.direction.cross(&e2);
        let a = e1.dot(&p);
        if a.abs() < 1e-6 {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin - self.points[0];
        let u = f * s.dot(&p);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * e2.dot(&q);
        if t > 1e-6 {
            Some(Intersection::new(t, ray, (self.clone()).into()))
        } else {
            None
        }
    }

    fn normal(&self, _point: &Point) -> Vector {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        e1.cross(&e2).normalize()
    }

    fn material_at(&self, point: &Point) -> Material {
        // Find uv coords in triangle
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = *point - self.points[0];
        let u = e1.dot(&p) / e1.dot(&e1);
        let v = e2.dot(&p) / e2.dot(&e2);

        self.texture.value(u, v)
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                self.points[0].x.min(self.points[1].x.min(self.points[2].x)),
                self.points[0].y.min(self.points[1].y.min(self.points[2].y)),
                self.points[0].z.min(self.points[1].z.min(self.points[2].z)),
            ),
            max: Point::new(
                self.points[0].x.max(self.points[1].x.max(self.points[2].x)),
                self.points[0].y.max(self.points[1].y.max(self.points[2].y)),
                self.points[0].z.max(self.points[1].z.max(self.points[2].z)),
            ),
        }
    }
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle {
            points: [
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
            ],
            texture: Texture::default(),
        }
    }
}

impl From<Triangle> for Object {
    fn from(triangle: Triangle) -> Object {
        Object::Triangle(triangle)
    }
}
