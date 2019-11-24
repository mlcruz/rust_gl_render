#[derive(Debug, Copy, Clone)]
pub struct Lighting {
    pub global: glm::Vec3,
    pub ambient: glm::Vec3,
}
#[allow(dead_code)]
impl Lighting {
    pub fn new(global: &glm::Vec3, ambient: &glm::Vec3) -> Self {
        Lighting {
            global: *global,
            ambient: *ambient,
        }
    }

    pub fn update(&self, global: &glm::Vec3, ambient: &glm::Vec3) -> Self {
        Lighting {
            global: *global,
            ambient: *ambient,
        }
    }
}
