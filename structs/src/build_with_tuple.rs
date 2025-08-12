#[derive(Debug)]
struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

fn main() {
    let user = build_user((1, 'A', true, 5.0));
    println!("Usuário criado com tupla: {:#?}", user);
}

fn build_user(user: (u32, char, bool, f64)) -> User {
    User {
        id: user.0,
        first_letter_name: user.1,
        active: user.2,
        experience_years: user.3,
    }
}
