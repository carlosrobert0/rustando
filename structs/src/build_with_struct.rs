#[derive(Debug)]
struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

fn main() {
    let user = build_user(&User {
        id: 1,
        first_letter_name: 'A',
        active: true,
        experience_years: 5.0,
    });

    println!("Usuário criado com struct: {:#?}", user);
}

fn build_user(user: &User) -> User {
    User {
        id: user.id,
        first_letter_name: user.first_letter_name,
        active: user.active,
        experience_years: user.experience_years,
    }
}
