use complex_obj::ComplexObj;
use cube::Cube;

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

// impl<'a> Attach<'a> for PrimitiveObject {
//     fn attach(&'a self, child: &'a dyn Draw) -> ComplexObj {
//         ComplexObj::new(self, vec![child.clone()], ss)
//     }
// }

// impl Draw for PrimitiveObject {
//     fn draw(&self, program: &u32) {
//         match self {
//             PrimitiveObject::Cube(cube) => cube.draw(&program),
//             PrimitiveObject::ObjModel(obj_model) => obj_model.draw(&program),
//         }
//     }
// }

// impl Draw for SceneObject {
//     fn draw(&self, program: &u32) {
//         match self {
//             SceneObject::PrimitiveObject(primitive_object) => primitive_object.draw(&program),
//             SceneObject::ComplexObj(complex_obj) => complex_obj.draw(&program),
//         }
//     }
// }

// impl DrawSelf for PrimitiveObject {}
// impl DrawSelf for SceneObject {}
