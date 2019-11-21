use draw::Attach;
use draw::Draw;
use draw::DrawSelf;
use matrix::GLMatrix;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ComplexObj<'a> {
    pub root: Box<&'a dyn Draw>,
    pub children: Box<Vec<&'a dyn Draw>>,
    pub matrix: GLMatrix,
}

#[allow(dead_code)]
impl<'a> ComplexObj<'a> {
    pub fn new(
        root: &'a (dyn Draw + 'a),
        child: Vec<&'a (dyn Draw + 'a)>,
        matrix: GLMatrix,
    ) -> ComplexObj<'a> {
        let new_root = root.clone();
        let new_children = child.clone();

        ComplexObj {
            root: Box::new(new_root),
            children: Box::new(new_children),
            matrix: matrix,
        }
    }
    pub fn attach_to(&'a self, parent: &'a dyn Draw) -> Self {
        let new_self = self.clone();
        let mut new_children = new_self.children;
        new_children.append(&mut vec![(*new_self.root)]);

        ComplexObj {
            root: Box::new(parent.clone()),
            children: Box::new(*new_children),
            matrix: new_self.matrix,
        }
    }

    pub fn add_child(&'a mut self, child: &'a dyn Draw) -> &mut Self {
        let new_children = child.clone();
        self.children.append(&mut vec![new_children]);
        self
    }

    pub fn add_children(&'a mut self, child: Vec<&'a dyn Draw>) -> &mut Self {
        let mut new_children = child;
        self.children.append(&mut new_children);
        self
    }

    pub fn draw_self(&self, program: &u32) -> &Self {
        self.draw_with_transform(self.matrix, program);
        self
    }
}
impl Draw for ComplexObj<'_> {
    fn draw(&self, program: &u32) {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));
    }

    fn draw_with_transform(&self, matrix: GLMatrix, program: &u32) {
        self.root.draw_with_transform(self.matrix, program);
        let new_matrix = matrix.matrix * self.matrix.matrix;
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw_with_transform(GLMatrix { matrix: new_matrix }, &program)));
    }
}

impl DrawSelf for ComplexObj<'_> {
    fn draw_self(&self, program: &u32) -> &Self {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));

        self
    }
}

impl<'a> Attach<'a> for ComplexObj<'a> {
    fn attach(&'a self, child: &'a dyn Draw) -> Self {
        let mut new_children = self.children.clone();
        new_children.append(&mut vec![child]);
        ComplexObj::new(*self.root.clone(), *new_children, self.matrix)
    }
}
