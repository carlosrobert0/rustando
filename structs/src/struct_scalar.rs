/*
    Struct com valores escalares
    Estrutura de dados simples que copia e gerencia memória automaticamente
    Sem precisar de gerenciamento manual de memória, com ownership

    Exibindo os dados únicos do usuário, sem mostrar estrutura completa
*/

struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

fn main() {
    let user = User {
        id: 1,
        first_letter_name: 'A',
        active: true,
        experience_years: 5.0,
    };

    let user_copy = user;

    let user_b = User {
        id: 2,
        first_letter_name: 'B',
        ..user
    };

    let mut user_mut = User {
        id: 3,
        first_letter_name: 'C',
        active: false,
        experience_years: 5.0,
    };

    user_mut.experience_years += 1.0;

    println!(
        "Usuário 'A' imutável, primeira letra: {}",
        user.first_letter_name
    );
    println!(
        "Usuário cópia, primeira letra: {}",
        user_copy.first_letter_name
    );
    println!("Usuário 'B', primeira letra: {}", user_b.first_letter_name);
    println!(
        "Usuário 'C' mutável, anos de experiência: {}",
        user_mut.experience_years
    );
}
