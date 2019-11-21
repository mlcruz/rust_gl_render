use draw::Attach;
use draw::Draw;
use draw::DrawSelf;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ComplexObj<'a> {
    pub root: Box<&'a dyn Draw>,
    pub children: Box<Vec<&'a dyn Draw>>,
}

#[allow(dead_code)]
impl<'a> ComplexObj<'a> {
    pub fn new(root: &'a (dyn Draw + 'a), child: Vec<&'a (dyn Draw + 'a)>) -> ComplexObj<'a> {
        let new_root = root.clone();
        let new_children = child.clone();

        ComplexObj {
            root: Box::new(new_root),
            children: Box::new(new_children),
        }
    }
    pub fn attach_to(&'a self, parent: &'a dyn Draw) -> Self {
        let new_self = self.clone();
        let mut new_children = new_self.children;
        new_children.append(&mut vec![(*new_self.root)]);

        ComplexObj {
            root: Box::new(parent.clone()),
            children: Box::new(*new_children),
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
}
impl Draw for ComplexObj<'_> {
    fn draw(&self, program: &u32) {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));
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
        ComplexObj::new(*self.root.clone(), *new_children)
    }
}
