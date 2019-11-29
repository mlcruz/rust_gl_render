use super::draw::Draw;
use super::matrix::GLMatrix;
use super::matrix::MatrixTransform;
use super::obj_model::ObjModel;

#[derive(Clone, Debug)]
pub struct CompositeObj {
    pub root: ObjModel,
    pub children: Vec<ObjModel>,
}

#[allow(dead_code)]
impl CompositeObj {
    pub fn add_children(&self, child: &ObjModel) -> Self {
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
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &mut Self {
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
                matrix: self.root.model.matrix * item.model.matrix,
            })
            .draw(program);
        });

        self
    }
}
