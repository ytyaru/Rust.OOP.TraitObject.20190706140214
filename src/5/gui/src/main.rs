//https://ja.stackoverflow.com/questions/34317/rust-vec-にトレイトを実装したオブジェクトを格納したい
trait Trait<T> {}
struct Struct<T> {
    a: Vec<Box<Trait<T>>>,
}
impl<T> Trait<T> for () {}
impl<T> Trait<T> for bool {}
fn main() {
    let mut s: Struct<()> = Struct { a: vec![] };
    s.a.push(Box::new(()));
    s.a.push(Box::new(true));
}
