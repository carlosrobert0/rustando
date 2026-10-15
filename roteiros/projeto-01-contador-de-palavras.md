# Projeto Rust #1 — Contador de palavras (Dom Casmurro)
> Série: Projetos em Rust · Pré-requisitos: aulas #9 (coleções) e #10 (erros)
> Duração alvo: **13–15 min** · Projeto: `contador-palavras/` (código final testado em `src/main.rs`)
> Thumbnail: `contador-palavras/thumbnail-projeto-01.png` (editável no thumbgen pelo `.json`)

**Por que este projeto:** o #3 (jogo de adivinhação) é o vídeo do canal com melhor retenção e mais tempo de
exibição. E os vídeos "Learn Rust: Build X" em inglês fizeram de 20 a 100 vezes a média dos canais. Este projeto
usa exatamente o que as aulas #9 e #10 ensinaram (HashMap, String, Result, `?`) e dá um motivo pra quem parou
nelas voltar.

---

## Antes de gravar (checklist)

- [ ] Baixar **Dom Casmurro** em texto: gutenberg.org → buscar "Dom Casmurro" → **Plain Text UTF-8**.
      Salvar como `contador-palavras/dom-casmurro.txt`.
- [ ] **Apagar o cabeçalho e o rodapé em inglês** do Gutenberg (tudo antes de `*** START OF` e depois de
      `*** END OF`). Senão "the", "project" e "gutenberg" entram no ranking.
- [ ] Rodar a versão final uma vez e **anotar os números reais** pra preencher os `[PREENCHER]` deste roteiro:
  ```bash
  cargo run --release -- dom-casmurro.txt 15
  cargo run -- dom-casmurro.txt 15
  ```
  - total de palavras: `[PREENCHER]` · tempo em release: `[PREENCHER]` · tempo em debug: `[PREENCHER]`
  - top 3 depois do filtro: `[PREENCHER]`
  - Referência do meu teste (texto de 76,5 mil palavras): **2,4 ms em release, 39 ms em debug** (16x).
- [ ] Se o tempo real não for ~3 ms, trocar o "EM 3 MS" da thumbnail (no thumbgen ou no `.html`) e regerar o PNG.
- [ ] Ver se sobrou palavra genérica no top 10 (ex.: "capítulo", "disse"). Se sobrou, **deixar pra adicionar
      na lista ao vivo** no passo 6: é um momento bom pro vídeo.
- [ ] Ter `src/main.rs` final aberto numa segunda janela como cola.

## Regras desta gravação (do diagnóstico do canal)

- **Nada de apresentação, vinheta ou "fala pessoal" antes do gancho.** No melhor vídeo do canal, 60% da
  audiência sai até 0:36. O nome do canal aparece só no fim.
- **Resultado na tela nos primeiros 5 segundos.**
- Pedir like/inscrição **só depois da revelação** (passo 6), nunca no começo.
- Fechar em **no máximo 15 min**. Se passar, corta o passo 8 e vira Parte 2.
- Últimos 20 s: tela final apontando pro próximo vídeo + playlist.

---

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** terminal com `cargo run --release -- dom-casmurro.txt` já digitado. Aperta Enter. O ranking
com as barras aparece. **A linha 1 fica borrada/coberta na edição.**

**Fala:**
> "Esse programa leu Dom Casmurro inteiro — [PREENCHER: 66] mil palavras — em [PREENCHER: 3] milissegundos.
> E descobriu qual palavra o Machado de Assis mais repete no livro."

**Tela (0:10):** zoom na linha 1 borrada.

> "Hoje você vai escrever esse programa do zero, em Rust, com menos de cem linhas.
> E já aviso: a primeira resposta que ele dá tá errada. Eu vou te mostrar por quê — e como consertar.
> Abre o terminal e vem comigo."

**Tela (0:28):** corte seco pro terminal vazio. Sem vinheta.

---

## 1. O que vamos construir (0:30 – 1:00)

**Mostrar na tela:** o resultado final de novo (ainda com a linha 1 coberta) e três tópicos:

> "Três coisas: ler um arquivo, contar cada palavra com um HashMap e montar o ranking.
> Se HashMap ou o operador interrogação forem novidade, os links das aulas #9 e #10 estão na descrição —
> mas dá pra acompanhar sem elas."

---

## 2. Criando o projeto e lendo o livro (1:00 – 2:30)

```bash
cargo new contador-palavras
cd contador-palavras
```

Colocar o `dom-casmurro.txt` na raiz do projeto (ao lado do `Cargo.toml`).

```rust
use std::env;
use std::fs;

fn main() {
    let caminho = env::args().nth(1).expect("passe o caminho do arquivo");
    let texto = fs::read_to_string(&caminho).expect("não consegui ler o arquivo");

    println!("{} caracteres", texto.chars().count());
}
```

```bash
cargo run -- dom-casmurro.txt
```

**Fala:**
> "`env::args()` são os argumentos da linha de comando. O primeiro é o nome do programa, por isso o `nth(1)`.
> E `read_to_string` joga o livro inteiro numa `String`. Por enquanto vou de `expect` — no final a gente
> troca por tratamento de erro de verdade."

**Ponto de ensino:** `chars().count()` e não `len()`: `len()` conta **bytes**, e "ç" e "ã" ocupam 2 bytes
cada em UTF-8 (lembra da aula #9).

---

## 3. Separando as palavras (2:30 – 4:30)

**Primeira tentativa — ERRO PROPOSITAL (o resultado sai errado, não o compilador):**

```rust
    let texto = texto.to_lowercase();

    for palavra in texto.split_whitespace().take(12) {
        println!("{palavra}");
    }
```

**Mostrar a saída:** aparecem palavras com vírgula, ponto ou exclamação grudados (`casmurro.`) e travessões
soltos (`—`).

**Fala:**
> "Separar por espaço não basta: a pontuação vem grudada. 'Capitu' e 'Capitu,' iam virar duas palavras
> diferentes."

**Correção:**

```rust
    let palavras = texto
        .split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty());

    for palavra in palavras.take(12) {
        println!("{palavra}");
    }
```

**Pontos de ensino:**
- `let texto = texto.to_lowercase();` é **shadowing** (aula #4): mesmo nome, valor novo.
- `split` com uma closure: corta em tudo que **não** é letra nem número.
- `is_alphanumeric()` entende Unicode, então "coração" e "ação" saem inteiras.
- Cada `palavra` é um `&str`, uma **fatia do próprio texto**. Nenhuma palavra é copiada.

---

## 4. Contando com HashMap (4:30 – 6:30)

**Fala:**
> "Lembra do fim da aula #9, quando a gente contou as palavras de uma frase? Agora é um livro inteiro."

```rust
use std::collections::HashMap;

fn contar_palavras(texto: &str) -> HashMap<&str, usize> {
    let mut contagem = HashMap::new();

    for palavra in texto.split(|c: char| !c.is_alphanumeric()) {
        if palavra.is_empty() {
            continue;
        }
        *contagem.entry(palavra).or_insert(0) += 1;
    }

    contagem
}
```

No `main`:

```rust
    let contagem = contar_palavras(&texto);
    let total: usize = contagem.values().sum();
    println!("{total} palavras ({} diferentes)", contagem.len());
```

**Mostrar a saída:** `[PREENCHER] palavras ([PREENCHER] diferentes)`.

**Pontos de ensino:**
- `entry(palavra).or_insert(0)` devolve uma referência mutável pro contador. O `*` desreferencia pra somar 1.
- A chave é `&str` apontando pro `texto`. O compilador garante que o `HashMap` não vive mais que o texto.
  > "Isso tem nome: lifetime. É o assunto da próxima aula do curso."

---

## 5. O ranking — e a resposta errada (6:30 – 8:30)

```rust
fn mais_frequentes(contagem: HashMap<&str, usize>, quantidade: usize) -> Vec<(&str, usize)> {
    let mut lista: Vec<(&str, usize)> = contagem.into_iter().collect();
    lista.sort_by(|a, b| b.1.cmp(&a.1));
    lista.truncate(quantidade);
    lista
}
```

No `main` — **ERRO PROPOSITAL** (usar `contagem` depois de entregar ela pra função):

```rust
    let contagem = contar_palavras(&texto);
    let ranking = mais_frequentes(contagem, 10);
    println!("{} palavras diferentes", contagem.len());
```

**Compilar e mostrar o erro:**
```
error[E0382]: borrow of moved value: `contagem`
11 |     let ranking = mais_frequentes(contagem, 10);
   |                                   -------- value moved here
12 |     println!("{} palavras diferentes", contagem.len());
   |                                        ^^^^^^^^ value borrowed here after move
```

**Fala:**
> "Ownership da aula #4 na prática: a função recebeu o HashMap **por valor** — agora ele é dela.
> Solução: pegar o que eu preciso antes de entregar."

```rust
    let contagem = contar_palavras(&texto);
    let total: usize = contagem.values().sum();
    let diferentes = contagem.len();
    let ranking = mais_frequentes(contagem, 10);

    println!("{total} palavras ({diferentes} diferentes)\n");

    for (posicao, (palavra, vezes)) in ranking.iter().enumerate() {
        println!("{:>2}. {palavra:<12} {vezes:>5}", posicao + 1);
    }
```

**Rodar. Mostrar o top:** "que", "de", "a", "o"...

**Fala (o gancho do meio do vídeo):**
> "Pronto: segundo o programa, a palavra favorita do Machado de Assis é... 'que'. Tecnicamente certo, e
> completamente inútil. Toda língua tem essas palavras que aparecem em qualquer texto. Vamos tirar elas —
> e aí aparece a resposta de verdade."

**Rodar de novo e mostrar que os empates mudam de ordem.** Ponto de ensino: HashMap não tem ordem (aula #9).

---

## 6. Tirando as palavras vazias: a revelação (8:30 – 10:30)

```rust
const PALAVRAS_VAZIAS: &[&str] = &[
    "a", "à", "ao", "aos", "as", "às", "com", "como", "da", "de", "do", "e", "é", "em",
    "na", "não", "no", "o", "os", "para", "por", "que", "se", "um", "uma", /* ... */
];

fn deve_ignorar(palavra: &str) -> bool {
    palavra.chars().count() < 2 || PALAVRAS_VAZIAS.contains(&palavra)
}
```

(A lista completa está no `src/main.rs`: são ~80 palavras.)

Em `mais_frequentes`, filtrar e desempatar:

```rust
    let mut lista: Vec<(&str, usize)> = contagem
        .into_iter()
        .filter(|(palavra, _)| !deve_ignorar(palavra))
        .collect();

    // mais vezes primeiro; empate fica em ordem alfabética
    lista.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
```

**Rodar. REVELAÇÃO** — pausa de 1 segundo antes de mostrar o top 3.

**Fala:**
> "E a palavra que o Machado de Assis mais repete em Dom Casmurro é... **[PREENCHER: palavra]**,
> [PREENCHER: N] vezes."
> [PREENCHER: um comentário de uma frase sobre o resultado — ex.: se for "Capitu", "o narrador passa o livro
> inteiro falando dela, e o programa provou isso em milissegundos".]

**Se sobrou palavra genérica no top 10:** adicionar na lista ao vivo e rodar de novo.

**CTA (único do vídeo, aqui):**
> "Se você não esperava essa, deixa o like — é o que faz esse vídeo chegar em mais gente que tá aprendendo Rust."

---

## 7. Barras, cronômetro e `--release` (10:30 – 12:00)

```rust
use std::time::Instant;

    let inicio = Instant::now();
    // ... ler, contar e ordenar ...
    let tempo = inicio.elapsed();

    println!("{total} palavras ({diferentes} diferentes) em {tempo:.2?}\n");

    let maior = ranking.first().map_or(1, |(_, vezes)| *vezes);
    for (posicao, (palavra, vezes)) in ranking.iter().enumerate() {
        let barra = "█".repeat(vezes * 30 / maior);
        println!("{:>2}. {palavra:<12} {vezes:>5}  {barra}", posicao + 1);
    }
```

**Rodar duas vezes, lado a lado:**
```bash
cargo run -- dom-casmurro.txt            # debug:   [PREENCHER] ms
cargo run --release -- dom-casmurro.txt  # release: [PREENCHER] ms
```

**Fala:**
> "Mesmo código, [PREENCHER: 16] vezes mais rápido. Sem `--release`, o Rust compila rápido pra você testar.
> Com `--release`, ele demora mais pra compilar e otimiza tudo. Pra medir desempenho, sempre `--release`."

---

## 8. Tratamento de erro de verdade (12:00 – 14:00)

**Mostrar o problema:** rodar sem argumento.

```
thread 'main' panicked at src/main.rs:...:
passe o caminho do arquivo
```

**Fala:**
> "Funciona, mas isso é mensagem pra programador, não pra quem usa. Vamos aplicar a aula #10."

```rust
use std::process;

fn main() {
    if let Err(erro) = executar() {
        eprintln!("Erro: {erro}");
        process::exit(1);
    }
}

fn executar() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let caminho = args
        .next()
        .ok_or("uso: cargo run -- <arquivo.txt> [quantidade]")?;
    let quantidade = match args.next() {
        Some(n) => n.parse().map_err(|_| format!("'{n}' não é um número"))?,
        None => 10,
    };

    let texto = fs::read_to_string(&caminho)
        .map_err(|e| format!("não consegui ler '{caminho}': {e}"))?
        .to_lowercase();

    // ... resto igual ...
    Ok(())
}
```

**Demonstrar os 3 erros (saída real):**
```
$ cargo run --release
Erro: uso: cargo run -- <arquivo.txt> [quantidade]

$ cargo run --release -- dom-casmurro.txt dez
Erro: 'dez' não é um número

$ cargo run --release -- nao-existe.txt
Erro: não consegui ler 'nao-existe.txt': No such file or directory (os error 2)
```

**Bônus (se tiver um .txt em Latin-1 à mão):** `stream did not contain valid UTF-8`, porque
`read_to_string` exige UTF-8.

**Pontos de ensino:** `ok_or` transforma `Option` em `Result`. `map_err` troca o erro por uma mensagem
melhor. `?` propaga. E `process::exit(1)` avisa o terminal que deu erro.

---

## 9. Fechamento e desafio (14:00 – 14:45)

**Fala:**
> "Recapitulando: lemos um arquivo, separamos as palavras sem copiar nada, contamos com HashMap,
> ordenamos e tratamos os erros. Tudo o que as aulas #9 e #10 ensinaram, num projeto só.
>
> Desafio: roda esse programa no seu livro favorito e comenta aqui qual palavra ganhou — e de qual livro.
> Eu respondo todo mundo.
>
> O código tá no GitHub, link na descrição. E o próximo projeto [PREENCHER: tema] já tá aparecendo aí na tela.
> Até lá, bora rustando!"

**Tela final (últimos 20 s):** vídeo seguinte (ou a aula #11) + playlist "Curso de Rust do Zero".

---

## Publicação

### Título (recomendado)
```
Projeto em Rust para iniciantes: contei as palavras de Dom Casmurro | Curso de Rust
```
- Começa com o que as pessoas buscam ("projeto em rust", "rust para iniciantes") e termina com
  "Curso de Rust", a busca que mais traz gente pro canal (~80% dos termos de busca identificados).
- Os primeiros ~60 caracteres (até "Dom Casmurro") cabem inteiros na busca do celular. O resto é indexado.
- Evita "Rust" sozinho: no Brasil, as 16 buscas mais comuns com "rust" são todas do jogo. "Projeto",
  "iniciantes" e "Curso" deixam claro que é programação.

**Alternativas** (pra testar no "Testar e comparar" do Studio, se estiver disponível no seu canal):
- `Qual palavra Machado de Assis mais usou? Descobri com Rust (projeto para iniciantes)`
- `Contador de palavras em Rust do zero: HashMap e erros na prática | Curso de Rust`

### Descrição (colar inteira; ajustar os capítulos depois da edição)
```
Qual palavra o Machado de Assis mais repete em Dom Casmurro? Neste projeto para iniciantes na linguagem Rust, você escreve do zero um contador de palavras que lê o livro inteiro em milissegundos — com HashMap, String, Vec e tratamento de erros com Result e o operador ?.

É o primeiro vídeo da série Projetos em Rust do Curso de Rust do Rustando: o que vimos nas aulas, aplicado num programa de verdade.

📌 Neste projeto
• Ler um arquivo .txt com fs::read_to_string
• Separar palavras com split e is_alphanumeric (acentos funcionam)
• Contar com HashMap e entry().or_insert()
• Montar o ranking com sort_by e desempate
• O erro E0382 (valor movido) e como resolver
• Filtrar palavras vazias ("que", "de", "a"...)
• Medir o tempo com Instant: debug vs cargo run --release
• Trocar expect por Result, ok_or, map_err e ?

⏱️ Capítulos
0:00 Dom Casmurro em milissegundos
0:30 O que vamos construir
1:00 Criando o projeto e lendo o arquivo
2:30 Separando as palavras
4:30 Contando com HashMap
6:30 O ranking (e a resposta errada)
8:30 A palavra que o Machado mais usa
10:30 Cronômetro e cargo --release
12:00 Tratamento de erro de verdade
14:00 Desafio

💻 Código do projeto
https://github.com/carlosrobert0/rustando/tree/main/contador-palavras

📚 Antes deste projeto
Curso de Rust #9 — Vec, String e HashMap: https://youtu.be/p-ByiaX3jSU
Curso de Rust #10 — Tratamento de erros: https://youtu.be/IgOrxvzHmGE
Começando do zero? https://youtu.be/iZd-s9THv1U

📺 Playlist completa do Curso de Rust
https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

📖 Dom Casmurro está em domínio público: o .txt está no Projeto Gutenberg (gutenberg.org).

💬 Desafio: rode no seu livro favorito e comente qual palavra ganhou!

#rust #rustlang #programacao
```
Os capítulos seguem o tempo do roteiro. **Conferir depois da edição**: o YouTube só aceita capítulos com o
primeiro em 0:00, em ordem crescente, e cada um com pelo menos 10 s.

### Tags
`curso de rust, projeto rust, rust para iniciantes, linguagem rust, rust hashmap, rustlang, programação rust, dom casmurro`

### Comentário fixado (publicar logo depois de subir o vídeo)
```
Desafio 👇 Rode o contador no seu livro favorito e comente: qual livro e qual palavra ganhou?
O meu: Dom Casmurro → [PREENCHER: palavra] ([PREENCHER: N] vezes).
Código: https://github.com/carlosrobert0/rustando/tree/main/contador-palavras
```

### No Studio, antes de publicar
- [ ] Thumbnail `contador-palavras/thumbnail-projeto-01.png` (com o tempo real)
- [ ] **Legendas**: conferir se as automáticas em português foram geradas (o #10 está sem nenhuma)
- [ ] Tela final (últimos 20 s): próximo vídeo + playlist
- [ ] Card aos ~4:30 ("Lembra da aula #9") → aula #9; card aos ~12:00 → aula #10
- [ ] Adicionar à playlist "Curso de Rust do Zero" **e** a uma playlist nova "Projetos em Rust"
- [ ] Publicar como vídeo normal (não Estreia), no dia fixo da semana
- [ ] Short de 30–45 s com a revelação (passo 6), com link pro vídeo completo, 1–2 dias depois
