use matrix::cross_product;
use matrix::identity_matrix;
use matrix::GLMatrix;
use matrix::MatrixTransform;
use std::path::Path;
use tobj;
use vertex::Vertex;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ObjModel {
    vertex_data: Vec<Vertex>,
    model: GLMatrix,
}

#[allow(dead_code)]
impl ObjModel {
    fn new(path: &str) -> Self {
        let (models, _materials) = tobj::load_obj(Path::new(path)).unwrap();

        let mut vertex_array = Vec::new();

        for model in &models {
            for index in &model.mesh.indices {
                let x = *(&model.mesh.positions[3 * *index as usize]);
                let y = *(&model.mesh.positions[1 + 3 * *index as usize]);
                let z = *(&model.mesh.positions[2 + 3 * *index as usize]);
                let w = 1f32;

                let maybe_normal = if !*(&model.mesh.normals.is_empty()) {
                    [
                        *(&model.mesh.normals[3 * *index as usize]),
                        *(&model.mesh.normals[1 + 3 * *index as usize]),
                        *(&model.mesh.normals[2 + 3 * *index as usize]),
                    ]
                } else {
                    [0f32, 0f32, 0f32]
                };

                vertex_array.push(Vertex {
                    pos: [x, y, z, w],
                    normal: maybe_normal,
                });
            }
        }
        ObjModel {
            vertex_data: vertex_array,
            model: identity_matrix(),
        }
    }

    fn compute_normal(p1: &glm::Vec4, p2: &glm::Vec4, p3: &glm::Vec4) -> glm::Vec4 {
        let u = *p3 - *p1;
        let v = *p2 - *p1;
        cross_product(u, v)
    }
}

impl MatrixTransform for ObjModel {
    fn get_matrix(&self) -> GLMatrix {
        self.model
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) {
        self.model = matrix.clone();
    }
    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        ObjModel {
            model: matrix.clone(),
            vertex_data: self.vertex_data.clone(),
        }
    }
}
