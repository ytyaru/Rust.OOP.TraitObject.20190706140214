pub trait Draw { fn draw(&self); }
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() { c.draw(); }
    }
}
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) { println!("Button: {:?}", self); }
}

