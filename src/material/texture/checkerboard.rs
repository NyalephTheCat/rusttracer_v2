use crate::material::Material;

use super::{Texturable, Texture};

#[derive(Debug, Clone, Copy)]
pub struct CheckerboardTexture {
    pub material1: Material,
    pub material2: Material,
    pub scale: f64,
}

impl CheckerboardTexture {
    pub fn new(material1: Material, material2: Material, scale: f64) -> CheckerboardTexture {
        CheckerboardTexture {
            material1,
            material2,
            scale,
        }
    }

    pub fn default() -> CheckerboardTexture {
        CheckerboardTexture {
            material1: Material::default(),
            material2: Material::default(),
            scale: 1.0,
        }
    }

    pub fn with_material1(self, material1: Material) -> CheckerboardTexture {
        CheckerboardTexture {
            material1,
            ..self
        }
    }

    pub fn with_material2(self, material2: Material) -> CheckerboardTexture {
        CheckerboardTexture {
            material2,
            ..self
        }
    }

    pub fn with_scale(self, scale: f64) -> CheckerboardTexture {
        CheckerboardTexture { scale, ..self }
    }

    pub fn with_materials(self, material1: Material, material2: Material) -> CheckerboardTexture {
        CheckerboardTexture {
            material1,
            material2,
            ..self
        }
    }
}

impl Texturable for CheckerboardTexture {
    fn value(self, u: f64, v: f64) -> Material {
        let sines = (self.scale * u).sin() * (self.scale * v).sin();
        if sines < 0.0 {
            self.material1
        } else {
            self.material2
        }
    }
}

impl Default for CheckerboardTexture {
    fn default() -> CheckerboardTexture {
        CheckerboardTexture::default()
    }
}

impl From<CheckerboardTexture> for Texture {
    fn from(texture: CheckerboardTexture) -> Texture {
        Texture::Checkerboard(texture)
    }
}