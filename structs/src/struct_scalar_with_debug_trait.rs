/*
    Struct com valores escalares
    Estrutura de dados simples que copia e gerencia memória automaticamente
    Sem precisar de gerenciamento manual de memória, com ownership

    Exibindo os dados do usuário com o trait Debug, mostrando todas as informações
*/

#[derive(Debug)]
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

    // {:?} para exibir todos os dados do usuário
    // exibe tudo em uma linha
    println!("Usuário 'A' imutável, {:?}", user);
    // {:#?} para exibir todos os dados do usuário de forma mais legível, (pretty-print)
    // adiciona quebras de linha e identação
    println!("Usuário 'A' imutável, com precisão: {:#?}", user);

    println!("Usuário cópia, {:?}", user_copy);
    println!("Usuário 'B', {:?}", user_b);
    println!("Usuário 'C' mutável, {:?}", user_mut);
}
