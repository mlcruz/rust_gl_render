use glm::Matrix4;
use models::matrix::ortographic_matrix;
use models::matrix::perpective_matrix;
use std::mem;
use world::free_camera::FreeCamera;
use world::lighting::Lighting;
static FIELD_OF_VIEW: f32 = 3.141592 / 3.0;
static G_SCREEN_RATIO: f32 = 1.0;
use std::ffi::CString;

// Representa um campo de visão, com sua propria camera e iluminação
#[derive(Debug, Copy)]
pub struct View {
    nearplane: f32, // Ângulo no plano ZX em relação ao eixo Z
    farplane: f32,  // Ângulo em relação ao eixo Y
    pub projection_matrix: Matrix4<f32>,
    camera: FreeCamera,
    pub lighting: Lighting,
}

#[allow(dead_code)]
impl View {
    pub fn new(nearplane: f32, farplane: f32, camera: &FreeCamera) -> Self {
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
            lighting: Lighting::new(
                &glm::vec3(1.0, 1.0, 1.0),
                &glm::vec3(0.25, 0.25, 0.25),
                &glm::vec4(1.0, 1.0, 0.0, 0.0),
            ),
        }
    }

    // Prepara para desenhar, sempre chamado antes do draw dos objs
    pub fn render(&self, program: &u32) -> Self {
        let camera_origin = glm::vec4(0.0, 0.0, 0.0, 1.0);

        unsafe {
            let view_uniform =
                gl::GetUniformLocation(*program, CString::new("view").unwrap().as_ptr());
            let projection_uniform =
                gl::GetUniformLocation(*program, CString::new("projection").unwrap().as_ptr());

            let global_lighting_uniform =
                gl::GetUniformLocation(*program, CString::new("global_lighting").unwrap().as_ptr());

            let global_lighting_direction_uniform = gl::GetUniformLocation(
                *program,
                CString::new("lighting_direction").unwrap().as_ptr(),
            );

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
                mem::transmute(&self.camera.view_matrix.matrix[0]),
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
                camera_origin.x,
                camera_origin.y,
                camera_origin.z,
                camera_origin.w,
            );

            gl::Uniform4f(
                global_lighting_direction_uniform,
                self.lighting.global_direction.x,
                self.lighting.global_direction.y,
                self.lighting.global_direction.z,
                self.lighting.global_direction.w,
            );
        }
        *self
    }
    pub fn update_camera(&mut self, camera: &FreeCamera) -> Self {
        self.camera = camera.clone();
        *self
    }

    pub fn update_lighting(&mut self, lighting: &Lighting) -> &Self {
        self.lighting = *lighting;
        self
    }

    pub fn ortographic(&mut self) -> &Self {
        let t = 1.5 * self.camera.distance / 2.5;
        let b = -t;
        let r = t * G_SCREEN_RATIO;
        let l = -r;

        self.projection_matrix =
            ortographic_matrix(l, r, b, t, self.nearplane, self.farplane).matrix;

        self
    }

    pub fn perpective(&mut self) -> &Self {
        self.projection_matrix =
            perpective_matrix(FIELD_OF_VIEW, G_SCREEN_RATIO, self.nearplane, self.farplane).matrix;

        self
    }

    pub fn with_ambient_lighting(&self, ambient: &glm::Vec3) -> Self {
        let new_lighting = Lighting {
            ambient: *ambient,
            ..self.lighting
        };
        Self {
            lighting: new_lighting,
            ..*self
        }
    }

    pub fn with_global_lighting(&self, global: &glm::Vec3) -> Self {
        let new_lighting = Lighting {
            global: *global,
            ..self.lighting
        };
        Self {
            lighting: new_lighting,
            ..*self
        }
    }

    pub fn with_global_direction(&self, global_direction: &glm::Vec4) -> Self {
        let new_lighting = Lighting {
            global_direction: *global_direction,
            ..self.lighting
        };
        Self {
            lighting: new_lighting,
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
