# Aula #11 (Parte 4) — Lifetimes
> Referência: The Rust Book, 10.3 (Validating References with Lifetimes)
> Duração alvo: **13–15 min** · Projeto: `cargo new lifetimes`
> Baseado nas seções 12 a 16 de [aula-11-generics-traits-lifetimes.md](aula-11-generics-traits-lifetimes.md).
> Código e erros abaixo **compilados e conferidos** (Rust 1.96).

## Regras desta gravação
- Abrir com o erro na tela, sem cumprimento antes. Nenhum "então é isso" no meio.
- CTA uma vez, depois do passo 6 (a assinatura completa).
- O tema assusta: **ler o `help:` do compilador em voz alta** em todo erro. Ele já dá a resposta.

---

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** o erro E0106, com o `help:` destacado.

```
error[E0106]: missing lifetime specifier
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
help: consider introducing a named lifetime parameter
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
```

> "Esse é o erro que faz muita gente desistir de Rust. Hoje ele vira o seu erro favorito — porque ele tá certo,
> e olha só: a própria mensagem já te dá a resposta pronta.
> Em quinze minutos você vai ler qualquer assinatura com esse apóstrofo `'a` sem travar, e vai entender por que
> quase nunca precisa escrever isso. Bora."

---

## 1. O problema: referência pendurada (0:30 – 2:30) — ERRO PROPOSITAL

> "Lifetime não é sobre quanto tempo o valor vive. É sobre o compilador conseguir **provar** que a referência
> não sobrevive ao dado."

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
}
```

**Erro real:**
```
error[E0597]: `x` does not live long enough
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {r}");
  |                   - borrow later used here
```
**Tela:** desenhar os dois escopos (`'a` por fora, `'b` por dentro). O borrow checker compara os dois e vê o
menor sendo guardado no maior.

---

## 2. Lifetime em funções (2:30 – 5:30) — ERRO PROPOSITAL

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```
**Erro:** o E0106 da abertura. **Ler o `help:` em voz alta:**
> "*this function's return type contains a borrowed value, but the signature does not say whether it is borrowed
> from `x` or `y`* — a função devolve uma referência, mas não diz se ela veio do x ou do y. Isso só se decide
> rodando. Então a gente conta pro compilador."

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
**A frase-chave da aula (mostrar escrita na tela):**
> "A anotação de lifetime **não muda** quanto tempo nada vive. Ela só descreve a relação entre os tempos de vida
> pro compilador poder checar."

Rodar: `O maior é uma string longa`.

---

## 3. Quebrando de propósito (5:30 – 7:30) — 2 ERROS PROPOSITAIS

```rust
fn main() {
    let string1 = String::from("uma string longa");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("O maior é {result}");
}
```
**Erro real:** `error[E0597]: \`string2\` does not live long enough`
> "O `'a` virou o **menor** dos dois tempos de vida. O `string2` morre antes do `println!`, então o resultado não
> pode ser usado ali."

Devolvendo algo criado dentro da função:
```rust
fn longest<'a>(_x: &str, _y: &str) -> &'a str {
    let result = String::from("uma string bem longa");
    result.as_str()
}
```
**Erro real:**
```
error[E0515]: cannot return value referencing local variable `result`
  |     returns a value referencing data owned by the current function
```
> "Nenhum lifetime salva isso: o `result` morre no fim da função. A solução é devolver a `String` inteira, o dono,
> em vez de uma referência."

---

## 4. Lifetime em structs (7:30 – 9:00)

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let texto = String::from("Rust é rápido. Rust é seguro.");
    let primeira_frase = texto.split('.').next().unwrap();
    let trecho = ImportantExcerpt { part: primeira_frase };
}
```
> "Isso diz: um `ImportantExcerpt` não pode viver mais que o texto que ele aponta."

---

## 5. Por que você quase nunca escreve `'a`: elisão (9:00 – 11:30)

> "Se lifetime é obrigatório, por que a gente nunca escreveu isso antes? Porque o compilador aplica três regras
> sozinho."

1. Cada parâmetro que é referência ganha o seu próprio lifetime.
2. Se há **exatamente uma** referência de entrada, o lifetime dela vai pra saída.
3. Se um dos parâmetros é `&self` ou `&mut self`, o lifetime do `self` vai pra saída.

**Ao vivo:**
- `fn first_word(s: &str) -> &str` → regra 2 resolve. (Lembra do contador de palavras: `contar_palavras(texto: &str)
  -> HashMap<&str, usize>` funcionou sem `'a` por causa dessa regra.)
- `fn longest(x: &str, y: &str) -> &str` → nenhuma regra resolve → E0106.

Regra 3 em ação:
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Atenção: {announcement}");
        self.part
    }
}
```

**`'static`:** vive o programa todo. Toda string literal é `&'static str`.
> "Aviso: não sai colocando `'static` pra calar o compilador. Quase sempre o problema real é uma referência
> pendurada."

---

## 6. Juntando tudo (11:30 – 13:00) — a assinatura da Parte 1

**Tela:** a assinatura que foi teaser na Parte 1.

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Atenção! {ann}");
    if x.len() > y.len() { x } else { y }
}
```
> "`'a` é o lifetime da Parte 4. `T` é o genérico das Partes 1 e 2. `Display` é o trait bound da Parte 3. Os três
> moram no mesmo par de sinais porque, pro compilador, são a mesma coisa: informação que ele precisa antes de rodar."

**CTA (único do vídeo):**
> "Se você leu essa assinatura inteira sem travar, deixa o like. A aula #11 inteira foi pra chegar aqui."

---

## 7. Fechamento (13:00 – 13:45)

> "Generics: um código pra muitos tipos, sem custo. Traits: comportamento compartilhado. Lifetimes: a garantia de
> que a referência vale enquanto for usada.
> Na próxima aula, **testes**: o Rust já vem com tudo pra testar, sem instalar nada. E é com testes que a gente vai
> construir o próximo projeto, um grep de verdade. Bora rustando!"

**Tela final (20 s):** #12 (ou #11 Parte 1, se o #12 ainda não saiu) + playlist.

---

## Publicação

**Título:** `Lifetimes em Rust sem medo: o que o 'a realmente diz | Curso de Rust #11 (Parte 4)`

**Thumbnail:** badge `#11` · kicker `CURSO DE RUST · PARTE 4` · título `Lifetimes` · acento `sem medo` ·
chips `fn longest<'a>` e `E0106` (na thumbnail pode usar `<` e `>`: a restrição vale só pra descrição) ·
subtítulo `O erro que faz todo mundo desistir de Rust`

**Primeiras linhas da descrição:**
```text
O erro "missing lifetime specifier" (E0106) faz muita gente desistir de Rust. Nesta parte 4 da aula #11 do Curso de Rust você entende lifetimes de vez: o que o 'a diz, por que quase nunca precisa escrever, e como ler qualquer assinatura.
```
**Capítulos (estimados):**
```text
0:00 O erro que faz todo mundo desistir
0:30 Referência pendurada (E0597)
2:30 Lifetime em funções: longest‹'a›
5:30 Quebrando de propósito (E0597 e E0515)
7:30 Lifetime em structs
9:00 Elisão: as 3 regras
11:30 Juntando tudo: generics + traits + lifetimes
13:00 Próxima aula: testes
```

## Checklist de gravação
- [ ] Erros propositais: E0597 (`r = &x`), E0106 (`longest`), E0597 (`string2`), E0515 (retornar referência local)
- [ ] Frase-chave escrita na tela no passo 2
- [ ] Mostrar a assinatura da Parte 1 no passo 6
- [ ] Nenhum "então é isso" antes do passo 7
