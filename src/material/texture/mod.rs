use self::{checkerboard::CheckerboardTexture, uniform::UniformTexture};

use super::Material;

pub mod checkerboard;
pub mod uniform;

pub trait Texturable {
    fn value(self, u: f64, v: f64) -> Material;
}

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Uniform(UniformTexture),
    Checkerboard(CheckerboardTexture),
}

impl Texture {
    pub fn new_uniform(material: Material) -> Texture {
        Texture::Uniform(UniformTexture::new(material))
    }
}

impl Default for Texture {
    fn default() -> Texture {
        Texture::Uniform(UniformTexture::default())
    }
}

impl Texturable for Texture {
    fn value(self, u: f64, v: f64) -> Material {
        match self {
            Texture::Uniform(texture) => texture.value(u, v),
            Texture::Checkerboard(texture) => texture.value(u, v),
        }
    }
}
