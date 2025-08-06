/*
    Ownership e Funções

    Em Rust, as funções também seguem as regras de ownership.
    Quando você passa uma variável para uma função, a propriedade dessa variável é movida para a função.
*/

fn main() {
    let name = String::from("Rustando"); // `name` é uma String que possui a propriedade da string

    // Passando a string para a função
    show_channel_name(name);

    // Após a chamada da função, `name` não é mais válido, pois a propriedade foi movida
    println!("Canal: {}", name); // Isso causaria um erro, pois `name` não é mais válido
}

fn show_channel_name(name: String) {
    println!("Canal: {}", name);
}
