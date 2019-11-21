use complex_obj::ComplexObj;
use cube::Cube;
use draw::Attach;
use draw::Draw;
use draw::DrawSelf;
use obj_model::ObjModel;

#[allow(dead_code)]
pub enum PrimitiveObject {
    ObjModel(ObjModel),
    Cube(Cube),
}

#[allow(dead_code)]
pub enum SceneObject {
    PrimitiveObject(PrimitiveObject),
    ComplexObj(ComplexObj<'static>),
}

impl Attach for PrimitiveObject {
    fn attach(&'static self, child: &'static dyn Draw) -> ComplexObj {
        ComplexObj {
            root: self,
            children: Box::new(vec![child]),
        }
    }
}

impl Draw for PrimitiveObject {
    fn draw(&self, program: &u32) {
        match self {
            PrimitiveObject::Cube(cube) => cube.draw(&program),
            PrimitiveObject::ObjModel(obj_model) => obj_model.draw(&program),
        }
    }
}

impl Draw for SceneObject {
    fn draw(&self, program: &u32) {
        match self {
            SceneObject::PrimitiveObject(primitive_object) => primitive_object.draw(&program),
            SceneObject::ComplexObj(complex_obj) => complex_obj.draw(&program),
        }
    }
}

impl DrawSelf for PrimitiveObject {}
impl DrawSelf for SceneObject {}
