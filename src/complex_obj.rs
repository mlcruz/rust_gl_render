use draw::Attach;
use draw::Draw;
use draw::DrawSelf;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ComplexObj<'a> {
    pub root: &'a dyn Draw,
    pub children: Box<Vec<&'a dyn Draw>>,
}

#[allow(dead_code)]
impl ComplexObj<'static> {
    pub fn attach_to(&'static self, parent: &'static dyn Draw) -> Self {
        let mut new_children = self.children.clone();
        new_children.append(&mut vec![self]);
        ComplexObj {
            root: parent,
            children: new_children,
        }
    }
}
impl Draw for ComplexObj<'static> {
    fn draw(&self, program: &u32) {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));
    }
}

impl DrawSelf for ComplexObj<'static> {
    fn draw_self(&self, program: &u32) -> &Self {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));

        self
    }
}

impl Attach for ComplexObj<'static> {
    fn attach(&'static self, child: &'static dyn Draw) -> Self {
        let mut new_children = self.children.clone();
        new_children.append(&mut vec![child]);
        ComplexObj {
            root: self,
            children: new_children,
        }
    }
}
