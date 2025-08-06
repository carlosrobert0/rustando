/*
    Escopo de Variáveis em Rust

    O escopo de uma variável em Rust é o bloco de código onde a variável é válida e pode ser usada.
    As variáveis são criadas quando são declaradas e são destruídas quando saem de escopo.

    O escopo é definido por chaves `{}` e pode ser aninhado, ou seja, um escopo pode conter outros escopos.
    Variáveis declaradas dentro de um escopo não podem ser acessadas fora desse escopo.
    Quando uma variável sai de escopo, a memória que ela ocupava é liberada automaticamente pelo Rust.
*/

fn main() {
    let subscriber_count = 100;
    let average_age = 30;

    {
        let average_rating = 4.5;
        let is_monetized = true;
        let first_letter = 'R';

        println!(
            "Subscriber Count: {}, Average Age: {}, Average Rating: {}, Is Monetized: {}, First Letter: {}",
            subscriber_count, average_age, average_rating, is_monetized, first_letter
        );
    }

    // A variável average_rating, is_monetized e first_letter não são mais válidas aqui, pois saíram de escopo
    // e a memória que ocupavam foi liberada automaticamente pelo Rust.

    println!(
        "Subscriber Count: {}, Average Age: {}",
        subscriber_count, average_age
    );

    {
        let average_age_copy = average_age; // average_age_copy é uma cópia (Copy) de average_age
        println!("Average Age Copy: {}", average_age_copy); // average_age_copy é válida aqui
    }

    // average_age_copy sai de escopo aqui, mas average_age ainda é válida, pois é um tipo Copy
    println!("Average Age: {}", average_age); // average_age ainda é válida aqui
    // Copy trait é aplicado a tipos primitivos como inteiros, floats, booleanos e caracteres.
    // Tipos que implementam Copy são copiados automaticamente quando atribuídos a outra variável.
    // E não precisam de aplicar o modelo de ownership, pois não são movidos.

    // Exemplo de uso do tipo String, que é um tipo composto e precisa do modelo de ownership

    let name = String::from("Rustando"); // name é válida neste escopo

    {
        let name_copy = name; // A propriedade de name é movida para name_copy
        println!("Name Copy: {}", name_copy); // name_copy é válida aqui
    }

    println!("Name: {}", name); // Isso causaria um erro, pois name não é mais válida aqui
    // Pois sua propriedade foi movida para name_copy.

    // Veremos a seguir como o Rust gerencia a memória alocada para tipos compostos como String.
}
