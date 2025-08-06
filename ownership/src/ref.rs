/*
    Referências e Borrowing

    Em Rust, você pode usar referências para acessar dados sem transferir a propriedade.
    Isso é chamado de borrowing. As referências podem ser imutáveis ou mutáveis.

    A referência é definida com o símbolo `&`, e uma referência mutável é definida com `&mut`.

    As regras de referências são:
    1. Em um dado momento, você pode ter uma ou outra, mas não os dois:
       - Uma referência mutável
       - Uma ou mais referências imutáveis
    2. Referências devem sempre ser válidas.
        Não pode haver referência solta para dados que saíram de escopo.
*/

fn main() {
    let name = String::from("Rustando");
    let mut name_mut = String::from("Rustando Mutável");

    show_channel_name(&name); // Passa uma referência imutável para a função

    println!("Canal: {}", name); // `name` ainda é válido, pois a propriedade não foi movida
    // A função show_channel_name recebe uma referência e não a propriedade da String

    update_channel_name(&mut name_mut); // Passa uma referência mutável para a função
    println!("Canal Atualizado: {}", name_mut); // `name_mut` ainda é válido, pois a propriedade não foi movida
    // A função update_channel_name modifica a String através de uma referência mutável

    let reference_mut = &mut name_mut; // Cria uma referência mutável
    let reference_immut = &name_mut; // Causaria um erro, pois não
    // podemos ter uma referência mutável e uma referência imutável ao mesmo tempo
    // Podemos ter uma referência mutável e uma ou mais referências imutáveis, mas não ambas ao mesmo tempo

    let reference_immut1 = &name; // Cria uma referência imutável
    let reference_immut2 = &name; // Outra referência imutável é permitida
    let reference_immut3 = &name; // Mais uma referência imutável é permitida
    println!(
        "Referências Imutáveis: {}, {}, {}",
        reference_immut1, reference_immut2, reference_immut3
    );

    let loose_ref = loose_reference(); // Causaria um erro, pois a referência retornada está solta
    println!("Referência Solta: {}", loose_ref);

    let channel_name = get_channel_name(); // A propriedade é movida para `channel_name`
    println!("Canal obtido da função: {}", channel_name);
}

fn show_channel_name(name: &String) {
    println!("Canal: {}", name);
}

fn update_channel_name(name: &mut String) {
    name.push_str(" - Canal Atualizado");
}

fn loose_reference() -> &String {
    let name = String::from("Rustando");

    &name
    // Isso causaria um erro, pois a referência &name está solta aqui, e name sai de escopo ao final da função
    // O compilador não permite referências soltas para dados que saíram de escopo
}

fn get_channel_name() -> String {
    let name = String::from("Rustando");
    name // A propriedade de `name` é movida para quem chamar essa função
    // A função retorna a String, transferindo a propriedade para o chamador
}

// As regras de referencias:
// 1- Em um dado momento, voce pode ter um ou outro mas nao os dois:
//    - Uma referencia mutavel
//    - Uma ou mais referencias imutaveis
// 2- Referencias devem sempre ser validas
