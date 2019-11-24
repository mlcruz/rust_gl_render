#[derive(Debug, Copy, Clone)]
pub struct Lighting {
    pub global: glm::Vec3,
}
#[allow(dead_code)]
impl Lighting {
    pub fn new(global: &glm::Vec3) -> Self {
        Lighting { global: *global }
    }

    pub fn update(&self, global: &glm::Vec3) -> Self {
        Lighting { global: *global }
    }
}
