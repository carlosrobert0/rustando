/*
    Slices são referências a partes de coleções de dados,
    como Strings, arrays, ou outros tipos de coleções.
    Slices são definidos com o símbolo `&`, e permitem acessar
    uma parte de uma coleção sem transferir a propriedade.
    Isso é útil quando você quer trabalhar com uma parte de uma coleção
    sem precisar copiar ou mover os dados.

    As regras de slices são:
        1. Slices são referências, então não transferem a propriedade dos dados.
        2. Slices podem ser usadas para acessar partes de Strings, arrays, ou outros tipos de coleções.
        3. Slices são úteis para evitar cópias desnecessárias de dados.
        4. Slices são imutáveis por padrão, mas podem ser mutáveis se necessário.
        5. Slices não têm ownership, então não há necessidade de liberar memória manualmente.
*/

fn main() {
    let string = String::from("Ola mundo!");
    let hello = &string[0..3]; // Pega "Ola"

    let world = &string[4..9]; // Pega "mundo"
    println!("Slice de String: {}, {}", hello, world);

    // Podemos omitir o índice inicial ou final
    let hello2 = &string[..3]; // Pega "Ola"
    let world2 = &string[4..]; // Pega "mundo!"
    let whole_string = &string[..]; // Pega a string inteira

    println!("Slice de String: {}, {}, {}", hello2, world2, whole_string);

    let first = first_word(&string);

    println!("Primeira palavra: {}", first);

    let second = second_word(&string);
    println!("Segunda palavra: {}", second);

    string_literals();

    other_slices();
}

fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}

fn second_word(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[i + 1..];
        }
    }

    &string[..]
}

fn string_literals() {
    let hello = "Olá"; // &str
    let world = "mundo"; // &str
    // As strings literais são slices, então não há necessidade de alocar memória
    println!("String Literal: {}, {}", hello, world);
}

fn other_slices() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..3]; // Pega os elementos 2 e 3
    println!("Slice de Array: {:?}", slice);
    // Slices de arrays são úteis para acessar partes de coleções sem transferir a propriedade
}
