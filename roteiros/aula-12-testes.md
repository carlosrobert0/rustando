# Aula #12 — Testes (2 partes)
> Referência: The Rust Book, capítulo 11 (11.1, 11.2, 11.3)
> Duração alvo: **2 vídeos de 11–14 min** · Projeto: `cargo new adder --lib`
> Código, saídas e falhas abaixo **rodados e conferidos** com `cargo test` (Rust 1.96).

## Regras desta gravação
- Abrir com o resultado na tela, sem cumprimento antes. Nenhum "então é isso" no meio.
- Cada parte termina num gancho pra próxima. A Parte 2 termina no minigrep (Projeto #2).

---

# Parte 1 — Escrevendo testes (e vendo eles falharem)

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** `cargo test` com um teste **FAILED** em vermelho → corrige uma linha → roda de novo → tudo `ok`.

> "Rust já vem com tudo pra testar seu código. Sem instalar nada, sem framework: um atributo em cima da função e o
> `cargo test` faz o resto.
> Hoje você vai escrever testes, **quebrar eles de propósito** e entender exatamente o que o Rust te diz quando
> alguma coisa dá errado. Bora."

## 1. O projeto e a anatomia de um teste (0:30 – 3:00)

```bash
cargo new adder --lib
cd adder
cargo test
```
O `cargo new --lib` já cria um teste. Ler cada linha:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_dois_mais_dois() {
        assert_eq!(add(2, 2), 4);
    }
}
```
- `#[cfg(test)]`: esse módulo só compila quando você roda `cargo test`. Não vai pro binário final.
- `use super::*`: traz tudo do módulo de cima. `super` é o "módulo pai" dos caminhos relativos (capítulo 7 do
  Book). Vale explicar em 1 frase: as aulas #7 e #8 não chegaram a mostrar `super`.
- `#[test]`: marca a função como teste.

## 2. Quebrando de propósito: `assert_eq!` (3:00 – 4:30) — FALHA PROPOSITAL

Trocar o `4` por `5`:
```rust
assert_eq!(add(2, 2), 5);
```
**Saída real:**
```
thread 'tests::soma_dois_mais_dois' panicked at src/lib.rs:44:9:
assertion `left == right` failed
  left: 4
 right: 5
```
> "Teste que falha é só um teste que deu panic. E o `assert_eq!` mostra os dois lados, o que veio e o que você
> esperava, sem você escrever nada."

## 3. `assert!` com o retângulo da aula #5 (4:30 – 6:30)

```rust
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn retangulo_maior_cabe_o_menor() {
    let maior = Rectangle { width: 8, height: 7 };
    let menor = Rectangle { width: 5, height: 1 };
    assert!(maior.can_hold(&menor));
}
```
> "`assert!` recebe um `bool`. Pra testar o contrário, `assert!(!menor.can_hold(&maior))`."

## 4. Mensagem própria quando falha (6:30 – 8:00) — FALHA PROPOSITAL

```rust
pub fn greeting(name: &str) -> String {
    format!("Olá, {name}!")
}

#[test]
fn saudacao_tem_o_nome() {
    let resultado = greeting("Capitu");
    assert!(resultado.contains("Capitu"), "A saudação não tem o nome, veio `{resultado}`");
}
```
Introduzir o bug: `String::from("Olá!")`. **Saída real:**
```
thread 'tests::saudacao_tem_o_nome' panicked at src/lib.rs:57:9:
A saudação não tem o nome, veio `Olá!`
```

## 5. Testando que **deu** panic: `should_panic` (8:00 – 10:30) — FALHA PROPOSITAL

O `Guess` da aula #10:
```rust
pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("O palpite deve estar entre 1 e 100, veio {value}.");
        }
        Guess { value }
    }
}

#[test]
#[should_panic(expected = "entre 1 e 100")]
fn palpite_maior_que_100() {
    Guess::new(200);
}
```
Introduzir o bug: tirar o `|| value > 100`. **Saída real:**
```
---- tests::palpite_maior_que_100 stdout ----
note: test did not panic as expected
```
> "O `expected` confere um pedaço da mensagem. Assim o teste não passa por causa de um panic qualquer."

## 6. Testes que devolvem `Result` (10:30 – 11:30)

```rust
#[test]
fn com_result() -> Result<(), String> {
    if add(2, 2) == 4 {
        Ok(())
    } else {
        Err(String::from("dois mais dois não deu quatro"))
    }
}
```
> "Assim dá pra usar o `?` dentro do teste."

**CTA (único):** depois de rodar tudo verde.

## 7. Fechamento (11:30 – 12:00)
> "Na Parte 2: rodar só um teste, ver o `println!` dos testes, testes ignorados e a diferença entre teste de
> unidade e de integração — que é o que a gente vai usar pra construir um grep do zero."

### Publicação (Parte 1)
**Título:** `Testes em Rust: escreva, quebre e conserte | Curso de Rust #12 (Parte 1)`
**Thumbnail:** badge `#12` · kicker `CURSO DE RUST · PARTE 1` · título `Testes` · acento `sem framework` ·
chips `#[test]` e `assert_eq!` · subtítulo `cargo test e o que fazer quando falha`
**Capítulos (estimados):** `0:00 Testes sem instalar nada` · `0:30 Anatomia de um teste` ·
`3:00 assert_eq! falhando` · `4:30 assert! e o retângulo` · `6:30 Mensagem própria` ·
`8:00 should_panic` · `10:30 Testes com Result` · `11:30 Próxima parte`

---

# Parte 2 — Rodando e organizando testes

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** `cargo test` com 7 testes → `cargo test palpite` roda só 1.

> "Quando o projeto cresce, rodar todos os testes toda vez cansa. Hoje: rodar só o teste que você quer, ver o que
> ele imprime, deixar os lentos de lado — e onde cada tipo de teste mora num projeto Rust de verdade. No fim, a
> gente tem a estrutura pronta pro próximo projeto: um grep do zero. Bora."

## 1. Controlando a execução (0:30 – 5:00)

```bash
cargo test palpite                # só os testes com "palpite" no nome
cargo test -- --test-threads=1    # um de cada vez (testes rodam em paralelo por padrão)
cargo test -- --show-output       # mostra o println! dos testes que passaram
```
> "O primeiro `--` separa o que é pro cargo do que é pro programa de teste."

Testes ignorados:
```rust
#[test]
#[ignore]
fn teste_demorado() {
    assert_eq!(add(1, 1), 2);
}
```
**Saída real:**
```
test tests::teste_demorado ... ignored
test result: ok. 6 passed; 0 failed; 1 ignored; ...
```
`cargo test -- --ignored` roda só os ignorados.

## 2. Testes de unidade e funções privadas (5:00 – 7:00)

```rust
fn soma_interna(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn testa_funcao_privada() {
    assert_eq!(soma_interna(2, 3), 5);
}
```
> "Função privada pode ser testada: o `mod tests` é filho do módulo dela, então enxerga tudo (aula #8,
> privacidade)."

## 3. Testes de integração: a pasta `tests/` (7:00 – 10:00)

```rust
// tests/integracao.rs
use adder::add;

#[test]
fn soma_pela_api_publica() {
    assert_eq!(add(3, 2), 5);
}
```
**Saída real:** uma seção nova, `Running tests/integracao.rs`, com `test soma_pela_api_publica ... ok`.
> "Cada arquivo em `tests/` é um crate separado: ele só enxerga o que é `pub`. É o teste do jeito que quem usa a
> sua biblioteca vê."

## 4. Por que projeto de verdade tem `lib.rs` (10:00 – 11:30)
> "Teste de integração só funciona com biblioteca. Por isso projeto Rust de verdade separa: `main.rs` pequeno, que
> só chama, e `lib.rs` com a lógica, que dá pra testar. É exatamente assim que a gente vai montar o minigrep."

**CTA (único):** aqui.

## 5. Fechamento (11:30 – 12:00)
> "No próximo vídeo: Projeto #2. Um grep do zero, o projeto do capítulo 12 do livro oficial, usando tudo da aula
> #11 e os testes de hoje."

### Publicação (Parte 2)
**Título:** `Rodando e organizando testes em Rust: unidade e integração | Curso de Rust #12 (Parte 2)`
**Thumbnail:** badge `#12` · kicker `CURSO DE RUST · PARTE 2` · título `Testes` · acento `organizados` ·
chips `cargo test nome` e `tests/` · subtítulo `Unidade, integração e testes ignorados`
**Capítulos (estimados):** `0:00 Rodando só o teste que você quer` · `0:30 Filtros, threads e --show-output` ·
`5:00 Testando funções privadas` · `7:00 Testes de integração: a pasta tests/` · `10:00 Por que lib.rs` ·
`11:30 Próximo: Projeto minigrep`

## Checklist de gravação
- [ ] Parte 1: as 3 falhas propositais (`assert_eq!` 4≠5, saudação sem nome, `should_panic` que não deu panic)
- [ ] Parte 2: `cargo test palpite`, `--show-output`, `#[ignore]` e `--ignored`, pasta `tests/`
- [ ] Usar exemplos das aulas anteriores (retângulo da #5, `Guess` da #10) e dizer de qual aula vieram
