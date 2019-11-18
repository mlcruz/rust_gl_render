use camera::Camera;
use glm::Matrix4;
use matrix::camera_view_matrix;
use matrix::ortographic_matrix;
use matrix::perpective_matrix;
use std::mem;
static FIELD_OF_VIEW: f32 = 3.141592 / 3.0;
static G_SCREEN_RATIO: f32 = 1.0;
use std::ffi::CString;

#[derive(Debug, Copy)]
pub struct View {
    nearplane: f32, // Ângulo no plano ZX em relação ao eixo Z
    farplane: f32,  // Ângulo em relação ao eixo Y
    pub projection_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,
    camera: Camera,
}

#[allow(dead_code)]
impl View {
    pub fn new(nearplane: f32, farplane: f32, camera: &Camera) -> Self {
        View {
            camera: camera.clone(),
            farplane: farplane,
            nearplane: nearplane,
            projection_matrix: perpective_matrix(
                FIELD_OF_VIEW,
                G_SCREEN_RATIO,
                nearplane,
                farplane,
            )
            .matrix,
            view_matrix: camera_view_matrix(camera.position, camera.view_vector, camera.up_vector)
                .matrix,
        }
    }

    pub fn render(&self, program: &u32) -> Self {
        unsafe {
            let view_uniform =
                gl::GetUniformLocation(*program, CString::new("view").unwrap().as_ptr());
            let projection_uniform =
                gl::GetUniformLocation(*program, CString::new("projection").unwrap().as_ptr());

            // Enviamos as matrizes "view" e "projection" para a placa de vídeo
            // (GPU). Veja o arquivo "shader_vertex.glsl", onde estas são
            // efetivamente aplicadas em todos os pontos.
            gl::UniformMatrix4fv(
                view_uniform,
                1,
                gl::FALSE,
                mem::transmute(&self.view_matrix[0]),
            );
            gl::UniformMatrix4fv(
                projection_uniform,
                1,
                gl::FALSE,
                mem::transmute(&self.projection_matrix[0]),
            );
        }
        *self
    }

    pub fn update_camera(&mut self, camera: &Camera) -> Self {
        self.camera = camera.clone();
        self.view_matrix =
            camera_view_matrix(camera.position, camera.view_vector, camera.up_vector).matrix;
        *self
    }

    pub fn update(
        &mut self,
        nearplane: f32,
        farplane: f32,
        camera: &Camera,
        projection_matrix: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
    ) -> Self {
        self.nearplane = nearplane;
        self.farplane = farplane;
        self.camera = camera.clone();
        self.projection_matrix = projection_matrix.clone();
        self.view_matrix = view_matrix.clone();
        *self
    }

    pub fn ortographic(&self) -> Self {
        let t = 1.5 * self.camera.distance / 2.5;
        let b = -t;
        let r = t * G_SCREEN_RATIO;
        let l = -r;

        self.clone().update(
            self.nearplane,
            self.nearplane,
            &self.camera,
            &ortographic_matrix(l, r, b, t, self.nearplane, self.farplane).matrix,
            &self.view_matrix,
        )
    }

    pub fn perpective(&self) -> Self {
        self.clone().update(
            self.nearplane,
            self.nearplane,
            &self.camera,
            &perpective_matrix(FIELD_OF_VIEW, G_SCREEN_RATIO, self.nearplane, self.farplane).matrix,
            &self.view_matrix,
        )
    }
}

impl Clone for View {
    fn clone(&self) -> Self {
        *self
    }
}
