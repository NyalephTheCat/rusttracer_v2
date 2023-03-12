use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::texture::{Texturable, Texture},
    utils::math::{point::Point, vector::Vector},
};
use std::io::Write;
use super::{Intersectable, Object};

#[derive(Debug, Clone)]
pub struct SmoothTriangle {
    pub points: [Point; 3],
    pub normals: Vec<Vector>,
    pub texture: Texture,
}

impl SmoothTriangle {
    pub fn new(points: [Point; 3], normals: [Vector; 3], texture: Texture) -> SmoothTriangle {
        SmoothTriangle {
            points,
            normals: normals.to_vec(),
            texture,
        }
    }

    pub fn default() -> SmoothTriangle {
        SmoothTriangle {
            points: [
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
            ],
            normals: vec![
                Vector::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 0.0),
            ],
            texture: Texture::default(),
        }
    }

    pub fn with_points(&self, points: [Point; 3]) -> SmoothTriangle {
        SmoothTriangle {
            points,
            ..self.clone()
        }
    }

    pub fn with_normals(&self, normals: Vec<Vector>) -> SmoothTriangle {
        SmoothTriangle {
            normals,
            ..self.clone()
        }
    }

    pub fn with_texture(&self, texture: Texture) -> SmoothTriangle {
        SmoothTriangle {
            texture,
            ..self.clone()
        }
    }

    pub fn with_a(&self, a: Point, a_n: Vector) -> SmoothTriangle {
        let mut points = self.points.clone();
        points[0] = a;
        let mut normals = self.normals.clone();
        normals[0] = a_n;
        SmoothTriangle {
            points,
            normals,
            ..self.clone()
        }
    }

    pub fn with_b(&self, b: Point, b_n: Vector) -> SmoothTriangle {
        let mut points = self.points.clone();
        points[1] = b;
        let mut normals = self.normals.clone();
        normals[1] = b_n;
        SmoothTriangle {
            points,
            normals,
            ..self.clone()
        }
    }

    pub fn with_c(&self, c: Point, c_n: Vector) -> SmoothTriangle {
        let mut points = self.points.clone();
        points[2] = c;
        let mut normals = self.normals.clone();
        normals[2] = c_n;
        SmoothTriangle {
            points,
            normals,
            ..self.clone()
        }
    }

    pub fn write_to_obj(&self, file: &mut std::fs::File) {
        for point in self.points.iter() {
            writeln!(file, "v {} {} {}", point.x, point.y, point.z).unwrap();
        }
        for normal in self.normals.iter() {
            writeln!(file, "vn {} {} {}", normal.x, normal.y, normal.z).unwrap();
        }
        writeln!(file, "f -1//-1 -2//-2 -3//-3").unwrap();
    }
}

impl Intersectable for SmoothTriangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = ray.direction.cross(&e2);

        let det = e1.dot(&p);
        if det.abs() < 1e-8 {
            return None;
        }

        let inv_det = 1.0 / det;
        let t = ray.origin - self.points[0];
        let u = t.dot(&p) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = t.cross(&e1);
        let v = ray.direction.dot(&q) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = e2.dot(&q) * inv_det;
        if t < 0.0 {
            return None;
        }

        Some(Intersection::new(t, ray, self.clone().into()))
    }

    fn normal(&self, point: &Point) -> Vector {
        // Compute normal using Pheng's method
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = *point - self.points[0];
        let d00 = e1.dot(&e1);
        let d01 = e1.dot(&e2);
        let d11 = e2.dot(&e2);
        let d20 = p.dot(&e1);
        let d21 = p.dot(&e2);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        let normal = self.normals[0] * u + self.normals[1] * v + self.normals[2] * w;
        normal.normalize()
    }

    fn material_at(&self, point: &Point) -> crate::material::Material {
        // Get uv coods at point on triangle
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = *point - self.points[0];
        let d00 = e1.dot(&e1);
        let d01 = e1.dot(&e2);
        let d11 = e2.dot(&e2);
        let d20 = p.dot(&e1);
        let d21 = p.dot(&e2);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        // Get material at point on triangle
        self.texture.value(u, v)
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut min = Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for point in self.points.iter() {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
        }
        BoundingBox::new(min, max)
    }
}

impl Default for SmoothTriangle {
    fn default() -> SmoothTriangle {
        SmoothTriangle::default()
    }
}

impl From<SmoothTriangle> for Object {
    fn from(triangle: SmoothTriangle) -> Object {
        Object::SmoothTriangle(triangle.clone())
    }
}
