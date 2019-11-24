pub trait Draw {
    fn draw(&self, program: &u32) -> &Self;
}
