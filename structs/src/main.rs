#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn hello_word() {
        println!("Hello world");
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("{:?}", Rectangle::hello_word());

    println!("A area do retangulo é {} em pixels quadrados", rect.area());
}