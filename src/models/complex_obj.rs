use super::draw::Draw;
use super::matrix::GLMatrix;
use super::matrix::MatrixTransform;
use super::obj_model::ObjModel;
use super::scene_object::SceneObject;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ComplexObj {
    pub root: ObjModel,
    pub children: Box<Vec<SceneObject>>,
}

#[allow(dead_code)]
impl ComplexObj {
    pub fn add_children(&self, child: &SceneObject) -> Self {
        let mut new_children = self.children.clone();
        new_children.append(&mut vec![(*child).clone()]);

        ComplexObj {
            root: self.root,
            children: new_children,
        }
    }
}

impl MatrixTransform for ComplexObj {
    fn get_matrix(&self) -> &GLMatrix {
        self.root.get_matrix()
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &Self {
        self.root.model.update(matrix);
        self
    }

    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        ComplexObj {
            root: *self.root.clone().update_matrix(matrix),
            children: Box::new((&*self.children).to_vec()),
        }
    }
}

impl Draw for ComplexObj {
    fn draw(&self, program: &u32) -> &Self {
        self.root.draw(program);

        self.children.iter().for_each(|item| match item {
            SceneObject::ObjModel(obj_model) => {
                obj_model
                    .from_matrix(&GLMatrix {
                        matrix: obj_model.model.matrix * self.root.model.matrix,
                    })
                    .draw(program);
            }
            SceneObject::CompositeObj(composite_obj) => {
                composite_obj
                    .from_matrix(&GLMatrix {
                        matrix: composite_obj.root.model.matrix * self.root.model.matrix,
                    })
                    .draw(program);
            }
            SceneObject::ComplexObj(complex_obj) => {
                complex_obj
                    .from_matrix(&GLMatrix {
                        matrix: complex_obj.root.model.matrix * self.root.model.matrix,
                    })
                    .draw(program);
            }
        });

        self
    }
}
