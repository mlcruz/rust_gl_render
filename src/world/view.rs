use glm::Matrix4;
use models::matrix::camera_view_matrix;
use models::matrix::ortographic_matrix;
use models::matrix::perpective_matrix;
use std::mem;
use world::camera::Camera;
use world::lighting::Lighting;
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
    pub lighting: Lighting,
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
            lighting: Lighting::new(
                &glm::vec3(1.0, 1.0, 1.0),
                &glm::vec3(0.9412, 0.7255, 0.7255),
            ),
        }
    }

    pub fn render(&self, program: &u32) -> Self {
        unsafe {
            let view_uniform =
                gl::GetUniformLocation(*program, CString::new("view").unwrap().as_ptr());
            let projection_uniform =
                gl::GetUniformLocation(*program, CString::new("projection").unwrap().as_ptr());

            let global_lighting_uniform =
                gl::GetUniformLocation(*program, CString::new("global_lighting").unwrap().as_ptr());

            let camera_origin_uniform =
                gl::GetUniformLocation(*program, CString::new("camera_origin").unwrap().as_ptr());

            let ambient_lighting_uniform = gl::GetUniformLocation(
                *program,
                CString::new("ambient_lighting").unwrap().as_ptr(),
            );

            gl::Uniform3f(
                ambient_lighting_uniform,
                self.lighting.ambient.x,
                self.lighting.ambient.y,
                self.lighting.ambient.z,
            );

            // Enviamos as matrizes "view" e "projection" para a placa de vídeo
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

            gl::Uniform3f(
                global_lighting_uniform,
                self.lighting.global.x,
                self.lighting.global.y,
                self.lighting.global.z,
            );

            gl::Uniform4f(
                camera_origin_uniform,
                self.camera.camera_origin.x,
                self.camera.camera_origin.y,
                self.camera.camera_origin.z,
                self.camera.camera_origin.w,
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

    pub fn update_lighting(&mut self, lighting: &Lighting) -> &Self {
        self.lighting = *lighting;
        self
    }
    pub fn update(
        &mut self,
        nearplane: f32,
        farplane: f32,
        camera: &Camera,
        projection_matrix: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        lighting: &Lighting,
    ) -> Self {
        self.nearplane = nearplane;
        self.farplane = farplane;
        self.camera = camera.clone();
        self.projection_matrix = projection_matrix.clone();
        self.view_matrix = view_matrix.clone();
        self.lighting = lighting.clone();
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
            &self.lighting,
        )
    }

    pub fn perpective(&self) -> Self {
        self.clone().update(
            self.nearplane,
            self.nearplane,
            &self.camera,
            &perpective_matrix(FIELD_OF_VIEW, G_SCREEN_RATIO, self.nearplane, self.farplane).matrix,
            &self.view_matrix,
            &self.lighting,
        )
    }

    pub fn with_ambient_lighting(&self, ambient: &glm::Vec3) -> Self {
        let new_lighting = Lighting {
            global: self.lighting.global,
            ambient: *ambient,
        };
        Self {
            lighting: new_lighting,
            ..*self
        }
    }

    pub fn with_global_lighting(&self, global: &glm::Vec3) -> Self {
        let new_lighting = Lighting {
            global: *global,
            ambient: self.lighting.ambient,
        };
        Self {
            lighting: new_lighting,
            ..*self
        }
    }

    pub fn with_camera(&self, camera: &Camera) -> Self {
        Self {
            camera: *camera,
            view_matrix: camera_view_matrix(camera.position, camera.view_vector, camera.up_vector)
                .matrix,
            ..*self
        }
    }

    pub fn with_lighting(&self, lighting: &Lighting) -> Self {
        Self {
            lighting: *lighting,
            ..*self
        }
    }

    pub fn with_near_plane(&self, nearplane: &f32) -> Self {
        Self {
            nearplane: *nearplane,
            projection_matrix: perpective_matrix(
                FIELD_OF_VIEW,
                G_SCREEN_RATIO,
                *nearplane,
                self.farplane,
            )
            .matrix,
            ..*self
        }
    }

    pub fn with_far_plane(&self, farplane: &f32) -> Self {
        Self {
            farplane: *farplane,
            projection_matrix: perpective_matrix(
                FIELD_OF_VIEW,
                G_SCREEN_RATIO,
                self.nearplane,
                *farplane,
            )
            .matrix,
            ..*self
        }
    }
}

impl Clone for View {
    fn clone(&self) -> Self {
        *self
    }
}
