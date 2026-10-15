# Melhorias nos vídeos publicados — o conteúdo em si

> Feito em 15/10/2026 a partir das **legendas automáticas de 15 vídeos** (o que é dito em cada segundo) e das
> **curvas de retenção** (onde a audiência sai). Tudo aqui se faz no YouTube Studio, **sem reenviar o vídeo e sem
> perder views, link nem comentários**. A exceção são o #9 e o #10, no fim do arquivo.
>
> Títulos e descrições estão em [videos-existentes.md](videos-existentes.md). Este arquivo é sobre o vídeo em si.

## O que se repete nos vídeos

| Padrão | Onde | O dado |
|---|---|---|
| **Cumprimento antes do conteúdo** ("Fala Dev, beleza? Seja muito bem-vindo…") | todos | Começando com Rust: 54% aos 0:18 e 40% aos 0:36 · #2: perde 42% nos primeiros 10 s · #4 Parte 2: perde 49% até 0:34 |
| **"Então é isso" no meio do vídeo** (soa como fim) | #4 p1, #8, #9, Começando, #2 | #4 p1: depois do "Então é isso, galera" em 3:16, a retenção cai de 52% pra 42% |
| **Legenda automática erra "Rust"** ("Hust", "Rush") | todos | ~110 de ~150 vezes. Também: "struc", "trade" (trait), "HIP" (heap), "cargo RAM" (run), "Hustálise" (rust-analyzer), "[roncando]" |
| **Sem capítulos ou com capítulos errados** | 11 sem · 3 errados | #7: o YouTube não reconhece (estão num parágrafo só) · macOS e #11 p2: tempos não batem com o vídeo |
| **Tempo morto** | macOS | 1:51 esperando o VS Code abrir e conversando enquanto instala |

## Como fazer (no Studio), nesta ordem

1. **Cortar** — Conteúdo → vídeo → **Editor** → **Cortar e aparar** → **Novo corte** → digitar início e fim → Salvar.
   Pode ter vários cortes no mesmo vídeo. O vídeo continua no ar enquanto processa.
2. **Esperar o processamento** terminar (aparece no próprio Editor).
3. **Legenda corrigida** — Legendas → vídeo → Português → **Adicionar** →
   - vídeo **que foi cortado**: **Sincronização automática** → colar o `.txt` da pasta [`legendas/`](legendas/)
     (o YouTube encaixa os tempos sozinho; o `.srt` ficaria fora de sincronia depois do corte);
   - vídeo **sem corte**: **Enviar arquivo** → "Com tempo" → o `.srt`.
4. **Capítulos** — colar o bloco na descrição. Use a versão **com corte** se cortou.
5. **Tela final e cards** — Editor → Tela final / Cards.

Os capítulos foram calculados pelos tempos da fala. Confira 2 ou 3 no player: podem estar 2–5 s adiantados.
**Não use `<` e `>` nos títulos dos capítulos**, porque o YouTube não aceita na descrição.

## Resumo

| Vídeo | Cortar | Legenda | Capítulos | Outro |
|---|---|---|---|---|
| [ ] Começando com Rust | 0:00–0:09 e 0:20–0:31 | `00-comecando-com-rust` | novos | comentário fixado → #3 |
| [ ] #1 Por que aprender Rust | — | `01-por-que-aprender-rust` | — (1:54) | tela final → #2 |
| [ ] #2 Instalando no Windows | 0:00–0:08 e 0:17–0:20 | `02-instalando-rust-windows` | novos | card → vídeo do macOS |
| [ ] #3 Jogo de adivinhação | 0:00–0:08 | `03-jogo-de-adivinhacao` | novos | — |
| [ ] #4 Parte 1 Variáveis | 3:02–3:27 | `04-parte1-…` | novos | — |
| [ ] #4 Parte 2 Ownership | — | `04-parte2-ownership` | novos | — |
| [ ] #5 Parte 1 Structs | 0:00–0:20 | `05-parte1-structs` | novos | — |
| [ ] #5 Parte 2 Structs | — | `05-parte2-structs-metodos` | novos | — |
| [ ] #6 Enums | — | `06-enums-pattern-matching` | novos | — |
| [ ] #7 Módulos na prática | — | `07-modulos-na-pratica` | **trocar** | descrição promete `super`/`pub use` |
| [ ] #8 Módulos e crates | — | `08-modulos-crates-privacidade` | novos | — |
| [ ] #9 Coleções | (republicar em partes) | `09-vec-string-hashmap` | ✅ aplicados | ver fim do arquivo |
| [ ] #10 Erros | (republicar em partes) | **sem legenda** | — | ver fim do arquivo |
| [ ] #11 Parte 1 | — | `11-parte1-…` | **trocar** | — |
| [ ] #11 Parte 2 | — | `11-parte2-…` | **trocar** | — |
| [ ] macOS | 3:29–3:55 e 5:31–6:56 | `extra-instalando-rust-macos` | **trocar** | — |

---

## Começando com Rust — `iZd-s9THv1U` (4.486 views, 1/3 dos inscritos)

**O que acontece:** 0:00–0:04 sem fala. 0:04 "Fala pessoal, beleza? Seja muito bem-vindo ao canal Rustando." 0:09 a
promessa ("instalação no Windows, Hello World e Cargo"), que é boa. 0:20 "Beleza? Toda documentação… vai tá aqui
no comentário. E é isso daí, pessoal." A instalação só começa em 0:31. Retenção: 75% aos 0:07, 54% aos 0:18,
**40% aos 0:36**.

- [ ] **Cortar 0:00–0:09** (silêncio + cumprimento) e **0:20–0:31** ("Beleza? toda documentação… é isso daí,
      pessoal"). O vídeo passa a abrir com "Hoje vamos começar nossa jornada no mundo do Rust…" e vai direto
      pra instalação. São 20 s a menos.
- [ ] Legenda `legendas/00-comecando-com-rust.txt` (sincronização automática, porque cortou).
- [ ] Capítulos (com corte):
```text
0:00 O que vamos fazer: instalar, Hello World e Cargo
0:11 Instalando o Rust no Windows (rustup)
0:53 Testando a instalação: rustc --version
1:14 Hello World com rustc
3:08 O que é o Cargo
3:38 Criando um projeto: cargo new
4:08 Cargo.toml e cargo run
4:32 A pasta target e cargo build
5:08 Recapitulando
```
- [ ] Tela final → #3 (primeiro projeto) + playlist. Comentário fixado do Lote 2.

## #1 Por que aprender Rust — `hi6KAMqrb1M` (1:54)

**O que acontece:** cumprimento curto e já entra na história. Retenção boa pra um vídeo curto: 69% aos 0:11, 62%
aos 0:30, 54% aos 1:31. A queda está nos últimos 20 s, no encerramento, o que é normal.

- [ ] Legenda `01-por-que-aprender-rust.srt`. Corrige também "Grey Warren" (Graydon Hoare) e "Cloudfire" (Cloudflare).
- [ ] Tela final → #2.

## #2 Instalando Rust e VS Code no Windows — `rG3Bnn4cxcM` (1.294 views)

**O que acontece:** 0:00–0:02 sem fala, depois cumprimento. A promessa vem em 0:08. 0:17 "Beleza? Então, bora lá."
Retenção: **70% aos 0:06, 58% aos 0:10**. É o maior tombo inicial do canal. Tem um **pico em 4:00–4:35**: a
retenção sobe de 33% pra 50%. É a parte da **extensão rust-analyzer**, o trecho que as pessoas procuram e que
hoje encontram arrastando a barra.

- [ ] **Cortar 0:00–0:08 e 0:17–0:20.** O vídeo passa a abrir com "Hoje vou mostrar para você passo a passo como
      instalar o Rust e o VS Code…".
- [ ] Legenda `02-instalando-rust-windows.txt` (sincronização automática).
- [ ] Capítulos (com corte). O "Extensão rust-analyzer" é o mais importante:
```text
0:00 O que vamos instalar
0:09 Achando o site oficial
0:45 Baixando o rustup-init
1:28 Linker: Visual Studio Community
1:53 Testando: rustc --version
2:42 Instalando o VS Code
3:53 Extensão rust-analyzer
4:26 Tudo pronto
```
- [ ] Card aos 0:05: "Está no Mac?" → vídeo do macOS. Tela final → #3.

## #3 Jogo de adivinhação — `SWn-JMEurs4` (melhor retenção do canal)

**O que acontece:** "Seja muito bem-vindo ao **quarto episódio**", mas é o #3. Logo depois vem uma boa
apresentação do projeto. Retenção 64% aos 0:17 e **sobe pra 65% em 1:00**.

- [ ] **Cortar 0:00–0:08** (cumprimento + o "quarto episódio" errado). Abre em "Hoje a gente vai tá trabalhando
      juntos aqui um projeto prático…".
- [ ] Legenda `03-jogo-de-adivinhacao.txt` (sincronização automática). Corrige "business game" (guessing game),
      "LED, met" (let, mut), "crepes" (crates).
- [ ] Capítulos (com corte):
```text
0:00 O jogo que vamos construir
0:36 Criando o projeto: cargo new
1:43 Lendo o palpite com std::io
3:07 Números aleatórios: o crate rand
6:33 Comparando o palpite: Ordering e match
10:44 Repetindo com loop
13:03 Palpite inválido e fim do jogo
```

## #4 Parte 1 Variáveis, tipos e funções — `9YXmfytr_CQ`

**O que acontece:** depois de mostrar o erro do `mut`, há **13 s de silêncio (3:02–3:14)** e então "Beleza? Então
é isso, galera. Mostramos aqui então sobre as variáveis e a mutabilidade… Beleza?". Quem ouve acha que acabou: a
retenção cai de 52% (2:50) pra 42% (3:50). O vídeo segue por mais 12 minutos.

- [ ] **Cortar 3:02–3:27.** O vídeo vai do "sem o erro tá aparecendo" direto pra "…são os tipos de dados em Rust".
- [ ] Legenda `04-parte1-variaveis-tipos-funcoes.txt` (sincronização automática).
- [ ] Capítulos (com corte):
```text
0:00 Introdução
0:19 Variáveis e mutabilidade: let e mut
0:59 Constantes
1:31 Shadowing
3:02 Tipos escalares: inteiros, float, bool, char
5:31 Tipos compostos: tuplas e arrays
7:34 Funções
9:57 if e else
11:22 loop
13:02 while
13:40 for
14:22 Comentários
```

## #4 Parte 2 Ownership — `0gmJARCCPO8`

**O que acontece:** abre com teoria (garbage collector × gerenciamento manual) sem mostrar código. Perde **49% até
0:34**. Cortar não resolve, porque a explicação é necessária. O que dá pra fazer agora:

- [ ] Legenda `04-parte2-ownership.srt` (corrige "onership", "HIP" e "steack").
- [ ] Capítulos (sem corte):
```text
0:00 Modelos de memória: garbage collector, manual e ownership
1:43 Stack e heap
2:57 As regras do ownership
5:38 Move: quando o valor muda de dono
7:41 clone e Copy
10:55 Referências e borrowing
11:39 Referências soltas (dangling)
15:20 Slices
18:38 Recapitulando
```
- Pra próxima gravação sobre o tema: abrir com o erro `borrow of moved value` na tela, como o Projeto #1 faz.

## #5 Parte 1 Structs — `wJPU__13Ek8`

**O que acontece:** **0:00–0:15 são imagens de uma cidade** ("começando esse vídeo com esse conteúdo muito
maneiro… nossa cidade"). Depois vem um segundo cumprimento ("Fala Dev, beleza? Fala Rustácio"). O conteúdo só
começa em 0:20.

- [ ] **Cortar 0:00–0:20.** Abre em "Vamos então estar implementando agora o struct…".
- [ ] Legenda `05-parte1-structs.txt` (sincronização automática).
- [ ] Capítulos (com corte):
```text
0:00 Definindo uma struct
1:13 Instanciando
2:06 Acessando campos com ponto
3:37 Struct mutável
4:15 Função que cria uma struct
5:13 Field init shorthand
7:48 Struct update syntax
```

## #5 Parte 2 Tuple structs, unit structs e métodos — `aXBSyv3EIFY`

- [ ] Legenda `05-parte2-structs-metodos.srt` (corrige "struc", "selfie" e "trade").
- [ ] Capítulos (sem corte):
```text
0:00 Tuple structs
5:25 Imprimindo com Debug
6:28 Unit structs
7:08 Exemplo: área do retângulo
13:34 Métodos com impl
14:51 &self
15:59 Funções associadas
```

## #6 Enums e pattern matching — `5Ub6tYVRFYc`

- [ ] Legenda `06-enums-pattern-matching.srt` (o `match` sai como "Mat"/"MET" na automática).
- [ ] Capítulos (sem corte):
```text
0:00 O que é enum
0:27 Exemplo: endereço IP
2:24 Guardando dados nas variantes
6:22 Option: Some e None
9:28 match
16:31 if let
17:21 let else
19:28 Recapitulando
```

## #7 Sistema de módulos na prática — `fKNp_Ufiw_Q`

**Problema:** os capítulos atuais estão **num parágrafo só** ("Capítulos deste vídeo: 0:00 - … 01:30 - …"). O
YouTube só reconhece capítulos com cada tempo numa linha própria. E os tempos estão 1,5–2 min fora do lugar. A
descrição também promete `super` e `pub use`, mas não achei os dois na fala: confira e tire se não estiverem no vídeo.

- [ ] Trocar o parágrafo de capítulos por:
```text
0:00 Por que módulos
0:29 Packages e crates
1:17 Crate root: main.rs e lib.rs
5:31 Caminhos absolutos e relativos
6:59 Privacidade: pub
8:38 Atalhos com use
10:16 Conflitos de nome com as
14:34 Separando em arquivos
18:38 Recapitulando
```
- [ ] Legenda `07-modulos-na-pratica.srt`.

## #8 Módulos, crates e privacidade — `XrTfvp0EZWs`

- [ ] Legenda `08-modulos-crates-privacidade.srt`.
- [ ] Capítulos (sem corte):
```text
0:00 Introdução
0:17 Packages e crates
0:42 Criando com cargo new
2:02 Biblioteca e módulos com mod
5:53 Privacidade: pub
7:04 Structs em módulos
8:58 Enums em módulos
```

## #11 Parte 1 — `LSKlmBGf-VA` e #11 Parte 2 — `C5vEZ_uUmt4`

Os capítulos atuais saíram das estimativas do roteiro e não batem com o vídeo. Na Parte 2, enums (4:32) e
métodos (7:13) estão depois do último capítulo listado (5:00).

- [ ] Parte 1, trocar o bloco de capítulos por:
```text
0:00 Generics, traits e lifetimes: o que é cada um
0:53 O problema: achar o maior número
2:02 Duplicando a função pra char
6:06 Uma função genérica: fn largest‹T›
6:50 Erro E0369
9:30 Trait bound: PartialOrd
```
- [ ] Parte 2, trocar o bloco de capítulos por:
```text
0:00 Struct genérica: Point‹T›
0:53 Erro: inteiro e float no mesmo T
2:58 Dois parâmetros: Point‹T, U›
4:30 Generics em enums: Option e Result
7:11 Métodos genéricos: impl‹T›
11:39 Método só pra Point‹f32›
13:28 Métodos com genéricos próprios
```
- [ ] Legendas `11-parte1-generics-funcoes.srt` e `11-parte2-generics-structs.srt` ("trade" → trait,
      "genérics" → generics, "imple" → impl).

## Instalando Rust no macOS — `hhkLo6GHLlc` (43 views, 10% assistido)

**O que acontece:** 3:29–3:55 esperando o VS Code abrir. **5:31–6:56: conversa enquanto a instalação roda**
(o notebook novo, os vídeos de IA que vêm aí). Uma instalação que leva 3 min ocupa quase 11.

- [ ] **Cortar 3:29–3:55 e 5:31–6:56.** São 1:51 a menos: o vídeo vai pra ~9:00 e a parte de esperar some.
- [ ] Legenda `extra-instalando-rust-macos.txt` (sincronização automática).
- [ ] Trocar os capítulos atuais (estão em tempos que não batem) por (com corte):
```text
0:00 Achando o site oficial (e o jogo Rust)
1:17 Página de instalação: rustup e cargo
2:58 O comando de instalação
3:29 Rodando no terminal
4:04 Instalando o rustup
5:05 Instalação concluída
5:17 Verificando: rustc --version e o PATH
6:34 Cargo e o primeiro projeto
7:47 Hello, Rust!
8:07 Próximos passos
```

---

## #9 e #10: republicar divididos em partes (maior esforço, maior ganho)

Os dois são os únicos que cortar não resolve. Com 57 e 41 min, eles têm 6,5% e 4% assistido. Na série, todo
mundo que chega no #9 trava ali. Como têm só ~90 views cada, quase nada se perde ao substituí-los por partes.

**#9 em 4 partes**, cortando nos capítulos que já estão no vídeo:

| Parte | Trecho do vídeo atual | Duração | Título sugerido |
|---|---|---|---|
| 1 | 0:00–19:03 | 19 min | Curso de Rust #9 (Parte 1): Vec — criar, adicionar e ler com .get() |
| 2 | 19:03–31:30 | 12 min | Curso de Rust #9 (Parte 2): Vec e o borrow checker — percorrer e guardar tipos diferentes |
| 3 | 31:30–41:18 | 10 min | Curso de Rust #9 (Parte 3): String e UTF-8 — por que não dá pra indexar |
| 4 | 41:18–56:40 | 15 min | Curso de Rust #9 (Parte 4): HashMap — e contando palavras |

**#10 em partes:** primeiro ativar a legenda (abaixo). Depois peça pro Claude achar os pontos de corte pela
transcrição. Sugestão de divisão pelo conteúdo: panic! e backtrace · Result e match · unwrap, expect e amigos ·
propagando erro e o operador `?` · quando usar panic! ou Result (+ thiserror/anyhow).

**Como fazer:**
1. Pegar o arquivo: a gravação original ou Studio → Conteúdo → ⋮ do vídeo → **Baixar**.
2. Cortar nos tempos da tabela (qualquer editor) e **gravar uma abertura nova de 15–20 s pra cada parte**
   (fórmula de [padroes.md](padroes.md#abertura-primeiros-30-s)). Sem isso, as partes 2 a 4 começam no meio de uma frase.
3. Publicar as partes, uma por semana ou todas juntas. Colocar na playlist no lugar do vídeo longo.
4. Quando as partes estiverem no ar: tirar o vídeo longo da playlist e deixá-lo **não listado**, pra não
   concorrer com as partes na busca. Atualizar "próxima aula" no #8 e "aula anterior" no #10/#11.

## #10 — por que não tem legenda

O YouTube não gerou nem a automática. A causa mais comum é o **idioma do vídeo não estar definido**:
Studio → vídeo → Detalhes → **Idioma e certificação de legenda** → Idioma do vídeo = **Português**. Depois de salvar,
a automática aparece em algumas horas. Se não aparecer, pode ser o áudio (volume baixo ou música por cima).
