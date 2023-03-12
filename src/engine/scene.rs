use image::RgbImage;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

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

        let mut color_each = self
            .camera
            .ray_par_iter((width as usize, height as usize))
            .progress_with(bar)
            .map(|(x, y, ray)| {
                let color = ray.cast(self.clone());
                (x, y, color)
            })
            .collect::<Vec<_>>();

        let local_contrast = color_each
            .iter()
            .map(|(x, y, color)| {
                let mut surrounding_colors = Vec::new();
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }
                        let x = *x as i32 + x_offset;
                        let y = *y as i32 + y_offset;
                        if x < 0 || y < 0 {
                            continue;
                        }
                        let x = x as u32;
                        let y = y as u32;
                        if x >= width || y >= height {
                            continue;
                        }
                        surrounding_colors.push(image.get_pixel(x, y));
                    }
                }

                let average_color = surrounding_colors
                    .iter()
                    .fold(Color::from((0.0, 0.0, 0.0)), |acc, color| {
                        acc + Into::<Color>::into(**color)
                    })
                    / surrounding_colors.len() as f64;

                color.contrast(average_color)
            })
            .collect::<Vec<_>>();

        // Apply anti-aliasing
        if self.camera.anti_aliasing > 1 {
            let bar = indicatif::ProgressBar::new((width * height).into());
            color_each = color_each
                .par_iter()
                .progress_with(bar)
                .map(|(x, y, color)| {
                    let rng = &mut rand::thread_rng();
                    if local_contrast[(x + y * width as usize)] > 0.5 {
                        (
                            *x,
                            *y,
                            self.camera
                                .ray_aa(
                                    (*x as f64, *y as f64),
                                    (width as usize, height as usize),
                                    self.camera.anti_aliasing as usize,
                                    rng,
                                )
                                .iter()
                                .fold(Color::zero(), |acc, ray| acc + ray.cast(self.clone()))
                                / self.camera.anti_aliasing as f64,
                        )
                    } else {
                        (*x, *y, *color)
                    }
                })
                .collect::<Vec<_>>();
        }

        for (x, y, color) in color_each {
            image.put_pixel(x as u32, y as u32, color.into());
        }
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            camera: Camera::default(),
            objects: Vec::new(),
            lights: Vec::new(),
            background: Color::from((0.0, 0.0, 0.0)),
        }
    }
}
