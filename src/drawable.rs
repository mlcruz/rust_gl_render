// Estrutura representando combinação final de objeto e shader a ser desenhado
use models::scene_object::SceneObject;

#[allow(dead_code)]
pub struct Drawable<'a> {
    pub object: &'a SceneObject,
    pub shader: &'a u32,
}

#[allow(dead_code)]
impl<'a> Drawable<'a> {
    pub fn new(object: &'a SceneObject, shader: &'a u32) -> Self {
        Drawable { object, shader }
    }

    pub fn update_object(&self, object: &'a SceneObject) -> Self {
        Drawable { object, ..*self }
    }

    pub fn update_shader(&self, shader: &'a u32) -> Self {
        Drawable { shader, ..*self }
    }
}
