use crate::utils::{color::Color, color_or_float::ColorOrFloat};

pub mod texture;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub name: &'static str,
    pub color: Color,
    pub ambient: ColorOrFloat,
    pub reflection: ColorOrFloat,
    pub diffuse: ColorOrFloat,
    pub specular: ColorOrFloat,
    pub specular_exponent: f64,
    pub transparency: ColorOrFloat,
    pub refraction_index: f64,
}

impl Material {
    pub fn new(
        name: &'static str,
        color: Color,
        ambient: ColorOrFloat,
        reflection: ColorOrFloat,
        diffuse: ColorOrFloat,
        specular: ColorOrFloat,
        specular_exponent: f64,
        transparency: ColorOrFloat,
        refraction_index: f64,
    ) -> Material {
        Material {
            name,
            color,
            ambient,
            reflection,
            diffuse,
            specular,
            specular_exponent,
            transparency,
            refraction_index,
        }
    }

    pub fn with_name(&self, name: &'static str) -> Material {
        let mut material = *self;
        material.name = name;
        material
    }

    pub fn with_color(&self, color: Color) -> Material {
        let mut material = *self;
        material.color = color;
        material
    }

    pub fn with_ambient(&self, ambient: ColorOrFloat) -> Material {
        let mut material = *self;
        material.ambient = ambient;
        material
    }

    pub fn with_reflection(&self, reflection: ColorOrFloat) -> Material {
        let mut material = *self;
        material.reflection = reflection;
        material
    }

    pub fn with_diffuse(&self, diffuse: ColorOrFloat) -> Material {
        let mut material = *self;
        material.diffuse = diffuse;
        material
    }

    pub fn with_specular(&self, specular: ColorOrFloat, specular_exponent: f64) -> Material {
        let mut material = *self;
        material.specular = specular;
        material.specular_exponent = specular_exponent;
        material
    }
    pub fn with_transparency(&self, transparency: ColorOrFloat, refraction_index: f64) -> Material {
        let mut material = *self;
        material.transparency = transparency;
        material.refraction_index = refraction_index;
        material
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            name: "default",
            color: "#FFFFFF".into(),
            ambient: ColorOrFloat::Float(0.02),
            reflection: ColorOrFloat::Float(0.01),
            diffuse: ColorOrFloat::Float(1.0),
            specular: ColorOrFloat::Float(0.5),
            specular_exponent: 10.0,
            transparency: ColorOrFloat::Float(0.0),
            refraction_index: 1.5,
        }
    }
}
