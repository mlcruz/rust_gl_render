use models::matrix::camera_view_matrix;
use models::matrix::normalize_vector;
use models::matrix::GLMatrix;

#[derive(Debug, Copy)]
pub struct FreeCamera {
    pub pos: glm::Vec4,
    pub view_matrix: GLMatrix,
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl FreeCamera {
    pub fn new(pos: glm::Vec3, pitch: &f32, yaw: &f32) -> Self {
        // Inicializa vetores para calculo de matriz de view da camera
        let up_world = glm::vec4(0.0, 1.0, 0.0, 0.0);

        // Ponto inicial e distancia
        let pos_as_point = glm::vec4(pos.x, pos.y, pos.z, 1.0);
        let distance = 2.5;

        // Cria vetor de direção da camera conforme angulos phi e theta (pitch e yaw) fornecidos
        let front = normalize_vector(glm::vec4(
            glm::cos(*yaw) * glm::cos(*pitch),
            glm::sin(*pitch),
            glm::sin(*yaw) * glm::cos(*pitch),
            0.0,
        ));

        FreeCamera {
            pos: pos_as_point,
            distance,
            pitch: 0.0,
            yaw: 0.0,
            view_matrix: camera_view_matrix(pos_as_point, front * distance, up_world),
        }
    }

    pub fn refresh(&mut self) -> &Self {
        let pos_as_point = glm::vec4(self.pos.x, self.pos.y, self.pos.z, 1.0);
        let up_world = glm::vec4(0.0, 1.0, 0.0, 0.0);

        let front = normalize_vector(glm::vec4(
            glm::cos(self.yaw) * glm::cos(self.pitch),
            glm::sin(self.pitch),
            glm::sin(self.yaw) * glm::cos(self.pitch),
            0.0,
        ));

        self.view_matrix = camera_view_matrix(pos_as_point, front * self.distance, up_world);
        self
    }
}

impl Clone for FreeCamera {
    fn clone(&self) -> Self {
        *self
    }
}
