use models::matrix::camera_view_matrix;
use models::matrix::cross_product;
use models::matrix::GLMatrix;

#[derive(Debug, Copy)]
pub struct FreeCamera {
    pub pos: glm::Vec4,
    pub front: glm::Vec4,
    pub view_matrix: GLMatrix,
    pub distance: f32,
}

impl FreeCamera {
    pub fn new(pos: glm::Vec3, front: glm::Vec4) -> Self {
        let up_world = glm::vec4(0.0, 1.0, 0.0, 0.0);
        let pos_as_point = glm::vec4(pos.x, pos.y, pos.z, 1.0);

        println!(" pos:{:?}", pos);
        FreeCamera {
            pos: pos_as_point,
            front,
            view_matrix: camera_view_matrix(pos_as_point, front, up_world),
            distance: 2.5,
        }
    }

    pub fn refresh(&mut self) -> &Self {
        let pos_as_point = glm::vec4(self.pos.x, self.pos.y, self.pos.z, 1.0);
        let up_world = glm::vec4(0.0, 1.0, 0.0, 0.0);

        self.view_matrix = camera_view_matrix(pos_as_point, self.front, up_world);
        self
    }
}

impl Clone for FreeCamera {
    fn clone(&self) -> Self {
        *self
    }
}
