use cube::Cube;
use matrix::GLMatrix;
use matrix::MatrixTransform;
use obj_model::ObjModel;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum SimpleObj {
    ObjModel(ObjModel),
    Cube(Cube),
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ComplexObj {
    pub root: SimpleObj,
    pub children: Box<Vec<SimpleObj>>,
}

#[allow(dead_code)]
impl ComplexObj {
    pub fn draw(&self, program: &u32) {
        match self.root {
            SimpleObj::Cube(cube) => {
                cube.draw(program);
            }
            SimpleObj::ObjModel(obj_model) => {
                obj_model.draw(program);
            }
        }

        match self.children.as_slice() {
            [] => {
                ((&*self.children).iter().for_each(|item| match item {
                    SimpleObj::ObjModel(obj_model) => {
                        obj_model.draw(program);
                    }
                    SimpleObj::Cube(cube) => {
                        cube.draw(program);
                    }
                }));
            }
            _ => (),
        }
    }
}

impl MatrixTransform for ComplexObj {
    fn get_matrix(&self) -> GLMatrix {
        match self.root {
            SimpleObj::Cube(cube) => cube.model,
            SimpleObj::ObjModel(obj_model) => obj_model.model,
        }
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) -> Self {
        match self.root {
            SimpleObj::Cube(mut cube) => {
                cube.model.update(matrix);
                ComplexObj {
                    children: self.children.clone(),
                    root: SimpleObj::Cube(cube),
                }
            }
            SimpleObj::ObjModel(mut obj_model) => {
                obj_model.model.update(matrix);
                ComplexObj {
                    children: self.children.clone(),
                    root: SimpleObj::ObjModel(obj_model),
                }
            }
        }
    }
    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        ComplexObj {
            children: self.children.clone(),
            root: match self.root {
                SimpleObj::Cube(cube) => SimpleObj::Cube(cube.from_matrix(matrix)),
                SimpleObj::ObjModel(obj_model) => {
                    SimpleObj::ObjModel(obj_model.from_matrix(matrix))
                }
            },
        }
    }
}
