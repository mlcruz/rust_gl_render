use draw::Draw;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ComplexObj<'a> {
    pub root: &'a dyn Draw,
    pub children: Box<Vec<&'a dyn Draw>>,
}

impl Draw for ComplexObj<'static> {
    fn draw(&self, program: &u32) {
        self.root.draw(program);
        ((&*self.children)
            .iter()
            .for_each(|item| item.draw(&program)));
    }
}
