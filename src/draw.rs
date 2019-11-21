use complex_obj::ComplexObj;
use matrix::GLMatrix;

pub trait DrawSelf: Draw {
    fn draw_self(&self, program: &u32) -> &Self {
        self.draw(program);
        self
    }
}

pub trait Draw {
    fn draw(&self, program: &u32);
    fn draw_with_transform(&self, matrix: GLMatrix, program: &u32);
}

pub trait Attach<'a> {
    fn attach(&'a self, child: &'a dyn Draw) -> ComplexObj<'a>;
}
