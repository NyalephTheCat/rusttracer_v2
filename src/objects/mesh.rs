use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::Material,
    utils::math::{point::Point, vector::Vector},
};

use super::{Intersectable, Object};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub faces: Vec<Object>,
    pub bounding_box: BoundingBox,
}

impl Mesh {
    pub fn new(faces: Vec<Object>) -> Mesh {
        let bounding_box = BoundingBox::from_objects(&faces);
        Mesh {
            faces,
            bounding_box,
        }
    }

    pub fn with_faces(&self, faces: Vec<Object>) -> Mesh {
        let bounding_box = BoundingBox::from_objects(&faces);
        Mesh {
            faces,
            bounding_box,
        }
    }

    pub fn with_bounding_box(&self, bounding_box: BoundingBox) -> Mesh {
        Mesh {
            bounding_box,
            ..self.clone()
        }
    }

    pub fn with_face(&self, face: Object) -> Mesh {
        let mut faces = self.faces.clone();
        faces.push(face);
        let bounding_box = BoundingBox::from_objects(&faces);
        Mesh {
            faces,
            bounding_box,
        }
    }

    pub fn write_to_obj(&self, file: &mut std::fs::File) {
        self.faces.iter().for_each(|face| match face {
            Object::Triangle(triangle) => triangle.write_to_obj(file),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.write_to_obj(file),
            Object::Mesh(mesh) => {
                mesh.write_to_obj(file);
            }
            _ => {}
        });
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.faces.iter().fold(None, |inter, face| {
            let face_inter = face.intersect(ray);
            match (inter, face_inter) {
                (None, None) => None,
                (Some(inter), None) => Some(inter),
                (None, Some(inter)) => Some(inter),
                (Some(inter1), Some(inter2)) => {
                    if inter1.distance < inter2.distance {
                        Some(inter1)
                    } else {
                        Some(inter2)
                    }
                }
            }
        })
    }

    fn normal(&self, _point: &Point) -> Vector {
        unimplemented!("Should get normal on face instead of mesh")
    }

    fn material_at(&self, _point: &Point) -> Material {
        unimplemented!("Should get material on face instead of mesh")
    }

    fn bounding_box(&self) -> BoundingBox {
        unimplemented!()
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            faces: Vec::new(),
            bounding_box: BoundingBox::empty(),
        }
    }
}

impl From<&Mesh> for Object {
    fn from(mesh: &Mesh) -> Object {
        Object::Mesh(mesh.clone())
    }
}
