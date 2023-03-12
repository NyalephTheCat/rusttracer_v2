use crate::utils::math::{point::Point, vector::Vector};

use super::ray::Ray;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point,
    pub direction: Vector,
    pub up: Vector,
    pub right: Vector,
    pub width_fov: f64,
    pub height_fov: f64,
    pub z_min: f64,
    pub max_bounces: u32,
}

impl Camera {
    pub fn new(
        position: Point,
        direction: Vector,
        up: Vector,
        alpha: f64,
        beta: f64,
        z_min: f64,
        max_bounces: u32,
    ) -> Camera {
        let right = direction.cross(&up);
        let height_fov = (beta.to_radians() / 2.0).tan();
        let width_fov = (alpha.to_radians() / 2.0).tan();
        Camera {
            position,
            direction,
            up,
            right,
            width_fov,
            height_fov,
            z_min,
            max_bounces,
        }
    }

    pub fn default() -> Camera {
        let height_fov = (60.0_f64.to_radians() / 2.0).tan();
        let width_fov = (60.0_f64.to_radians() / 2.0).tan();

        Camera {
            position: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
            up: Vector::new(0.0, 1.0, 0.0),
            right: Vector::new(0.0, 0.0, 1.0),
            width_fov,
            height_fov,
            z_min: 1.0,
            max_bounces: 5,
        }
    }

    pub fn with_position(&self, position: Point) -> Camera {
        let mut camera = self.clone();
        camera.position = position;
        camera
    }

    pub fn with_direction(&self, direction: Vector) -> Camera {
        let mut camera = self.clone();
        camera.direction = direction.normalize();
        camera.right = camera.direction.cross(&camera.up).normalize();
        camera
    }

    pub fn with_up(&self, up: Vector) -> Camera {
        let mut camera = self.clone();
        camera.up = up.normalize();
        camera.right = camera.direction.cross(&camera.up).normalize();
        camera
    }

    pub fn with_fov(&self, alpha: f64, beta: f64) -> Camera {
        let mut camera = self.clone();
        camera.width_fov = (alpha.to_radians() / 2.0).tan();
        camera.height_fov = (beta.to_radians() / 2.0).tan();
        camera
    }

    pub fn with_z_min(&self, z_min: f64) -> Camera {
        let mut camera = self.clone();
        camera.z_min = z_min;
        camera
    }

    pub fn with_max_bounces(&self, max_bounces: u32) -> Camera {
        let mut camera = self.clone();
        camera.max_bounces = max_bounces;
        camera
    }

    fn ray_inner(&self, coord: (f64, f64), offset: (f64, f64), image_size: (usize, usize)) -> Ray {
        let (x, y) = coord;
        let (x_offset, y_offset) = offset;
        let (width, height) = image_size;

        let x = (x + x_offset) / width as f64;
        let y = (y + y_offset) / height as f64;

        let x = (2.0 * x - 1.0) * self.width_fov;
        let y = (1.0 - 2.0 * y) * self.height_fov;

        let direction = self.direction * self.z_min + self.right * x + self.up * y;
        Ray::new(self.position, direction.normalize(), 0)
    }

    pub fn ray(&self, coord: (f64, f64), image_size: (usize, usize)) -> Ray {
        self.ray_inner(coord, (0.5, 0.5), image_size)
    }

    pub fn ray_aa(
        &self,
        coord: (f64, f64),
        image_size: (usize, usize),
        aa_samples: usize,
    ) -> Vec<Ray> {
        (0..aa_samples)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let x_offset = rng.gen_range(0.0..1.0);
                let y_offset = rng.gen_range(0.0..1.0);
                self.ray_inner(coord, (x_offset, y_offset), image_size)
            })
            .collect::<Vec<_>>()
    }

    pub fn ray_iter(
        &self,
        image_size: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize, Ray)> + '_ {
        let (width, height) = image_size;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .map(move |(x, y)| (x, y, self.ray((x as f64, y as f64), image_size)))
    }

    pub fn ray_par_iter(
        &self,
        image_size: (usize, usize),
    ) -> impl ParallelIterator<Item = (usize, usize, Ray)> + '_ {
        let (width, height) = image_size;
        (0..height)
            .into_par_iter()
            .flat_map(move |y| (0..width).into_par_iter().map(move |x| (x, y)))
            .map(move |(x, y)| (x, y, self.ray((x as f64, y as f64), image_size)))
    }
}
