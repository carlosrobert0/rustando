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
- Últimos 20 s: tela final (próximo vídeo + playlist). Sem "tchau" longo.

## Thumbnail

- Gerar no thumbgen (`~/Documents/thumbgen`) e guardar o `.json` + `.png` na pasta da aula.
- No máximo **4 palavras grandes**. Tem que dar pra ler no celular, com a imagem do tamanho de um polegar.
- Projetos: o **resultado ou o número** na thumbnail (ex.: "DOM CASMURRO EM 3 MS").
- Mesma identidade: fundo escuro, laranja do Rust, o caranguejo, a marca RUSTANDO.

## Série de projetos

O #3 (jogo de adivinhação) é o vídeo com melhor retenção e mais tempo de exibição do canal. Em inglês,
vídeos "Learn Rust: Build X" fizeram de 20 a 100 vezes a média do canal. Ideias, em ordem de dificuldade:

1. ✅ Contador de palavras — Dom Casmurro (HashMap, String, erros)
2. Lista de tarefas no terminal salvando em arquivo (structs, enums, Vec, serde)
3. Jogo da cobrinha no terminal (loop, enums, crate `crossterm`)
4. Clone do `grep` (ler arquivos, iterators, testes: capítulo 12 do Book)
5. API REST com Axum do zero (async, JSON)
6. Mini Redis: servidor TCP com threads (Arc, Mutex)

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
