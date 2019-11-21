use complex_obj::ComplexObj;

pub trait DrawSelf {
    fn draw_self(&self, program: &u32) -> &Self;
}

pub trait Draw {
    fn draw(&self, program: &u32);
}

pub trait Attach {
    fn attach(&self) -> ComplexObj;
}
