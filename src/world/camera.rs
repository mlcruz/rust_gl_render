use glm::cos;
use glm::sin;

#[derive(Debug, Copy)]
pub struct Camera {
    pub theta: f32,    // Ângulo no plano ZX em relação ao eixo Z
    pub phi: f32,      // Ângulo em relação ao eixo Y
    pub distance: f32, // Distância da câmera para a origem

    // Posição da câmera utilizando coordenadas esféricas.
    pub y: f32,
    pub z: f32,
    pub x: f32,

    pub position: glm::Vector4<f32>, // Ponto "c", centro da câmera
    pub target: glm::Vector4<f32>, // Ponto "l", para onde a câmera (look-at) estará sempre olhando

    pub view_vector: glm::Vector4<f32>, // Vetor "view", sentido para onde a câmera está virada
    pub up_vector: glm::Vector4<f32>, // Vetor "up" fixado para apontar para o "céu" (eito Y global)
    pub camera_origin: glm::Vec4,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(theta: f32, phi: f32, distance: f32) -> Self {
        let x = distance * cos(phi) * sin(theta);
        let y = distance * sin(phi);
        let z = distance * cos(phi) * cos(theta);

        let position = glm::vec4(x, y, -z, 1.0);
        let target = glm::vec4(0.0, 0.0, 0.0, 1.0); // Ponto "l", para onde a câmera (look-at) estará sempre olhando

        Camera {
            theta,
            phi,
            distance,
            x,
            y,
            z,
            position,
            target,
            view_vector: target - position, // Vetor "view", sentido para onde a câmera está virada
            up_vector: glm::vec4(0.0, 1.0, 0.0, 1.0), // Vetor "up" fixado para apontar para o "céu" (eito Y global),
            camera_origin: glm::vec4(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn update_angle(&mut self, theta: f32, phi: f32) -> &Self {
        self.phi = phi;
        self.theta = theta;

        self.x = self.distance * cos(phi) * sin(theta);
        self.y = self.distance * sin(phi);
        self.z = self.distance * cos(phi) * cos(theta);

        self.target = glm::vec4(-self.x, -self.y, self.z, 1.0);
        self.view_vector = self.target - self.position;
        self
    }

    pub fn update_position(&mut self, position: &glm::Vec4) -> &Self {
        self.position = *position;
        self
    }

    pub fn translate_position(&mut self, position: &glm::Vec4) -> &Self {
        self.position = self.position + *position;
        self
    }
    pub fn with_origin(&self, origin: &glm::Vec4) -> Self {
        Self {
            camera_origin: *origin,
            ..*self
        }
    }

    pub fn with_theta(&self, theta: &f32) -> Self {
        Self {
            theta: *theta,
            ..*self
        }
    }

    pub fn with_phi(&self, phi: &f32) -> Self {
        Self { phi: *phi, ..*self }
    }

    pub fn with_distance(&self, distance: &f32) -> Self {
        Self {
            distance: *distance,
            ..*self
        }
    }

    pub fn with_position(&self, position: &glm::Vec4) -> Self {
        Self {
            position: *position,
            ..*self
        }
    }

    pub fn with_target(&self, target: &glm::Vec4) -> Self {
        Self {
            target: *target,
            ..*self
        }
    }

    pub fn with_view_vector(&self, view_vector: &glm::Vec4) -> Self {
        Self {
            view_vector: *view_vector,
            ..*self
        }
    }

    pub fn with_up_vector(&self, up_vector: &glm::Vec4) -> Self {
        Self {
            up_vector: *up_vector,
            ..*self
        }
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        *self
    }
}
