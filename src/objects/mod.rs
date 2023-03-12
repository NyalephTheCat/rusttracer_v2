use crate::{
    engine::{bounding_box::BoundingBox, intersection::Intersection, ray::Ray},
    material::Material,
    utils::math::{point::Point, vector::Vector},
};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn normal(&self, point: &Point) -> Vector;
    fn material_at(&self, point: &Point) -> Material;
    fn bounding_box(&self) -> BoundingBox;
}

pub mod mesh;
pub mod plane;
pub mod smooth_triangle;
pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
    Triangle(triangle::Triangle),
    SmoothTriangle(smooth_triangle::SmoothTriangle),
    Mesh(mesh::Mesh),
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Plane(plane) => plane.intersect(ray),
            Object::Triangle(triangle) => triangle.intersect(ray),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.intersect(ray),
            Object::Mesh(mesh) => mesh.intersect(ray),
        }
    }

    fn normal(&self, point: &Point) -> Vector {
        match self {
            Object::Sphere(sphere) => sphere.normal(point),
            Object::Plane(plane) => plane.normal(point),
            Object::Triangle(triangle) => triangle.normal(point),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.normal(point),
            Object::Mesh(mesh) => mesh.normal(point),
        }
    }

    fn material_at(&self, point: &Point) -> Material {
        match self {
            Object::Sphere(sphere) => sphere.material_at(point),
            Object::Plane(plane) => plane.material_at(point),
            Object::Triangle(triangle) => triangle.material_at(point),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.material_at(point),
            Object::Mesh(mesh) => mesh.material_at(point),
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        match self {
            Object::Sphere(sphere) => sphere.bounding_box(),
            Object::Plane(plane) => plane.bounding_box(),
            Object::Triangle(triangle) => triangle.bounding_box(),
            Object::SmoothTriangle(smooth_triangle) => smooth_triangle.bounding_box(),
            Object::Mesh(mesh) => mesh.bounding_box(),
        }
    }
}
