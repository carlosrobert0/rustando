# Padrões do canal

> Regras práticas tiradas dos dados do canal (ver [diagnóstico](diagnostico-2026-10-15.md)).
> Quando um teste contrariar uma regra, troque a regra e anote o porquê aqui.

## Título

**Curso:** `Curso de Rust #N: <tema com as palavras que alguém buscaria> 🦀`
**Partes:** `<Tema> em Rust: <promessa> | Curso de Rust #N (Parte X)` (padrão do #11, funciona bem)
**Projetos:** `Projeto em Rust para iniciantes: <o que foi construído> | Curso de Rust`

- "Curso de Rust" sempre no título: cerca de 80% dos termos de busca que trazem gente são variações de
  "curso de rust".
- **Nunca "Rust" sozinho como palavra-chave.** No Brasil, as 16 buscas mais comuns com "rust" são do jogo
  (rust gameplay, base rust, solo rust...). "Curso", "projeto", "linguagem", "programação" e "iniciantes"
  deixam claro que é programação.
- O mais importante cabe nos primeiros ~60 caracteres, que é o que aparece no celular.
- Numeração única: nada de dois "#4". Se dividir, use "(Parte 1)" e "(Parte 2)", não "p1".
- Não troque o título do **"Começando com Rust"**: ele traz 1/3 dos inscritos pela busca.

## Descrição

```text
<1ª linha: o que a pessoa vai saber fazer + palavras de busca. Ela aparece na busca e na prévia.>
<2ª linha: em qual aula/série isso se encaixa.>

📌 Neste vídeo
• tópico 1
• tópico 2

⏱️ Capítulos
0:00 ...        ← tempos reais, em ordem, cada capítulo com 10 s ou mais

💻 Código da aula
https://github.com/carlosrobert0/rustando/tree/main/<pasta>   ← conferir se a pasta foi enviada

📚 Referência
<link do The Rust Book>

📺 Playlist completa do Curso de Rust
https://www.youtube.com/playlist?list=PL5_WhrqTYdknQk5oVabIuLuJFE0RFZ55B

⬅️ Aula anterior — <nome>: https://youtu.be/<id>
➡️ Próxima aula — <nome>: https://youtu.be/<id>

<pergunta específica pra puxar comentário>

#rust #rustlang #programacao
```

- Nada de texto de exemplo esquecido (`[link da playlist]`, `playlist?list=...`, `[INSCREVA-SE]`): já aconteceu
  em 3 vídeos.
- Desculpas, avisos e "assista em 1.5x" vão pro fim, nunca na primeira linha.
- O YouTube **não aceita `<` e `>`** na descrição (os `<...>` acima são só marcadores do template). Pra código,
  use `‹T›`, como no #11.

## Abertura (primeiros 30 s)

No melhor vídeo do canal, 40% da audiência já saiu aos 36 s. A abertura é o ponto que mais rende melhorar.

1. **0–5 s:** o resultado na tela (programa rodando, erro que vai ser resolvido, número impressionante).
2. **5–15 s:** a promessa em uma frase: o que a pessoa vai saber fazer no fim.
3. **15–25 s:** um motivo pra ficar (uma pergunta em aberto, "a primeira tentativa vai dar errado").
4. **25–30 s:** "abre o terminal e vem comigo" e corte seco pro código.

Proibido antes dos 30 s: "fala pessoal", apresentação, vinheta, pedido de like/inscrição, agenda do vídeo.

## Estrutura e duração

- **12–20 min.** Acima disso, dividir em partes. (#9 com 57 min: 6,5% assistido; #10 com 41 min: 4%.)
- A cada 2–3 min, um novo "por que continuar": o próximo erro, o próximo resultado.
- Erros propositais funcionam: mostrar a mensagem real do compilador e resolver.
- CTA (like/inscrição) uma vez, **logo depois do melhor momento**, não no começo.
- **Nunca "então é isso" no meio do vídeo.** Soa como fim: no #4 Parte 1 a retenção caiu de 52% pra 42% logo
  depois dessa frase. Aparece no meio de 6 vídeos. Pra trocar de assunto: "agora vem a parte que mais confunde…".
- Pausas e esperas (instalação, abrir programa) saem na edição. No vídeo do macOS eram 1:51 de espera.
- Últimos 20 s: tela final (próximo vídeo + playlist). Sem "tchau" longo.

## Thumbnail

- Gerar no thumbgen (`~/Documents/thumbgen`) e guardar o `.json` + `.png` na pasta da aula.
- No máximo **4 palavras grandes**. Tem que dar pra ler no celular, com a imagem do tamanho de um polegar.
- Projetos: o **resultado ou o número** na thumbnail (ex.: "DOM CASMURRO EM 3 MS").
- Mesma identidade: fundo escuro, laranja do Rust, o caranguejo, a marca RUSTANDO.

## Roteiro do curso (o curso é a base)

O **Curso de Rust seguindo o Rust Book é a linha do canal** e não muda. A didática funciona:
18 inscritos a cada mil views, retenção do meio do vídeo entre as melhores do YouTube, e metade do tempo de
exibição vem de quem segue a playlist em ordem.

Os **projetos entram a cada 3–4 aulas**, do mesmo jeito que o próprio Book faz: ele alterna capítulos de
conceito com projetos (cap. 2 jogo de adivinhação, cap. 12 minigrep, cap. 21 servidor web). Regras:
- O projeto **só usa o que já foi ensinado** nas aulas, e diz de quais aulas cada parte veio.
- Quando o Book tem projeto, ele é o projeto da vez. Entre eles, um projeto próprio que junta as últimas aulas.
- O curso nunca para por causa de um projeto.

| Aulas (capítulo do Book) | Projeto depois delas |
|---|---|
| ✅ #1–#3 (cap. 1–2) | ✅ #3 Jogo de adivinhação (projeto do Book, cap. 2) |
| ✅ #4–#10 (cap. 3–9) | **Projeto #1: contador de palavras / Dom Casmurro** (HashMap, String, erros) |
| #11 generics, traits e lifetimes (cap. 10, 4 partes) · #12 testes (cap. 11, 2 partes) | **Projeto #2: minigrep** (projeto do Book, cap. 12, 2 partes) |
| #13 closures e iterators (cap. 13) · #14 Cargo e crates.io (cap. 14) · #15 smart pointers (cap. 15) | **Projeto #3: lista de tarefas no terminal** salvando em arquivo (crates `serde`/`clap`, iterators) |
| #16 concorrência (cap. 16) · #17 async (cap. 17) · #18 orientação a objetos (cap. 18) | **Projeto #4: mini Redis**, um servidor TCP com threads (`Arc`, `Mutex`) |
| #19 patterns (cap. 19) · #20 recursos avançados (cap. 20) | **Projeto final: servidor web multithread** (projeto do Book, cap. 21) |

Números das aulas a partir do #12 são sugestão. Se o #11 ganhar mais partes (traits, lifetimes), tudo anda uma casa.

**É um teste:** se o Projeto #1 não tiver mais views engajadas e % assistido que as aulas no D+28
(ver [metricas.md](metricas.md#teste-projeto-vs-aula)), os próximos projetos ficam só nos do Book.

## Shorts

- No máximo 1–2 por semana, **nunca vários no mesmo dia** (os 9 Shorts de 30/03 tiveram de 0 a 10 views cada).
- Sempre um corte de um momento forte de uma aula, com o campo **"Vídeo relacionado"** apontando pra ela.
- Curtos funcionaram melhor: os de 16 s sobre struct tiveram de 700 a 2.100 views.
- Nada fora de Rust nem Shorts de aviso ("hoje tem vídeo") — eles não trazem inscritos.

## Checklist de publicação

- [ ] Título no padrão; primeiros ~60 caracteres dizem o essencial
- [ ] Descrição no template, sem texto de exemplo, todos os links clicados e funcionando
- [ ] Código enviado pro GitHub **antes** de publicar
- [ ] Capítulos com tempos reais
- [ ] Legenda automática em português gerada (Studio → Legendas)
- [ ] Thumbnail do thumbgen
- [ ] Tela final + 1–2 cards (aula anterior / aula citada)
- [ ] Na playlist certa, na posição certa
- [ ] Vídeo normal (não Estreia), no dia fixo da semana
- [ ] Comentário fixado com uma pergunta ou desafio
- [ ] Atualizar o "➡️ Próxima aula" na descrição da aula anterior
