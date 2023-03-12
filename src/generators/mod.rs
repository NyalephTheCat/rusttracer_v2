use crate::objects::mesh::Mesh;

pub trait MeshGenerator {
    fn generate(&self) -> Mesh;
}

pub mod metaball;
