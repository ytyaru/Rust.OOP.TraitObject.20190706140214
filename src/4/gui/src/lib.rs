pub trait Draw {
//pub trait Draw<T> {
    fn draw(&self);
}
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
//    pub components: Vec<Box<T>>,
//    pub components: Vec<Box<Draw<T>>>, // error[E0107]: wrong number of type arguments: expected 0, found 1
//    pub components: Vec<Box<Draw>>, // error[E0392]: parameter `T` is never used
}
impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}
#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
