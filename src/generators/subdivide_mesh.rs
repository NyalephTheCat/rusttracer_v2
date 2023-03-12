use crate::objects::{mesh::Mesh, Object};

use super::MeshGenerator;

#[derive(Debug, Clone, Default)]
pub struct SubdivideMesh {
    pub mesh: Mesh,
    pub depth: u32,
}

impl SubdivideMesh {
    pub fn new(mesh: Mesh, depth: u32) -> SubdivideMesh {
        SubdivideMesh {
            mesh,
            depth,
        }
    }

    pub fn with_mesh(&self, mesh: Mesh) -> SubdivideMesh {
        SubdivideMesh {
            mesh,
            ..self.clone()
        }
    }

    pub fn with_depth(&self, depth: u32) -> SubdivideMesh {
        SubdivideMesh {
            depth,
            ..self.clone()
        }
    }
}

impl MeshGenerator for SubdivideMesh {
    fn generate(&self) -> Mesh {
        let mut objects = Vec::<Object>::new();
        
        self.mesh.faces.iter().for_each(|face| {
            match face {
                Object::Mesh(_) => {},
                t => objects.push(t.clone())
            }
        });

        Mesh::default().with_faces(objects)
    }
}