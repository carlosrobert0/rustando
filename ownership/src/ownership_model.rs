// Modelos de Gerenciamento de Memória em Linguagens de Programação

/*
    Garbage Collector: O coletor de lixo é um sistema que automaticamente gerencia a memória,
    liberando memória que não é mais necessária.
    Prós: Fácil de usar, não requer gerenciamento manual de memória.
    Contras: Pode causar pausas no programa para coletar lixo, o que pode afetar o desempenho.

    Gerenciamento de Memória Manual: O programador é responsável por alocar e liberar memória.
    Prós: Maior controle sobre a memória, pode ser mais eficiente em termos de desempenho.
    Contras: Maior risco de vazamentos de memória e erros, como acessar memória liberada.

    Ownership: O modelo de ownership é um sistema de gerenciamento de memória que garante
    que cada valor tenha um único proprietário, e a memória é liberada automaticamente
    quando o proprietário sai de escopo.
    Prós: Evita vazamentos de memória, não requer coleta de lixo, é eficiente em termos de desempenho.
    Contras: Pode ser mais difícil de entender para iniciantes, requer que o programador pense
    sobre a propriedade dos valores.

    Rust utiliza o modelo de ownership para gerenciar a memória, evitando a necessidade de um
    coletor de lixo.
*/
