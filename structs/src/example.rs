// Um exemplo de programa usando estruturas (structs) em Rust
// Refatorando...

// Calculando a área espefica de um retangulo por width (largura) e height (altura)
fn main() {
    let width = 30;
    let height = 50;

    println!("A área do retângulo é: {}", calculate_area(width, height));
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

// Refatorando com tuplas
// Calculando a área espefica de um retangulo por width (largura) e height (altura)
fn main() {
    let rect = (30, 50);

    println!("A área do retângulo é: {}", calculate_area(rect));
}

fn calculate_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refatorando com struct: adicionando mais significado
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("A área do retângulo é: {}", calculate_area(&rect));
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// Sintaxe do método: Method Syntax
// Definindo um método para calcular a área
// O primeiro parâmetro é sempre uma referência para a instância (&self)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("A área do retângulo é: {}", rect.area());
}

// Métodos com mais de um parâmetro
// Essa função verifica se um retângulo pode conter outro
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!(
        "O retângulo 1 pode conter o retângulo 2? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "O retângulo 2 pode conter o retângulo 1? {}",
        rect2.can_hold(&rect1)
    );
}

// Funções associadas (Associated Functions)
// Pode ter multiplas implementações
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(20);

    println!("A área do retângulo 1 é: {}", rect1.area());
    println!("A área do retângulo 2 é: {}", rect2.area());
}
