// Resumo

// Os conceitos de ownership, borrowing, e slices são o que garante a segurança
// de memória em Rust em tempo de compilação. A linguagem dá controle sobre o uso da memória
// assim como outras linguagens de programacao, mas como o dono dos dados limpa automaticamente
// a memoria quando ele sai de escopo, voce nao tem que escrever e debugar codigo extra
// para ter esse controle. Isso significa que Rust não tem garbage collector,
// e não há necessidade de alocar e desalocar memória manualmente, o que reduz a
// complexidade e o risco de bugs relacionados à memória.
