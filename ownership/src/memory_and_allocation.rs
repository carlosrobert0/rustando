/*
    Para copiar valores escalares, como inteiros, floats, booleanos e caracteres,
    o Rust utiliza o Copy trait, que permite que esses valores sejam copiados automaticamente.

    Para copiar valores que são alocados na heap, como Strings, o Rust não permite a cópia automática.
    Em vez disso, a propriedade do valor é movida para outra variável.

    Para copiar um valor alocado na heap, como uma String, você precisa usar o método clone(),
    que cria uma cópia do valor e aloca nova memória na heap para a cópia.
*/

fn main() {
    let original_string = String::from("Hello, Rust!"); // String original alocada na heap

    let copied_string = original_string; // A propriedade de original_string é movida para copied_string
    // original_string não é mais válido aqui, pois sua propriedade foi movida para copied_string

    println!("Copied String: {}", copied_string); // Acesso ao novo dono da String, que é copied_string
    println!("Original String: {}", original_string); // Isso causaria um erro, pois original_string não é mais válido aqui

    let cloned_string = copied_string.clone(); // Clonando a String para criar uma nova cópia
    println!("Cloned String: {}", cloned_string); // Acesso à cópia da String, que é cloned_string
    println!("Copied String after clone: {}", copied_string); // copied_string ainda é válido aqui por causa do clone

    // A memória alocada para cloned_string será liberada automaticamente quando sair de escopo
    // A memória alocada para copied_string também será liberada automaticamente quando sair de escopo
}
