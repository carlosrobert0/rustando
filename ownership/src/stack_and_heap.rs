/*
    A pilha (stack) é usada para armazenar dados de tamanho fixo e conhecidos em tempo de compilação,
    enquanto a heap é usada para armazenar dados de tamanho variável, dinâmico, ou desconhecido em tempo de
    compilação. O Rust utiliza um modelo de ownership para gerenciar a memória na heap, garantindo
    que a memória seja liberada automaticamente quando não for mais necessária, evitando vazamentos de
    memória e outros problemas comuns em linguagens que usam gerenciamento de memória manual ou
    garbage collector.

    Stack: A pilha armazena valores na ordem em que eles chegam, e os remove na ordem inversa
    (LIFO - Last In, First Out). O último valor adicionado é o primeiro a ser removido.
    O gerenciamento de memória na pilha é automático e rápido, pois as variáveis são liberadas quando saem de escopo.
    Os tipos escalares, como inteiros, floats, caracteres, booleanos e tuplas, composta por dados de tipos escalares,
    são armazenados na pilha.

    Heap: A heap é mais lenta que a pilha, pois envolve alocação dinâmica e gerenciamento de memória.
    Tipos compostos, como vetores, strings e estruturas, são armazenados na heap.
    A alocação de memória na heap é feita através de ponteiros, que são referências a locais na heap.
    E o Rust utiliza o modelo de ownership para garantir que a memória seja liberada quando não for mais necessária.
*/

fn main() {
    // Exemplo de uso da pilha
    let subscriber_count: i32 = 100; // valor fixo de tipo inteiro com tamanho conhecido
    let average_age: u8 = 30; // valor fixo de tipo inteiro sem sinal com tamanho conhecido
    let average_rating: f64 = 4.5; // valor fixo de tipo float de 64 bits com tamanho conhecido
    let is_monetized: bool = true; // valor fixo de tipo booleano com tamanho conhecido
    let first_letter: char = 'R'; // valor fixo de tipo caractere com tamanho conhecido
    let data_tuple = (
        subscriber_count,
        average_age,
        average_rating,
        is_monetized,
        first_letter,
    ); // tupla com dados de tipos escalares

    println!(
        "Subscriber Count: {}, Average Age: {}, Average Rating: {}, Is Monetized: {}, First Letter: {}",
        subscriber_count, average_age, average_rating, is_monetized, first_letter
    ); // Imprime os valores escalares

    println!("Data Tuple: {:?}", data_tuple); // Imprime a tupla completa

    /*
       Exemplo de uso da heap
       A heap é usada para armazenar dados de tamanho variável ou desconhecido em tempo de compilação.
       Aqui, estou criando uma String, que é um tipo composto que armazena dados na heap e precisa do modelo de ownership
       para gerenciar sua memória.

       Aplicando as regras de ownership:
           1. Cada valor em Rust tem uma variável que é seu "dono" (owner).
              Aqui, a variável `name` é o dono da String alocada na heap.
           2. Um valor pode ter apenas um dono por vez.
              Aqui, `name` é o único dono da String.
           3. Quando o dono de um valor sai de escopo, o valor será descartado (liberado, destruído).
              Veremos a seguir.
    */
    let name = String::from("Rustando"); // Aqui estou alocando memória na heap para armazenar a string
    println!("{}", name); // Acesso ao valor alocado na heap

    let another_name = name; // A propriedade de `name` é movida para `another_name`
    // `another_name` é o novo dono da String alocada na heap. `name` não é mais válido aqui.

    println!("Another Name: {}", another_name); // Acesso ao novo dono da String, que é `another_name`, que é válido aqui

    println!("Name: {}", name); // Isso causa um erro, pois `name` não é mais válido aqui, pois sua
    // propriedade foi movida para `another_name`. O Rust garante que não possamos acessar `name` após a movimentação.
    // A memória alocada para a String será liberada automaticamente quando `another_name` sair de escopo.

    // Veremos a seguir sobre escopo de variáveis.
}
