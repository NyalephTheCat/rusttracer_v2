use crate::light::Emittable;
use crate::utils::{
    color::Color,
    math::{point::Point, vector::Vector},
};

use super::scene::Scene;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub depth: u32,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, depth: u32) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
            depth,
        }
    }

    pub fn point_at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }

    pub fn cast(&self, scene: Scene) -> Color {
        if self.depth > scene.camera.max_bounces {
            return scene.background;
        }

        //println!("{:?}", self);

        scene
            .trace(self.clone())
            .map(|intersection| {
                let reflected = if Into::<f64>::into(intersection.material.reflection) == 0.0 {
                    Color::zero()
                } else {
                    Ray::new(
                        intersection.point + intersection.normal * 1e-4,
                        self.direction.reflect(&intersection.normal),
                        self.depth + 1,
                    )
                    .cast(scene.clone())
                        * intersection.material.reflection
                }
                .clamp();

                let light_color: Color = scene
                    .lights
                    .iter()
                    .filter(|light| {
                        let shadow_ray = Ray::new(
                            intersection.point + intersection.normal * 1e-4,
                            light.direction(intersection.point),
                            0,
                        );
                        scene.trace(shadow_ray).is_none()
                    })
                    .fold(
                        intersection.material.color * intersection.material.ambient,
                        |color, light| {
                            let diffuse = intersection
                                .normal
                                .dot(&light.direction(intersection.point))
                                * light.intensity(intersection.point)
                                * intersection.material.color
                                * intersection.material.diffuse;

                            let specular = self
                                .direction
                                .reflect(&intersection.normal)
                                .dot(&light.direction(intersection.point))
                                .powf(intersection.material.specular_exponent)
                                * light.intensity(intersection.point)
                                * intersection.material.specular;

                            let color = color + diffuse;
                            let color = color + specular;

                            color.clamp()
                        },
                    );

                light_color + reflected
            })
            .unwrap_or(scene.background)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
            depth: 0,
        }
    }
}
