use crate::material::Material;

use super::{Texturable, Texture};

#[derive(Debug, Clone, Copy, Default)]
pub struct UniformTexture {
    pub material: Material,
}

impl UniformTexture {
    pub fn new(material: Material) -> UniformTexture {
        UniformTexture { material }
    }
}

impl Texturable for UniformTexture {
    fn value(self, _u: f64, _v: f64) -> Material {
        self.material
    }
}

impl From<UniformTexture> for Texture {
    fn from(texture: UniformTexture) -> Texture {
        Texture::Uniform(texture)
    }
}
