struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn hello_world() -> String {
        String::from("hello world")
    }
}

fn main() {
    let rect = Rectangle {
        length: 50,
        width: 30,
    };

    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!("A area do retangulo é {}", rect.area());

    println!("O rect1 cabe no rect2? {}", rect1.can_hold(&rect2));
    println!("O rect1 cabe no rect3? {}", rect1.can_hold(&rect3));

    println!(
        "Olá mundo com struct, funcao associada: {}",
        Rectangle::hello_world()
    );
}
