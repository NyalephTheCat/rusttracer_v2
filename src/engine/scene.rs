use image::RgbImage;
use rayon::prelude::ParallelIterator;

use crate::{
    light::Light,
    objects::{Intersectable, Object},
    utils::color::Color,
};

use super::{camera::Camera, intersection::Intersection, ray::Ray};

use indicatif::ParallelProgressIterator;

#[derive(Debug, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub background: Color,
}

impl Scene {
    pub fn new(
        camera: Camera,
        objects: Vec<Object>,
        lights: Vec<Light>,
        background: Color,
    ) -> Scene {
        Scene {
            camera,
            objects,
            lights,
            background,
        }
    }

    pub fn default() -> Scene {
        Scene {
            camera: Camera::default(),
            objects: Vec::new(),
            lights: Vec::new(),
            background: Color::from((0.0, 0.0, 0.0)),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn with_camera(&self, camera: Camera) -> Scene {
        let mut scene = self.clone();
        scene.camera = camera;
        scene
    }

    pub fn with_objects(&self, objects: Vec<Object>) -> Scene {
        let mut scene = self.clone();
        scene.objects = objects;
        scene
    }

    pub fn with_lights(&self, lights: Vec<Light>) -> Scene {
        let mut scene = self.clone();
        scene.lights = lights;
        scene
    }

    pub fn with_background(&self, background: Color) -> Scene {
        let mut scene = self.clone();
        scene.background = background;
        scene
    }

    pub fn with_object(&self, object: Object) -> Scene {
        let mut scene = self.clone();
        scene.add_object(object);
        scene
    }

    pub fn with_light(&self, light: Light) -> Scene {
        let mut scene = self.clone();
        scene.add_light(light);
        scene
    }

    pub fn trace(&self, ray: Ray) -> Option<Intersection> {
        self.objects.iter().fold(None, |iter, object| {
            let object_inter = object.intersect(&ray);
            match (iter, object_inter) {
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

    pub fn render_into(&self, image: &mut RgbImage) {
        let (width, height) = image.dimensions();
        let bar = indicatif::ProgressBar::new((width * height).into());
        let color_each = self
            .camera
            .ray_par_iter((width as usize, height as usize))
            .progress_with(bar)
            .map(|(x, y, ray)| {
                let color = ray.cast(self.clone());
                (x, y, color)
            })
            .collect::<Vec<_>>();
        for (x, y, color) in color_each {
            image.put_pixel(x as u32, y as u32, color.into());
        }
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::default()
    }
}
