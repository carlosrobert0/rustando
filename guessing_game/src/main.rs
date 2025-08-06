use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Adivinhe o número!");

        println!("Digite o seu palpite.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
