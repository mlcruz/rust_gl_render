#[derive(Debug, Copy, Clone)]
pub struct Lighting {
    pub global: glm::Vec3,
    pub ambient: glm::Vec3,
    pub global_direction: glm::Vec4,
}
#[allow(dead_code)]
impl Lighting {
    pub fn new(global: &glm::Vec3, ambient: &glm::Vec3, global_direction: &glm::Vec4) -> Self {
        Lighting {
            global: *global,
            ambient: *ambient,
            global_direction: *global_direction,
        }
    }

    pub fn update(
        &self,
        global: &glm::Vec3,
        ambient: &glm::Vec3,
        global_direction: &glm::Vec4,
    ) -> Self {
        Lighting {
            global: *global,
            ambient: *ambient,
            global_direction: *global_direction,
        }
    }
}
