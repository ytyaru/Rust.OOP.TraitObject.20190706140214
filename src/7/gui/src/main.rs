//https://ja.stackoverflow.com/questions/34317/rust-vec-にトレイトを実装したオブジェクトを格納したい
//pub trait Draw<T> { fn draw(&self); }
pub trait Draw { fn draw(&self); }
pub struct Screen {
//pub struct Screen<T> {
//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//    pub a: Vec<Box<Trait<T>>>,
//    pub components: Vec<Box<Draw<T>>>,
    pub components: Vec<Box<Draw>>,
}
//impl<T> Screen<T> where T: Draw<T> {
//impl<T> Screen<T> where T: Draw {
impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() { c.draw(); }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
//impl<T> Draw<T> for Button {
impl Draw for Button {
    fn draw(&self) {}
//    fn draw(&self) { println!("{:?}", self); }
}
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
//impl<T> Draw<T> for SelectBox {
impl Draw for SelectBox {
    fn draw(&self) {}
}
/*
pub struct Struct<T> {
    a: Vec<Box<Trait<T>>>,
}
impl<T> Trait<T> for () {}
impl<T> Trait<T> for bool {}
fn main() {
    let mut s: Struct<()> = Struct { a: vec![] };
    s.a.push(Box::new(()));
    s.a.push(Box::new(true));
}
*/
fn main() {
//    let mut s: Struct<()> = Struct { a: vec![] };
//    s.a.push(Box::new(()));
//    s.a.push(Box::new(true));
    let screen = Screen { // error[E0282]: type annotations needed
//    let screen = Screen<_> { // error: expected expression, found reserved identifier `_`
//    let screen: Screen<Draw> = Screen { // error[E0107]: wrong number of type arguments: expected 1, found 0
//    let screen: Screen<()> = Screen { // error[E0599]: no method named `run` found for type `Screen<()>` in the current scope
//    let screen: Screen<_>= Screen { // error[E0282]: type annotations needed
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
