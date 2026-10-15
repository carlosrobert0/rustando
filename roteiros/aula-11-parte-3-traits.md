# Aula #11 (Parte 3) — Traits
> Referência: The Rust Book, 10.2 (Traits) + o fim do 10.1 (monomorfização)
> Duração alvo: **13–15 min** · Projeto: continua no `generics/` (ou `cargo new traits`)
> Baseado nas seções 6 a 11 de [aula-11-generics-traits-lifetimes.md](aula-11-generics-traits-lifetimes.md),
> reorganizadas com abertura de 30 s e um gancho que só se resolve no fim.
> Código e erros abaixo **compilados e conferidos** (Rust 1.96).

## Regras desta gravação
- Nada de "fala galera" antes do gancho. O nome do canal só no fim.
- **Não dizer "então é isso"** no meio do vídeo. No #4 Parte 1, essa frase derrubou a retenção de 52% pra 42%.
  Pra trocar de assunto: "agora vem a parte que mais confunde…", "e aqui tem uma pegadinha…".
- Pedir like **uma vez só**, logo depois da revelação do passo 8.

---

## 0. Abertura (0:00 – 0:30) — falar exatamente assim

**Tela (0:00):** terminal com `println!("{}", 3.to_string());` rodando e imprimindo `3`.

> "Você já chamou `.to_string()` num número. Mas vai procurar na documentação do `i32`: ninguém escreveu um
> `to_string` pra ele. Então de onde esse método vem?
> Ele vem de **uma linha só** da biblioteca padrão — e no fim desse vídeo você vai ler essa linha e entender.
> No caminho: o que é um trait, como dar comportamento de graça pros seus tipos, e por que às vezes um método
> simplesmente **some**. Bora."

**Tela (0:28):** corte seco pro editor.

---

## 1. Em 30 segundos: onde a gente parou + monomorfização (0:30 – 2:00)

> "Nas partes 1 e 2: funções, structs, enums e métodos genéricos. Antes de traits, a pergunta que todo mundo faz:
> genérico deixa o código mais lento?"

```rust
let integer = Some(5);
let float = Some(5.0);
```
vira, na prática:
```rust
enum Option_i32 { Some(i32), None }
enum Option_f64 { Some(f64), None }
```
> "Não. O compilador gera uma versão pra cada tipo que você usou. Isso se chama monomorfização: você escreve
> genérico e o binário sai específico."

---

## 2. Definindo um trait (2:00 – 4:30)

> "Trait é um contrato: um conjunto de métodos que um tipo promete ter. Se você vem de outra linguagem, lembra
> interface."

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, por {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

**Ponto de ensino:** `impl Summary for NewsArticle` se lê "NewsArticle assina o contrato Summary".

---

## 3. A regra do órfão (4:30 – 5:30) — ERRO PROPOSITAL

> "Posso implementar qualquer trait pra qualquer tipo? Vamos testar com dois que não são meus."

```rust
use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} itens", self.len())
    }
}
```

**Erro real:**
```
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
3 | impl fmt::Display for Vec<i32> {
  |                       -------- `Vec` is not defined in the current crate
```
> "`Display` é da biblioteca padrão. `Vec` também. Pelo menos um dos dois tem que ser seu. Sem essa regra, duas
> bibliotecas poderiam implementar a mesma coisa de dois jeitos e o Rust não saberia qual usar."

---

## 4. Comportamento de graça: implementação padrão (5:30 – 7:30)

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Leia mais de {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize vem de graça
}
```
**Saída real:** `Novidade! (Leia mais de @rustando...)`

> "O método padrão pode chamar um método que **não** tem padrão. Você implementa um e ganha o outro. A trait
> `Iterator` faz isso em escala: você escreve só o `next` e ganha dezenas de métodos (`map`, `filter`, `sum`...)."

---

## 5. Traits como parâmetro (7:30 – 9:30)

```rust
pub fn notify(item: &impl Summary) {
    println!("Novidade! {}", item.summarize());
}
```
É açúcar pra:
```rust
pub fn notify<T: Summary>(item: &T) { /* ... */ }
```

**Quando a diferença importa:**
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // podem ser tipos diferentes
pub fn notify<T: Summary>(item1: &T, item2: &T) {}           // TÊM que ser o mesmo tipo
```

**Vários bounds e `where`:**
```rust
pub fn notify(item: &(impl Summary + Display)) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ /* ... */ }
```
> "Quando a assinatura começar a ficar ilegível, joga tudo no `where`."

---

## 6. Retornando `impl Trait` (9:30 – 10:30) — ERRO PROPOSITAL

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { /* ... */ }
    } else {
        SocialPost { /* ... */ }
    }
}
```

**Erro real:**
```
error[E0308]: `if` and `else` have incompatible types
   |         expected `NewsArticle`, found `SocialPost`
help: you could change the return type to be a boxed trait object
   |  fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
```
> "`impl Trait` no retorno quer dizer **um** tipo concreto, só que escondido. Dois tipos diferentes não dá.
> O próprio compilador já sugere a saída: `Box<dyn Summary>`. Isso fica pra aula de smart pointers."

---

## 7. O método que some (10:30 – 12:00) — ERRO PROPOSITAL

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("O maior é x = {}", self.x);
        } else {
            println!("O maior é y = {}", self.y);
        }
    }
}
```
`Pair::new(3, 7).cmp_display()` → **`O maior é y = 7`**

Agora com um tipo que não sabe se imprimir nem se comparar:
```rust
struct Ponto;
Pair::new(Ponto, Ponto).cmp_display();
```
**Erro real:**
```
error[E0599]: the method `cmp_display` exists for struct `Pair<Ponto>`, but its trait bounds were not satisfied
note: the following trait bounds were not satisfied:
      `Ponto: PartialOrd`
      `Ponto: std::fmt::Display`
```
> "O método existe, mas só pra `Pair` de tipos que sabem se comparar e se imprimir. Pra `Pair<Ponto>`, ele
> simplesmente não está lá."

---

## 8. A linha do `to_string` — a revelação (12:00 – 13:00)

**Tela:** a documentação da std, `impl ToString for T`.

```rust
impl<T: Display> ToString for T {
    // ...
}
```
> "Essa é a linha. A biblioteca padrão implementou `ToString` pra **todo** tipo que implementa `Display`, de uma vez
> só. Isso se chama implementação cobertora, ou blanket implementation. O `i32` implementa `Display`, então ganhou
> o `to_string()` de graça: o mesmo truque do passo 4, só que pra todos os tipos ao mesmo tempo."
>
> (Na std ainda tem um `?Sized` no bound. É um detalhe que não muda nada aqui.)

**CTA (único do vídeo):**
> "Se essa linha fez sentido agora, deixa o like: é o que faz esse curso chegar em mais gente."

---

## 9. Fechamento (13:00 – 13:45)

> "Recapitulando: trait é contrato, pode trazer comportamento padrão, restringe o que um genérico aceita, e pode
> fazer um método aparecer ou sumir.
> Na Parte 4, a última peça: **lifetimes**. E aí aquela assinatura do começo da Parte 1, com `'a`, `T` e `Display`
> juntos, finalmente vai ser lida inteira. Até lá, bora rustando!"

**Tela final (20 s):** #11 Parte 4 + playlist.

---

## Publicação

**Título:** `Traits em Rust: o contrato que todo tipo pode assinar | Curso de Rust #11 (Parte 3)`

**Thumbnail (thumbgen, mesmo layout do #11):** badge `#11` · kicker `CURSO DE RUST · PARTE 3` · título `Traits` ·
acento `de graça` · chips `impl Summary for` e `T: Display` · subtítulo `Por que 3.to_string() funciona?`

**Primeiras linhas da descrição:**
```text
Por que 3.to_string() funciona se ninguém escreveu to_string pro i32? Nesta parte 3 da aula #11 do Curso de Rust você entende traits: o contrato que dá comportamento aos seus tipos — e a linha da biblioteca padrão que explica o to_string.
```
**Capítulos (estimados pelo roteiro; conferir depois da edição):**
```text
0:00 De onde vem o to_string()?
0:30 Monomorfização: genérico não é mais lento
2:00 Definindo um trait
4:30 A regra do órfão (E0117)
5:30 Implementação padrão
7:30 Traits como parâmetro e where
9:30 Retornando impl Trait (E0308)
10:30 O método que some (E0599)
12:00 A linha do to_string: blanket implementation
13:00 Próxima parte: lifetimes
```
Nos capítulos, escreva `‹T›` em vez de `<T>`.

## Checklist de gravação
- [ ] Erros propositais: E0117 (órfão), E0308 (`impl Trait` com dois tipos), E0599 (`Pair<Ponto>`)
- [ ] Ler a mensagem do compilador em voz alta, inclusive o `help:` do E0308
- [ ] Deixar o `3.to_string()` da abertura visível no passo 8, pra fechar o ciclo
- [ ] Nenhum "então é isso" antes do passo 9
