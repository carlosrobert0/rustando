# Correções nos vídeos que já estão no ar

> Levantado em 15/10/2026 a partir das descrições reais (vidIQ) e da playlist pública.
> **Lote 1** = defeitos concretos, com título e descrição finais prontos.
> **Lote 2** = só acrescentar navegação (playlist + aula anterior/próxima) no fim da descrição.
> Marque `[x]` quando aplicar. Aplicar pelo vidIQ custa 5 créditos por vídeo; colar no Studio é de graça.

## Playlist "Curso de Rust do Zero [2025] – Completo e Prático" (só pelo Studio)

Link: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

- [ ] **Colocar o #5 p1 antes do #5 p2.** Hoje quem termina o #5 p2 cai no #5 p1, de trás pra frente.
- [ ] **Adicionar o #8** (https://youtu.be/XrTfvp0EZWs) entre o #7 e o #9. Ele está fora da playlist.
- [ ] Trocar `[2025]` por `[2026]` no nome da playlist.
- [ ] Criar a playlist **"Projetos em Rust"** quando o projeto #1 sair.
- [ ] (Opcional) Pôr o vídeo de instalação no macOS logo depois do #2.

Ordem certa: Começando com Rust → #1 → #2 → #3 → #4 p1 → #4 p2 → #5 p1 → #5 p2 → #6 → #7 → #8 → #9 → #10 → #11 p1 → #11 p2.

---

## Lote 1 — defeitos concretos

### [ ] #9 — `p-ByiaX3jSU` (título fora do padrão + capítulos quebrados)

**Problema:** o título começa com "Rust do Zero" (sem "Curso de Rust", a busca que mais traz gente pro canal) e
**todos os capítulos estão em 00:00**, então o YouTube ignora a lista. Capítulos abaixo gerados pela
transcrição (vidIQ); vale conferir 2 ou 3 no player.

**Título:**
```text
Curso de Rust #9: Vec, String e HashMap — as 3 coleções que você VAI usar sempre 🦀
```

**Descrição:**
```text
Nesse episódio #9 do Curso de Rust em Português a gente mergulha nas Common Collections: os três tipos que guardam múltiplos valores na heap e que você vai usar em praticamente todo projeto Rust — Vec, String e HashMap.

Diferente de array e tupla (que têm tamanho fixo e vivem na stack), as coleções crescem e diminuem em tempo de execução. E é aqui que ownership e borrow checker começam a aparecer de um jeito bem prático — inclusive naquele erro clássico de fazer push num vector enquanto ainda tem uma referência viva.

🦀 O QUE VOCÊ VAI APRENDER
• Vec: criar com Vec::new() e vec![], push, ler com índice vs .get() (panic vs Option)
• Por que o borrow checker barra "let first = ref de v[0]; v.push(6);" — e o que a realocação tem a ver com isso
• Percorrer vector por referência imutável e mutável (e o famoso asterisco da desreferência)
• Vector com "tipos diferentes" usando enum + match
• String: String::new(), to_string(), String::from(), push_str, push
• Concatenação com + (e a pegadinha do move do s1!) vs o format! que não toma posse de nada
• Por que NÃO dá pra indexar String em Rust — UTF-8, bytes e char boundaries ("Здравствуйте" tem 24 bytes, não 12!)
• O jeito certo de percorrer texto: .chars() e .bytes()
• HashMap: insert, get + copied + unwrap_or, iteração em ordem arbitrária
• Ownership no HashMap: tipos Copy são copiados, String é MOVIDA
• entry() + or_insert(): sobrescrever vs inserir só se não existir
• O exemplo clássico: contar palavras de um texto em poucas linhas

⏱️ CAPÍTULOS
0:00 Introdução — o que são Common Collections
1:03 Criando o projeto
2:29 Vector: o que é e quando usar
4:46 Criando: Vec::new() e vec!
7:50 Adicionando valores (e o mut)
10:02 Lendo: índice vs .get()
14:19 .get() devolve Option: tratando com match
19:03 Borrow checker no vector
22:15 Percorrendo o vector
25:40 Vector de tipos diferentes com enum
31:30 String, UTF-8 e por que não dá pra indexar
41:18 HashMap e contando palavras

💻 CÓDIGO DA AULA
https://github.com/carlosrobert0/rustando/tree/main/collections

📚 REFERÊNCIA
The Rust Programming Language — Capítulo 8 (Common Collections)
https://doc.rust-lang.org/book/ch08-00-common-collections.html

📺 PLAYLIST COMPLETA DO CURSO
https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ AULA ANTERIOR
Curso de Rust #8 — Módulos: https://youtu.be/XrTfvp0EZWs

➡️ PRÓXIMA AULA
Curso de Rust #10 — Tratamento de Erros: https://youtu.be/IgOrxvzHmGE

Se esse vídeo te ajudou, deixa o like 👍, se inscreve no canal 🔔 e comenta aí qual coleção você mais usa. Bora rustando! 🦀🧡

#rust #rustlang #programação #curso #desenvolvimento
```

---

### [ ] #10 — `IgOrxvzHmGE` (capítulos quebrados + links de exemplo + sem legenda)

**Problema:** todos os capítulos em 00:00, `[link da playlist]` no lugar do link, anterior/próximo sem link, e o
link do código (`/tree/main/erros`) dá 404 até a pasta `erros/` ser enviada pro GitHub. **O vídeo também não
tem nenhuma legenda**, nem automática: confira em Studio → Legendas. Sem legenda, o YouTube não sabe o
que você fala no vídeo, e o vidIQ não consegue gerar os capítulos. Por isso os capítulos saem da descrição
até ter os tempos certos.

**Título:** manter.

**Descrição:**
```text
Rust não tem exceptions — e isso é de propósito. Nesse episódio #10 do Curso de Rust em Português a gente vê como a linguagem separa os erros em duas categorias e força você a lidar com cada uma na hora certa: panic! para erros irrecuperáveis e Result para erros recuperáveis.

A grande sacada é que o compilador não deixa você ignorar um erro por acidente. Se uma função pode falhar, isso aparece no tipo de retorno — e você decide o que fazer.

O ponto alto é o operador ?: ele transforma aquele match gigante de propagação de erro em um único caractere.

🦀 O QUE VOCÊ VAI APRENDER
• panic! — quando o programa não tem como continuar
• unwinding vs abort e o RUST_BACKTRACE=1 para achar a origem do panic
• Result: os casos Ok e Err, e por que ele é só um enum
• Tratando erros com match: arquivo não existe? Cria o arquivo
• Atalhos: unwrap, expect e por que expect é quase sempre melhor
• unwrap_or, unwrap_or_else e unwrap_or_default
• Propagando erro na mão com match e return Err
• O operador ? — a mesma coisa em uma linha
• Encadeando o ? em chamadas seguidas
• Por que o ? só funciona em funções que retornam Result ou Option
• main() retornando Result e o que o Rust faz com o código de saída
• Criando um tipo que garante o dado válido: o exemplo do Guess (1..=100)
• panic! ou Result? O guia prático de quando usar cada um
• Bônus: thiserror para criar seus erros e anyhow para prototipar rápido

💻 CÓDIGO DA AULA
https://github.com/carlosrobert0/rustando/tree/main/erros

📚 REFERÊNCIA
The Rust Programming Language — Capítulo 9 (Error Handling)
https://doc.rust-lang.org/book/ch09-00-error-handling.html

📺 PLAYLIST COMPLETA DO CURSO
https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ AULA ANTERIOR
Curso de Rust #9 — Vec, String e HashMap: https://youtu.be/p-ByiaX3jSU

➡️ PRÓXIMA AULA
Curso de Rust #11 — Generics (Parte 1): https://youtu.be/LSKlmBGf-VA

Se esse vídeo te ajudou, deixa o like 👍, se inscreve 🔔 e comenta aí: você é do time unwrap ou do time expect? Bora rustando! 🦀🧡

#rust #rustlang #programação #curso #desenvolvimento
```

---

### [ ] #4 Ownership — `0gmJARCCPO8` (dois vídeos chamados "#4" + link de exemplo)

**Problema:** existem dois "#4" (Variáveis e Ownership). E a descrição tem `playlist?list=...` no lugar do link.

**Título:**
```text
Curso de Rust #4 (Parte 2): Ownership, Referências e Slices — Entenda o sistema de memória do Rust
```

**Descrição:**
```text
Na parte 2 da aula #4 do Curso de Rust, você vai entender um dos conceitos mais importantes e revolucionários da linguagem: o Ownership. Vamos explorar como o Rust gerencia a memória de forma segura e eficiente — sem garbage collector!

📚 O que você vai aprender nesta aula:

O que é Ownership em Rust

Regras de propriedade e validade

Referências e Borrowing (imutáveis e mutáveis)

Slices e como funcionam na prática

Demonstrações de código no VSCode

Casos comuns e erros frequentes explicados

🔥 Entender o sistema de propriedade do Rust é essencial para se tornar um desenvolvedor produtivo nessa linguagem. Não pule essa aula!

🦀 Documentação oficial usada:

Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

Referências e Empréstimos: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

Slices: https://doc.rust-lang.org/book/ch04-03-slices.html

💻 Código da aula: https://github.com/carlosrobert0/rustando/tree/main/ownership

📺 Playlist completa do Curso de Rust: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ Aula anterior — #4 (Parte 1) Variáveis, tipos e funções: https://youtu.be/9YXmfytr_CQ
➡️ Próxima aula — #5 Structs (Parte 1): https://youtu.be/wJPU__13Ek8

🔔 Inscreva-se no canal e ative as notificações para acompanhar as próximas aulas.

👍 Gostou do conteúdo? Deixe seu like, comente e compartilhe com amigos que querem aprender Rust!

#RustLang #CursoDeRust #Ownership #Borrowing #Slices #RustDoZero #Programacao #Desenvolvimento #RustTutorial
```

---

### [ ] #4 Variáveis — `9YXmfytr_CQ` (par do anterior + referências sem link)

**Problema:** vira "Parte 1" do #4. E a descrição diz "→ Livro Oficial em PT-BR / → Versão em Inglês"
sem os links.

**Título:**
```text
Curso de Rust #4 (Parte 1): Variáveis, Tipos, Funções e Controle de Fluxo em Rust
```

**Descrição:**
```text
Aprenda os principais conceitos fundamentais de programação em Rust! Nesta aula, vamos abordar as bases essenciais da linguagem, como variáveis mutáveis, tipos de dados, funções, comentários e controle de fluxo com if, loop, while e for.

Este é o Capítulo 3 do livro oficial The Rust Programming Language, traduzido em português — e vamos acompanhar passo a passo com exemplos práticos no VSCode.

📚 Referência do conteúdo:
→ Livro oficial em PT-BR: https://rust-br.github.io/rust-book-pt-br/ch03-00-common-programming-concepts.html
→ Versão em inglês: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

🔧 O que você vai aprender:

Como declarar e modificar variáveis

Diferença entre let, mut, const e shadowing

Tipos escalares e compostos em Rust

Como escrever funções com parâmetros e retorno

Comentários no código

Controle de fluxo: if, match, loop, while, for

🚀 Dominar esses conceitos é essencial para seguir em frente no Rust com segurança e produtividade.

💻 Código da aula: https://github.com/carlosrobert0/rustando/tree/main/common-concepts

📺 Playlist completa do Curso de Rust: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ Aula anterior — #3 Jogo de adivinhação: https://youtu.be/SWn-JMEurs4
➡️ Próxima aula — #4 (Parte 2) Ownership: https://youtu.be/0gmJARCCPO8

Se inscreva no canal, ative o sininho e continue aprendendo!

#rustlang #programacao #cursoRust #rust
```

---

### [ ] #6 Enums — `5Ub6tYVRFYc` (a primeira linha é um pedido de desculpas)

**Problema:** a primeira linha da descrição, que aparece na busca, é "Obs.: Caso ache lento, sugiro assistir
com a velocidade em 1.25x ou 1.5x". Ela vai pro fim.

**Título:** manter.

**Descrição:**
```text
Neste vídeo da série Curso de Rust, você vai aprender como trabalhar com Enums e Pattern Matching, dois pilares fundamentais da linguagem Rust para criar código seguro, expressivo e livre de erros comuns.

Vamos explorar como definir e usar enum, entender o poderoso match e também as formas mais concisas de tratar padrões com if let, if let else e o moderno let else. Além disso, você vai conhecer o enum mais usado do Rust: o Option, essencial para lidar com valores que podem ou não existir.

Você vai aprender:

✅ O que são enums e quando usar
✅ Como criar e instanciar enums no Rust
✅ Como usar match para tratar todos os casos possíveis
✅ O que é pattern matching e por que ele é tão poderoso
✅ Como simplificar código com if let e if let else
✅ Como usar let else para validações rápidas
✅ Como funciona o enum Option T (Some e None)
✅ Diferenças entre match, if let e let else na prática
✅ Exemplos reais de uso no dia a dia

Tudo com exemplos práticos e explicações diretas para você dominar enums e controle de fluxo no Rust!

💻 Código da aula: https://github.com/carlosrobert0/rustando/tree/main/enums

📺 Playlist completa do Curso de Rust: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ Aula anterior — #5 (Parte 2) Structs e métodos: https://youtu.be/aXBSyv3EIFY
➡️ Próxima aula — #7 Sistema de módulos na prática: https://youtu.be/fKNp_Ufiw_Q

Dica: se achar o ritmo lento, assista em 1.25x ou 1.5x.

#rust #rustlang #programacao #backend #desenvolvimento #cursoderust #enum #match #patternmatching #iflet #option #sistemas #tecnologia
```

---

### [ ] #11 Parte 1 — `LSKlmBGf-VA` (não aponta pra Parte 2)

**Título:** manter. **Descrição:** trocar a linha `➡️ Na parte 2: generics em structs, enums e métodos.` por:
```text
➡️ Parte 2 (generics em structs e métodos): https://youtu.be/C5vEZ_uUmt4

📺 Playlist completa do Curso de Rust: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B
⬅️ Aula anterior — #10 Tratamento de erros: https://youtu.be/IgOrxvzHmGE
```
O link do código (`/tree/main/generics`) só funciona depois que a pasta `generics/` for enviada pro GitHub.

### [ ] #11 Parte 2 — `C5vEZ_uUmt4` (link da Parte 1 começa no meio)

**Título:** manter. **Descrição:** trocar `⬅️ Parte 1 (generics em funções): https://www.youtube.com/watch?v=LSKlmBGf-VA&t=209s`
(o `&t=209s` abre o vídeo em 3:29) por:
```text
⬅️ Parte 1 (generics em funções): https://youtu.be/LSKlmBGf-VA

📺 Playlist completa do Curso de Rust: https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B
```
**Conferir os capítulos:** a lista acaba em 5:00 num vídeo de 14:37 e parece ter vindo das estimativas do
roteiro. Quando a legenda automática ficar pronta, peça pro Claude gerar os capítulos certos.

---

## Lote 2 — acrescentar navegação no fim da descrição

Playlist = `https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B`

- [ ] **Começando com Rust** `iZd-s9THv1U` (traz 1/3 dos inscritos do canal). **Não mexer no título.** Colar
      logo depois do primeiro parágrafo:
  ```text
  ▶️ Próximo passo: seu primeiro projeto em Rust — Curso de Rust #3, jogo de adivinhação: https://youtu.be/SWn-JMEurs4
  📺 Curso de Rust completo e gratuito (playlist): https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B
  ```
  E **fixar um comentário** com o mesmo texto.
- [ ] **#1** `hi6KAMqrb1M`: trocar "📌 Próxima aula: Instalando Rust..." por
      `➡️ Próxima aula — #2 Instalando Rust no Windows: https://youtu.be/rG3Bnn4cxcM` + playlist.
- [ ] **#2** `rG3Bnn4cxcM`: `⬅️ #1 Por que aprender Rust: https://youtu.be/hi6KAMqrb1M` ·
      `➡️ #3 Jogo de adivinhação: https://youtu.be/SWn-JMEurs4` · `🍎 No Mac? https://youtu.be/hhkLo6GHLlc` + playlist.
- [ ] **#3** `SWn-JMEurs4`: `💻 https://github.com/carlosrobert0/rustando/tree/main/guessing_game` ·
      `➡️ #4 (Parte 1) Variáveis, tipos e funções: https://youtu.be/9YXmfytr_CQ` + playlist.
- [ ] **#5 p1** `wJPU__13Ek8`: `💻 .../tree/main/structs` · `⬅️ #4 (Parte 2) Ownership: https://youtu.be/0gmJARCCPO8` ·
      `➡️ #5 (Parte 2): https://youtu.be/aXBSyv3EIFY` + playlist.
- [ ] **#5 p2** `aXBSyv3EIFY`: `💻 .../tree/main/structs` · `⬅️ #5 (Parte 1): https://youtu.be/wJPU__13Ek8` ·
      `➡️ #6 Enums e pattern matching: https://youtu.be/5Ub6tYVRFYc` + playlist.
- [ ] **#7** `fKNp_Ufiw_Q`: `💻 https://github.com/carlosrobert0/rustando` (confirme se é a pasta `restaurant/` ou `restaurante/` e aponte direto) · `⬅️ #6 Enums: https://youtu.be/5Ub6tYVRFYc` ·
      `➡️ #8 Módulos, crates e privacidade: https://youtu.be/XrTfvp0EZWs` + playlist.
- [ ] **#8** `XrTfvp0EZWs`: apagar a linha `Inscrito? [INSCREVA-SE] para não perder nenhum vídeo!` e colar
      `💻 https://github.com/carlosrobert0/rustando` (idem) · `⬅️ #7: https://youtu.be/fKNp_Ufiw_Q` ·
      `➡️ #9 Vec, String e HashMap: https://youtu.be/p-ByiaX3jSU` + playlist.
- [ ] **macOS** `hhkLo6GHLlc`: `➡️ Depois de instalar: Curso de Rust #3, jogo de adivinhação: https://youtu.be/SWn-JMEurs4` + playlist.

## Shorts

- [ ] `k1Tb7HOAmFg` "Details in nature" não é sobre Rust: deixar **privado**. Teve 382 views e nenhum inscrito.
- [ ] Nos Shorts que continuarem públicos, usar o campo **"Vídeo relacionado"** do Studio pra apontar pra aula
      completa (ex.: os Shorts de struct → #5 p1).

## Depois que o projeto #1 sair

- [ ] Acrescentar nas descrições do #9 e do #10: `🛠️ Na prática: Projeto Rust #1 — contador de palavras: <link>`
- [ ] Tela final do #10 e do #11 p2 apontando pro projeto #1.
