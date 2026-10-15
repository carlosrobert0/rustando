# Aula #11 — Generics, Traits e Lifetimes
> Referência: The Rust Book, capítulo 10 (10-00, 10-01, 10-02, 10-03)
> Duração estimada: 40–50 min · Projeto: `cargo new generics`

---

## 0. Abertura (0:00 – 1:30)

**Fala:**
> "Toda linguagem tem um jeito de lidar com duplicação de código. Em Rust esse jeito
> tem três nomes: generics, traits e lifetimes. Generics é 'esse código funciona pra
> qualquer tipo'. Trait é 'esse tipo sabe fazer tal coisa'. E lifetime é 'essa
> referência vive tempo suficiente'. Os três juntos são o que faz o Rust ser rápido
> igual C e seguro ao mesmo tempo. Bora."

**Mostrar na tela:** os três em uma assinatura só (guardar como teaser, explicar no final):

```rust
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{ /* ... */ }
```

> "No fim da aula você vai ler essa assinatura e ela vai fazer sentido."

---

## 1. O problema: duplicação (1:30 – 5:00)

**Passo a passo (digitar junto):** função que acha o maior número de uma lista.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("O maior número é {largest}");
}
```

Agora precisamos do mesmo para `char`. **Copiar e colar** — deixar a duplicação
na tela por alguns segundos:

```rust
fn largest_i32(list: &[i32]) -> &i32 { /* ...igual... */ }
fn largest_char(list: &[char]) -> &char { /* ...igual... */ }
```

**Fala:**
> "Repara: o corpo das duas é idêntico. A única coisa que muda é o tipo. É exatamente
> isso que generics resolve."

---

## 2. Generics em funções (5:00 – 9:00)

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {   // ERRO PROPOSITAL
            largest = item;
        }
    }
    largest
}
```

**Compilar e mostrar o erro:**
```
error[E0369]: binary operation `>` cannot be applied to type `&T`
help: consider restricting type parameter `T`: `std::cmp::PartialOrd`
```

**Fala:**
> "O compilador tá certo: `T` pode ser qualquer coisa, e nem toda coisa é comparável.
> Se `T` fosse um `Vec`, o que significaria `maior`? A gente precisa **restringir**
> o T — dizer que ele tem que saber se comparar. Isso é um trait bound."

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { /* ... */ }
```

**Ponto de ensino:** `<T>` se lê "genérico sobre o tipo T". Convenção é `T`, mas pode
ser qualquer nome. Declara-se depois do nome da função, antes dos parênteses.

---

## 3. Generics em structs (9:00 – 12:00)

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 5, y: 4.0 }; // ERRO PROPOSITAL
}
```

**Mostrar o erro** (`expected integer, found floating-point number`) e explicar:
um único `T` significa que os dois campos são do **mesmo** tipo.

Solução com dois parâmetros:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

---

## 4. Generics em enums (12:00 – 14:00)

**Fala:**
> "Você já usou generics em todas as aulas anteriores e nem percebeu."

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Amarrar com a aula #10: `Result<T, E>` é genérico sobre o valor de sucesso **e**
sobre o erro. Por isso `File::open` devolve `Result<File, std::io::Error>`.

---

## 5. Generics em métodos (14:00 – 18:00)

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

**Ponto que confunde todo mundo:** o `impl<T>` declara o genérico; o `Point<T>` usa.
Sem o primeiro `<T>`, o Rust acha que `T` é um tipo concreto chamado T.

Implementação só para um tipo concreto:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

**Fala:** "Esse método só existe pra `Point<f32>`. Um `Point<i32>` não tem esse método
— o compilador nem deixa chamar."

Método com genéricos próprios (diferentes dos da struct):

```rust
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}
```

---

## 6. Custo de performance: monomorfização (18:00 – 20:00)

**Fala:**
> "A pergunta óbvia: isso é mais lento? Não. Zero. Em tempo de compilação o Rust faz
> monomorfização — ele olha todos os tipos concretos que você usou e gera uma versão
> especializada de cada um."

Mostrar o "antes e depois" conceitual:

```rust
let integer = Some(5);
let float = Some(5.0);
```
vira, na prática:
```rust
enum Option_i32 { Some(i32), None }
enum Option_f64 { Some(f64), None }
```

> "Você escreve genérico, o binário sai específico. É o mesmo desempenho de ter
> escrito as duas funções na mão."

---

## 7. Traits: definindo comportamento (20:00 – 24:00)

**Fala:**
> "Trait é um contrato: um conjunto de métodos que um tipo precisa ter. Se você vem
> de outra linguagem, é parecido com interface — mas com superpoderes."

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, por {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

**Regra do órfão (coherence):** só dá pra implementar um trait para um tipo se pelo
menos um dos dois for seu. Não dá pra implementar `Display` para `Vec<T>` — os dois
são da std.

> "Sem essa regra, duas crates poderiam implementar o mesmo trait pro mesmo tipo de
> jeitos diferentes e o Rust não saberia qual usar."

---

## 8. Implementação padrão (24:00 – 26:00)

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Leia mais de {}...)", self.summarize_author())
    }
}
```

```rust
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize vem de graça
}
```

**Ponto forte:** o método padrão pode chamar métodos que **não** têm implementação
padrão. É o jeito de escrever muito comportamento a partir de pouquíssimo código
obrigatório — exatamente o que a `Iterator` faz.

---

## 9. Traits como parâmetro (26:00 – 30:00)

```rust
pub fn notify(item: &impl Summary) {
    println!("Novidade! {}", item.summarize());
}
```

A forma completa (é a mesma coisa, `impl Trait` é açúcar sintático):

```rust
pub fn notify<T: Summary>(item: &T) { /* ... */ }
```

**Quando a diferença importa** — dois parâmetros:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) { }   // podem ser tipos diferentes
pub fn notify<T: Summary>(item1: &T, item2: &T) { }             // TÊM que ser o mesmo tipo
```

Múltiplos bounds e `where`:

```rust
pub fn notify(item: &(impl Summary + Display)) { }

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ }
```

**Fala:** "Quando a assinatura começa a ficar ilegível, joga tudo no `where`."

---

## 10. Retornando `impl Trait` (30:00 – 32:00)

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost { /* ... */ }
}
```

**Limitação importante (mostrar o erro):** não dá pra retornar `impl Trait` quando
podem sair dois tipos diferentes em ramos diferentes do `if`. Isso não compila —
e a solução (`Box<dyn Trait>`) fica pra aula de smart pointers.

---

## 11. Trait bounds pra habilitar métodos (32:00 – 35:00)

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

**Fala:** "`new` existe pra qualquer `Pair`. `cmp_display` só existe se o T souber
se imprimir e se comparar. O método aparece ou some conforme o tipo."

**Blanket implementation** (mostrar rápido, é o pulo do gato):

```rust
impl<T: Display> ToString for T {
    // ...
}
```

> "É por isso que `3.to_string()` funciona: a std implementou `ToString` para
> **todo** tipo que implementa `Display`, de uma vez só."

---

## 12. Lifetimes: o problema (35:00 – 38:00)

**Fala:**
> "Última peça. Lifetime não é sobre quanto tempo o valor vive — é sobre o compilador
> conseguir **provar** que a referência não sobrevive ao dado."

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;   // ERRO PROPOSITAL
    }
    println!("r: {r}");
}
```

**Mostrar o erro** `x does not live long enough` e desenhar na tela os dois escopos
(`'a` externo, `'b` interno). O borrow checker compara os dois tempos de vida e vê
que o menor está sendo guardado no maior.

---

## 13. Lifetime em funções (38:00 – 42:00)

```rust
fn longest(x: &str, y: &str) -> &str {   // ERRO PROPOSITAL
    if x.len() > y.len() { x } else { y }
}
```

Erro: `missing lifetime specifier ... expected named lifetime parameter`.

**Fala:**
> "O compilador não sabe se o retorno veio do x ou do y — isso só é decidido em tempo
> de execução. Então ele não consegue saber por quanto tempo a referência devolvida é
> válida. A gente precisa **contar** pra ele."

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**A frase-chave da aula:**
> "A anotação de lifetime **não muda** quanto tempo nada vive. Ela só descreve a
> relação entre os tempos de vida para o compilador poder checar."

Demonstrar que funciona e depois quebrar de propósito:

```rust
fn main() {
    let string1 = String::from("uma string longa");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("O maior é {result}");   // ERRO: string2 já morreu
}
```

**Mostrar também** que retornar uma referência para algo criado dentro da função não
compila — e que a solução nesse caso é devolver `String` (dono), não referência.

---

## 14. Lifetime em structs (42:00 – 44:00)

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Chame-me Ismael. Alguns anos atrás...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };
}
```

**Fala:** "Isso diz: uma instância de `ImportantExcerpt` não pode viver mais que a
referência que ela guarda."

---

## 15. Elisão de lifetime (44:00 – 47:00)

**Fala:**
> "Se lifetime é obrigatório, por que a gente nunca escreveu isso antes? Porque o
> compilador infere na maioria dos casos, usando três regras."

1. Cada parâmetro que é referência ganha o seu próprio lifetime.
2. Se há **exatamente um** parâmetro de entrada, o lifetime dele vai pra todas as saídas.
3. Se um dos parâmetros é `&self` ou `&mut self`, o lifetime do `self` vai pras saídas.

Aplicar as regras ao vivo em `fn first_word(s: &str) -> &str` (regra 2 resolve) e
em `fn longest(x: &str, y: &str) -> &str` (nenhuma regra resolve → erro).

Lifetime em método (regra 3 em ação):

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Atenção: {announcement}");
        self.part
    }
}
```

Mencionar `'static` — vive por toda a duração do programa; toda string literal é
`&'static str`. **Aviso:** não saia colocando `'static` pra calar o compilador; quase
sempre o problema real é uma referência pendurada.

---

## 16. Juntando tudo (47:00 – 49:00)

Voltar ao teaser da abertura e destrinchar linha por linha:

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Atenção! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

> "`'a` é o lifetime, `T` é o genérico, `Display` é o trait bound. Os três moram no
> mesmo par de sinais de menor e maior porque, pro compilador, são a mesma coisa:
> informação que ele precisa em tempo de compilação."

---

## 17. Recap e gancho (49:00 – 50:00)

- **Generics** — um código, muitos tipos, sem custo em runtime (monomorfização).
- **Traits** — comportamento compartilhado + restrição do que o genérico aceita.
- **Lifetimes** — garantia de que a referência é válida enquanto for usada.

**Gancho:** "Na próxima aula: testes. Como o Rust traz test runner embutido, `assert!`,
`#[test]`, e como testar exatamente o código que a gente escreveu hoje."

---

## Checklist de gravação

- [ ] Erros propositais: `>` sem `PartialOrd`, `Point { x: 5, y: 4.0 }`, `longest` sem lifetime, `r = &x` fora de escopo, `impl Trait` com dois tipos de retorno
- [ ] Rodar `cargo run` a cada bloco — mostrar o erro **antes** da solução
- [ ] Sempre ler a mensagem do compilador em voz alta (é o melhor professor da linguagem)
- [ ] Deixar o teaser da abertura visível no final para fechar o ciclo
