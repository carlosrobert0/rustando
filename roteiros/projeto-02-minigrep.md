# Projeto Rust #2 — minigrep (2 partes)
> Referência: The Rust Book, capítulo 12 (An I/O Project: Building a Command Line Program)
> Pré-requisitos: aulas #10 (erros), #11 (lifetimes) e #12 (testes)
> Duração alvo: **2 vídeos de 13–15 min** · Projeto: `cargo new minigrep`
> Código final **rodado e conferido** (Rust 1.96): 2 testes passando, busca com e sem `IGNORE_CASE`, erros no stderr.
> O livro usa um poema em inglês. Aqui é a **"Canção do Exílio"** (Gonçalves Dias, 1843, domínio público).

**Por que este projeto:** é o projeto do próprio Book, e o curso segue o Book. Ele junta tudo das últimas aulas:
o lifetime da assinatura de `search` (#11 Parte 4), o TDD (#12), `Result` + `?` + `Box<dyn Error>` (#10) e a
separação `main.rs` / `lib.rs` (#12 Parte 2).

## Regras desta gravação
- Resultado na tela nos primeiros 5 s. Nenhum "então é isso" no meio.
- Cada parte termina com o programa rodando e um gancho.

## Arquivo `poema.txt` (criar na raiz do projeto)
```text
Minha terra tem palmeiras,
Onde canta o Sabiá;
As aves, que aqui gorjeiam,
Não gorjeiam como lá.

Nosso céu tem mais estrelas,
Nossas várzeas têm mais flores,
Nossos bosques têm mais vida,
Nossa vida mais amores.
```

---

# Parte 1 — Lendo argumentos e arquivo, e organizando o código

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** `cargo run -- terra poema.txt` → `Minha terra tem palmeiras,`

> "Isso aqui é um grep: o comando que procura texto dentro de arquivo, que todo dev usa todo dia. Hoje a gente
> escreve o nosso, do zero, em Rust. É o projeto do capítulo 12 do livro oficial — o que junta tudo que a gente
> viu até aqui.
> Nesta primeira parte: ler os argumentos, ler o arquivo e organizar o código do jeito que projeto de verdade é
> organizado. Bora."

## 1. Lendo os argumentos (0:30 – 3:00)
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Buscando {query} em {file_path}");
}
```
**ERRO PROPOSITAL:** `cargo run` sem argumentos → `index out of bounds: the len is 1 but the index is 1` (panic).
> "Funciona, mas qualquer um derruba. Guarda esse erro, a gente conserta no passo 4."

## 2. Lendo o arquivo (3:00 – 4:30)
```rust
let contents = fs::read_to_string(file_path)
    .expect("Deveria conseguir ler o arquivo");
println!("Conteúdo:\n{contents}");
```

## 3. Refatorando: `Config` (4:30 – 8:00)
> "O `main` está fazendo tudo. O livro chama isso de separação de responsabilidades: o `main` só junta as peças."

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
}
```
Por que `clone()` aqui: o `Config` precisa ser **dono** das Strings (aula #4 Parte 2). O custo é mínimo, e
evita lifetimes na struct (aula #11 Parte 4).

## 4. Tratando o erro de verdade: `Config::build` (8:00 – 10:30)
```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("faltam argumentos: minigrep <busca> <arquivo>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
```
No `main`:
```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Erro nos argumentos: {err}");
    process::exit(1);
});
```
**Saída real sem argumentos:** `Erro nos argumentos: faltam argumentos: minigrep <busca> <arquivo>` (exit 1).
> "O `&'static str` é o `'static` da aula de lifetimes: a mensagem é um texto fixo do programa."

## 5. `run` e o `Box<dyn Error>` (10:30 – 12:30)
```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // ...
    Ok(())
}
```
> "`Box<dyn Error>`: qualquer tipo de erro. É a saída que o compilador sugeriu na aula de traits, quando o
> `impl Trait` não deu conta."

**Saída real com arquivo que não existe:** `Erro na aplicação: No such file or directory (os error 2)`

## 6. Mudando pra `lib.rs` (12:30 – 14:00)
Mover `Config`, `build` e `run` pra `src/lib.rs`. No `main.rs`: `use minigrep::Config;` e `minigrep::run(config)`.
> "Agora a lógica está numa biblioteca, e biblioteca dá pra testar (aula #12). Na Parte 2 a gente escreve a
> busca **começando pelo teste**."

**CTA (único):** antes do gancho.

### Publicação (Parte 1)
**Título:** `Projeto em Rust: seu próprio grep do zero (Parte 1) | Curso de Rust`
**Thumbnail:** badge `PROJETO 2` · kicker `RUST PARA INICIANTES` · título `Seu grep` · acento `do zero` ·
chips `Config::build` e `Box<dyn Error>` · subtítulo `O projeto do capítulo 12 do Rust Book`
**Capítulos (estimados):** `0:00 Um grep em Rust` · `0:30 Lendo os argumentos` · `3:00 Lendo o arquivo` ·
`4:30 Refatorando: Config` · `8:00 Tratando erros: Config::build` · `10:30 run e Box‹dyn Error›` ·
`12:30 Separando em lib.rs`

---

# Parte 2 — Busca com TDD, maiúsculas e stderr

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** `cargo run -- minha poema.txt` → nada. Depois `IGNORE_CASE=1 cargo run -- minha poema.txt` →
`Minha terra tem palmeiras,`

> "Esse grep não acha 'minha' porque no poema está com M maiúsculo. Até o fim do vídeo ele vai achar — e o código
> da busca vai nascer de um teste que falha primeiro. É assim que muito dev experiente trabalha, e em Rust fica
> muito natural. Bora."

## 1. TDD: o teste primeiro (0:30 – 3:30) — FALHA PROPOSITAL
```rust
#[cfg(test)]
mod tests {
    use super::*;

    const POEMA: &str = "\
Nosso céu tem mais estrelas,
Nossas várzeas têm mais flores,
Nossos bosques têm mais vida,
Nossa vida mais amores.";

    #[test]
    fn diferencia_maiusculas() {
        assert_eq!(
            vec!["Nossos bosques têm mais vida,", "Nossa vida mais amores."],
            search("vida", POEMA)
        );
    }
}
```
Implementação mínima, só pra compilar:
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```
**Saída real:**
```
assertion `left == right` failed
  left: ["Nossos bosques têm mais vida,", "Nossa vida mais amores."]
 right: []
```

## 2. O `'a` da assinatura (3:30 – 5:00)
> "Lembra da Parte 4 da aula #11? As linhas devolvidas são fatias do `contents`, não da `query`. Por isso o `'a`
> liga a saída ao `contents`. Sem ele, o compilador não sabe de qual dos dois as referências vêm."

## 3. Fazendo o teste passar (5:00 – 7:00)
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```
`cargo test` → `test tests::diferencia_maiusculas ... ok`. Ligar no `run` e rodar `cargo run -- terra poema.txt`.

## 4. Ignorando maiúsculas: teste primeiro de novo (7:00 – 10:00)
```rust
#[test]
fn ignora_maiusculas() {
    assert_eq!(
        vec!["Nossas várzeas têm mais flores,", "Nossa vida mais amores."],
        search_case_insensitive("nOsSa", POEMA)
    );
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```
> "Repara: 'nOsSa' também acha 'Nossas'. É um 'contém', não uma palavra inteira. Fica de desafio nos comentários."

## 5. Variável de ambiente `IGNORE_CASE` (10:00 – 12:00)
```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
// em build:
let ignore_case = env::var("IGNORE_CASE").is_ok();
// em run:
let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
} else {
    search(&config.query, &contents)
};
```
**Saída real:** `IGNORE_CASE=1 cargo run -- minha poema.txt` → `Minha terra tem palmeiras,`

**REVELAÇÃO / CTA (único):** aqui, com o "minha" da abertura sendo encontrado.

## 6. Erros no lugar certo: `eprintln!` (12:00 – 13:30)
```bash
cargo run > saida.txt
```
Com `println!`, a mensagem de erro iria parar dentro do `saida.txt`. Com `eprintln!` ela vai pro terminal e o
**`saida.txt` fica vazio** (conferido: 0 bytes).
> "Saída normal no stdout, erro no stderr. É isso que deixa o seu programa se comportar como os comandos de verdade
> do terminal."

## 7. Fechamento (13:30 – 14:15)
> "Um grep de verdade, com testes, tratamento de erro e lifetimes, e tudo veio de aulas do curso.
> Desafio: fazer a busca por palavra inteira, em vez de 'contém'. Comenta como você fez.
> Na próxima aula: closures e iterators. E a gente volta neste código pra deixar a busca em uma linha."

### Publicação (Parte 2)
**Título:** `Projeto em Rust: minigrep com TDD, maiúsculas e stderr (Parte 2) | Curso de Rust`
**Thumbnail:** badge `PROJETO 2` · kicker `PARTE 2` · título `Seu grep` · acento `com testes` ·
chips `IGNORE_CASE=1` e `eprintln!` · subtítulo `TDD de verdade, em Rust`
**Capítulos (estimados):** `0:00 Por que não acha "minha"?` · `0:30 TDD: o teste primeiro` ·
`3:30 O 'a da assinatura` · `5:00 Fazendo o teste passar` · `7:00 Ignorando maiúsculas` ·
`10:00 Variável de ambiente IGNORE_CASE` · `12:00 eprintln! e stderr` · `13:30 Desafio`

## Checklist de gravação
- [ ] `poema.txt` criado antes de gravar
- [ ] Parte 1: panic do índice sem argumentos (passo 1) → resolvido no passo 4
- [ ] Parte 2: teste falhando com `right: []` antes de implementar
- [ ] Mostrar `saida.txt` vazio no passo 6
- [ ] Nas descrições: `Box‹dyn Error›` com `‹ ›` (o YouTube não aceita `<` e `>`)
