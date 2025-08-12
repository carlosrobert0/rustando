#[derive(Debug)]
struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

impl User {
    fn new(id: u32, first_letter_name: char, active: bool, experience_years: f64) -> Self {
        User {
            id,
            first_letter_name,
            active,
            experience_years,
        }
    }
}

impl User {
    fn experience_years_is_greater_than(&self, years: f64) -> bool {
        self.experience_years > years
    }
}

fn main() {
    let user = User::new(1, 'A', true, 5.0);
    let user_copy = user;
    let user_b = User::new(2, 'B', true, 5.0);
    let mut user_mut = User::new(3, 'C', false, 5.0);
    user_mut.experience_years += 1.0;

    println!("Usuário 'A' imutável: {:#?}", user);
    println!("Usuário cópia: {:#?}", user_copy);
    println!("Usuário 'B': {:#?}", user_b);
    println!("Usuário 'C' mutável: {:#?}", user_mut);

    println!(
        "Usuário 'A' tem mais de 4 anos de experiência? {}",
        user.experience_years_is_greater_than(4.0)
    );
    println!(
        "Usuário 'B' tem mais de 4 anos de experiência? {}",
        user_b.experience_years_is_greater_than(4.0)
    );
    println!(
        "Usuário 'C' tem mais de 4 anos de experiência? {}",
        user_mut.experience_years_is_greater_than(4.0)
    );
}
