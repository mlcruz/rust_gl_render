use complex_obj::ComplexObj;
use draw::Draw;
use matrix::GLMatrix;
use matrix::MatrixTransform;
use obj_model::ObjModel;

#[derive(Clone, Debug)]
pub struct CompositeObj {
    pub root: ObjModel,
    pub children: Vec<ObjModel>,
}

#[allow(dead_code)]
impl CompositeObj {
    fn add_children(&self, child: &ObjModel) -> Self {
        let mut new_children = self.children.clone();
        new_children.append(&mut vec![*child]);

        CompositeObj {
            root: self.root,
            children: new_children,
        }
    }
}

impl MatrixTransform for CompositeObj {
    fn get_matrix(&self) -> &GLMatrix {
        &self.root.model
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &Self {
        self.root.model.update(matrix);
        self
    }

    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        CompositeObj {
            root: *self.root.clone().update_matrix(matrix),
            children: self.children.clone(),
        }
    }
}

#[allow(dead_code)]
impl Draw for CompositeObj {
    fn draw(&self, program: &u32) -> &Self {
        self.root.draw(program);

        self.children.iter().for_each(|item| {
            item.from_matrix(&GLMatrix {
                matrix: item.model.matrix * self.root.model.matrix,
            })
            .draw(program);
        });

        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SceneObject {
    CompositeObj(CompositeObj),
    ObjModel(ObjModel),
    ComplexObj(ComplexObj),
}

#[allow(dead_code)]
impl SceneObject {
    fn new(path: &str) -> Self {
        SceneObject::ObjModel(ObjModel::new(path))
    }

    pub fn add_children(&self, child: &SceneObject) -> Self {
        match self {
            SceneObject::ObjModel(obj_model) => match child {
                SceneObject::ObjModel(c_obj) => SceneObject::CompositeObj(CompositeObj {
                    root: *obj_model,
                    children: vec![*c_obj],
                }),
                SceneObject::CompositeObj(c_cmp) => SceneObject::ComplexObj(ComplexObj {
                    root: *obj_model,
                    children: Box::new(vec![SceneObject::CompositeObj(c_cmp.clone())]),
                }),
                SceneObject::ComplexObj(c_cplx) => SceneObject::ComplexObj(ComplexObj {
                    root: *obj_model,
                    children: Box::new(vec![SceneObject::ComplexObj(c_cplx.clone())]),
                }),
            },
            SceneObject::CompositeObj(cmp_model) => match child {
                SceneObject::ObjModel(c_obj) => {
                    SceneObject::CompositeObj(cmp_model.add_children(c_obj))
                }
                SceneObject::CompositeObj(c_cmp) => {
                    let mut new_children: Vec<SceneObject> = cmp_model
                        .children
                        .clone()
                        .iter()
                        .map(|item| SceneObject::ObjModel(*item))
                        .collect();

                    new_children.append(&mut vec![SceneObject::CompositeObj((*c_cmp).clone())]);

                    SceneObject::ComplexObj(ComplexObj {
                        root: cmp_model.root,
                        children: Box::new(new_children),
                    })
                }
                SceneObject::ComplexObj(c_cplx) => {
                    let mut new_children: Vec<SceneObject> = cmp_model
                        .children
                        .clone()
                        .iter()
                        .map(|item| SceneObject::ObjModel(*item))
                        .collect();

                    new_children.append(&mut vec![SceneObject::ComplexObj((*c_cplx).clone())]);

                    SceneObject::ComplexObj(ComplexObj {
                        root: cmp_model.root,
                        children: Box::new(new_children),
                    })
                }
            },
            SceneObject::ComplexObj(cplx_model) => {
                SceneObject::ComplexObj(cplx_model.add_children(child))
            }
        }
    }
}

#[allow(dead_code)]
impl Draw for SceneObject {
    fn draw(&self, program: &u32) -> &Self {
        match self {
            SceneObject::ObjModel(obj_model) => {
                obj_model.draw(&program);
            }
            SceneObject::ComplexObj(complex_obj) => {
                complex_obj.draw(&program);
            }
            SceneObject::CompositeObj(composite_obj) => {
                composite_obj.draw(&program);
            }
        }
        self
    }
}

impl MatrixTransform for SceneObject {
    fn get_matrix(&self) -> &GLMatrix {
        match self {
            SceneObject::ObjModel(obj_model) => &obj_model.get_matrix(),
            SceneObject::ComplexObj(complex_obj) => &complex_obj.get_matrix(),
            SceneObject::CompositeObj(composite_obj) => &composite_obj.get_matrix(),
        }
    }

    fn update_matrix(&mut self, matrix: &GLMatrix) -> &Self {
        match self {
            SceneObject::ObjModel(obj_model) => {
                let new_me = obj_model.update_matrix(matrix).clone();
                *self = SceneObject::ObjModel(new_me);
                self
            }
            SceneObject::ComplexObj(complex_obj) => {
                *self = SceneObject::ComplexObj(complex_obj.update_matrix(matrix).clone());
                self
            }
            SceneObject::CompositeObj(composite_obj) => {
                *self = SceneObject::CompositeObj(composite_obj.update_matrix(matrix).clone());
                self
            }
        }
    }
    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        match self {
            SceneObject::ObjModel(obj_model) => {
                SceneObject::ObjModel(obj_model.from_matrix(matrix))
            }
            SceneObject::ComplexObj(complex_obj) => {
                SceneObject::ComplexObj(complex_obj.from_matrix(matrix))
            }
            SceneObject::CompositeObj(composite_obj) => {
                SceneObject::CompositeObj(composite_obj.from_matrix(matrix))
            }
        }
    }
}

// match &self {

//     SceneObject::CompositeObj(composite_obj) => {
//         composite_obj
//             .from_matrix(&GLMatrix {
//                 matrix: composite_obj.root.model.matrix * self.root.model.matrix,
//             })
//             .draw(program);
//     }
//     SceneObject::ComplexObj(complex_obj) => {
//         complex_obj
//             .from_matrix(&GLMatrix {
//                 matrix: complex_obj.root.model.matrix * self.root.model.matrix,
//             })
//             .draw();
//     }
// }
// };
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
