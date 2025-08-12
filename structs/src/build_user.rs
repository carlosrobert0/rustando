#[derive(Debug)]
struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

fn main() {
    let user = build_user(1, 'A', true, 5.0);
    let user_copy = user;
    let user_b = build_user(2, 'B', true, 5.0);
    let mut user_mut = build_user(3, 'C', false, 5.0);
    user_mut.experience_years += 1.0;

    println!("Usuário 'A' imutável: {:#?}", user);
    println!("Usuário cópia: {:#?}", user_copy);
    println!("Usuário 'B': {:#?}", user_b);
    println!("Usuário 'C' mutável: {:#?}", user_mut);
}

fn build_user(id: u32, first_letter_name: char, active: bool, experience_years: f64) -> User {
    User {
        id,
        first_letter_name,
        active,
        experience_years,
    }
}
