use gui::{Screen,Button};
use gui::Draw;
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };
    screen.run();
}

