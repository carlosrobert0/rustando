# Plano de melhorias

> Ordem = impacto esperado. Cada item diz **por quê** (o dado do diagnóstico) e **quem faz**.
> Atualize marcando `[x]` e anotando a data. Itens novos entram embaixo da seção certa.

## ✅ Feito em 15/10/2026 (Claude)

- [x] Diagnóstico completo do canal com dados do YouTube Analytics → [diagnostico-2026-10-15.md](diagnostico-2026-10-15.md)
- [x] **Projeto Rust #1 (contador de palavras / Dom Casmurro)**: código testado em `contador-palavras/`,
      roteiro com abertura de 30 s, título, descrição, capítulos, tags, comentário fixado e thumbnail
      → [`../roteiros/projeto-01-contador-de-palavras.md`](../roteiros/projeto-01-contador-de-palavras.md)
- [x] Correções prontas pros vídeos publicados (Lote 1 e Lote 2) → [videos-existentes.md](videos-existentes.md)
- [x] Capítulos reais do #9 gerados a partir da transcrição (os atuais estão todos em 00:00)
- [x] Encontrados: #8 fora da playlist, #5 p1/p2 invertidos, #10 sem legenda, links do código do #10 e do #11
      dando 404 (pastas `erros/` e `generics/` nunca enviadas pro GitHub)
- [x] `.gitignore` na raiz do repositório (ignora `target/` e `.DS_Store`)

## 🔜 Esta semana

- [ ] **Enviar o código pro GitHub** (`erros/`, `generics/`, `contador-palavras/`, `roteiros/`, `canal/`).
      *Por quê:* as descrições do #10 e do #11 apontam pra pastas que dão 404. — você (ou peça pro Claude)
- [ ] **Aplicar o Lote 1** de [videos-existentes.md](videos-existentes.md) (7 vídeos).
      *Por quê:* capítulos quebrados, "#4" duplicado, links de exemplo. — Claude pelo vidIQ (35 créditos) ou você no Studio
- [ ] **Arrumar a playlist**: #5 p1 antes do p2, incluir o #8, trocar `[2025]` por `[2026]`.
      *Por quê:* 51% de todo o tempo de exibição vem de playlist. — você no Studio
- [ ] **Ativar/conferir legendas** do #10 (Studio → Legendas). — você
- [ ] **Gravar e publicar o Projeto #1.** — você
- [ ] **Medir o CTR atual** (Studio → Análises → Conteúdo → Taxa de cliques de impressões, últimos 90 dias)
      e anotar em [metricas.md](metricas.md). — você

## 📅 Próximas 4 semanas

- [ ] **Aplicar o Lote 2** (navegação no fim de 9 descrições + comentário fixado no "Começando com Rust").
      *Por quê:* os sugeridos trazem só 4% das views. Aula anterior/próxima em toda descrição e tela final ajudam. — você
- [ ] **Telas finais em todas as aulas** apontando pra próxima aula + playlist. — você no Studio
- [ ] **Escolher um dia fixo** e publicar 1 vídeo longo por semana (mínimo a cada 15 dias).
      *Por quê:* buracos de 4,5 meses, 3 meses e 7 semanas. As views se mantiveram, mas o crescimento não acelerou.
- [ ] **Projeto #2**: escolher entre as ideias em [padroes.md](padroes.md#série-de-projetos) e escrever o roteiro.
- [ ] **Aula #12 (lifetimes)** em até 2 partes de 12–15 min (continua o capítulo 10 do Book).
- [ ] Deixar privado o Short "Details in nature" (não é sobre Rust). — você ou Claude
- [ ] Short de 30–45 s com a revelação do Projeto #1, com "vídeo relacionado" apontando pro vídeo completo.

## 🔁 Hábitos (todo vídeo)

- [ ] Abertura de 30 s escrita, resultado na tela nos primeiros 5 s, sem vinheta/apresentação.
      *Por quê:* 60% saem até 0:36 no melhor vídeo do canal.
- [ ] Duração entre 12 e 20 min. *Por quê:* 14–20 min → 20–31% assistido; 41–57 min → 4–7%.
- [ ] "Curso de Rust #N" no título (ou "Projeto em Rust" na série de projetos). Nunca "Rust" sozinho.
- [ ] Capítulos com tempos reais, legenda conferida, tela final, card pra aula anterior.
- [ ] Responder todos os comentários nas primeiras 48 h. *Por quê:* nenhum vídeo passa de 6 comentários.
- [ ] Shorts: no máximo 1–2 por semana, espaçados, sempre um corte de aula com link pro vídeo completo.
      *Por quê:* os 9 Shorts postados em 40 minutos em 30/03 tiveram de 0 a 10 views cada. Shorts dão
      1 inscrito a cada mil views; vídeos longos dão 18.

## 🧊 Ideias pra depois

- Vídeo de entrada atualizado: "Curso de Rust 2026 para iniciantes: do zero ao primeiro projeto"
  (o "Começando com Rust" de 2025 carrega o canal sozinho e já tem mais de um ano).
- Série "Rust pra quem vem de JS/Python/Java". A audiência tem 25–44 anos e provavelmente já programa.
- Monetização: hoje são ~360 h de exibição em 12 meses. O YouTube exige 4.000 h + 1.000 inscritos. Não é meta agora.
