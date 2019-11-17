use glm::cos;
use glm::sin;

#[derive(Debug, Copy)]
pub struct Camera {
    theta: f32,    // Ângulo no plano ZX em relação ao eixo Z
    phi: f32,      // Ângulo em relação ao eixo Y
    distance: f32, // Distância da câmera para a origem

    // Posição da câmera utilizando coordenadas esféricas.
    y: f32,
    z: f32,
    x: f32,

    pub position: glm::Vector4<f32>,    // Ponto "c", centro da câmera
    pub lookat: glm::Vector4<f32>, // Ponto "l", para onde a câmera (look-at) estará sempre olhando
    pub view_vector: glm::Vector4<f32>, // Vetor "view", sentido para onde a câmera está virada
    pub up_vector: glm::Vector4<f32>, // Vetor "up" fixado para apontar para o "céu" (eito Y global)
}

#[allow(dead_code)]
impl Camera {
    pub fn new(theta: f32, phi: f32, distance: f32) -> Self {
        let x = distance * cos(phi) * sin(theta);
        let y = distance * sin(phi);
        let z = distance * cos(phi) * cos(theta);
        let position = glm::vec4(x, y, z, 1.0);
        let lookat = glm::vec4(0.0, 0.0, 0.0, 1.0); // Ponto "l", para onde a câmera (look-at) estará sempre olhando

        Camera {
            theta,
            phi,
            distance,
            x,
            y,
            z,
            position,
            lookat,
            view_vector: lookat - position, // Vetor "view", sentido para onde a câmera está virada
            up_vector: glm::vec4(0.0, 1.0, 0.0, 0.0), // Vetor "up" fixado para apontar para o "céu" (eito Y global)
        }
    }

    pub fn update(&mut self, theta: f32, phi: f32, distance: f32) {
        let x = distance * cos(phi) * sin(theta);
        let y = distance * sin(phi);
        let z = distance * cos(phi) * cos(theta);
        let position = glm::vec4(x, y, z, 1.0);
        let lookat = glm::vec4(0.0, 0.0, 0.0, 1.0); // Ponto "l", para onde a câmera (look-at) estará sempre olhando

        self.theta = theta;
        self.phi = phi;
        self.distance = distance;
        self.x = x;
        self.y = y;
        self.z = z;
        self.position = position;
        self.lookat = lookat;
        self.up_vector = glm::vec4(0.0, 1.0, 0.0, 0.0);
        self.view_vector = lookat - position;
    }

    pub fn offset_distance(&mut self, offset: f32) {
        self.update(self.theta, self.phi, self.distance + offset);
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        *self
    }
}
