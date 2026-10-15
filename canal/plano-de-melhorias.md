# Plano de melhorias

> **O curso seguindo o Rust Book é a base do canal e não muda.** Quase tudo aqui é embalagem (abertura, título,
> descrição, playlist, telas finais) e ritmo. A única mudança de conteúdo é encaixar um projeto a cada 3–4 aulas,
> como o próprio Book faz (ver [padroes.md](padroes.md#roteiro-do-curso-o-curso-é-a-base)).
>
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
- [x] **Código enviado pro GitHub** (commit `94c5d63`): `erros/`, `generics/`, `contador-palavras/`, `roteiros/`, `canal/`.
      Os links do código do #10 e do #11 voltaram a funcionar. `target/` de `enums` e `structs` saiu do repositório.
- [x] **Lote 1 aplicado pelo vidIQ nos 7 vídeos** (#9, #10, #4 p1, #4 p2, #6, #11 p1, #11 p2). Conferido na página
      pública: títulos novos na playlist e os 12 capítulos do #9 reconhecidos pelo YouTube.

- [x] **Revisão do conteúdo de 15 vídeos** pelas legendas automáticas + retenção → [melhorias-por-video.md](melhorias-por-video.md):
      cortes no Editor do Studio, capítulos de todos os vídeos e **legendas corrigidas** em [legendas/](legendas/).
- [x] **Roteiros dos próximos 6 vídeos**, com código e erros conferidos: #11 Parte 3 (traits), #11 Parte 4
      (lifetimes), #12 Partes 1 e 2 (testes), Projeto #2 minigrep (2 partes) → [calendário](../roteiros/README.md)

## 🔜 Esta semana

- [ ] **Arrumar a playlist**: #5 p1 antes do p2, incluir o #8, trocar `[2025]` por `[2026]`.
      *Por quê:* 51% de todo o tempo de exibição vem de playlist. — você no Studio
- [ ] **Ativar a legenda do #10**: Detalhes → Idioma do vídeo = Português (a causa mais comum de não ter legenda automática). — você
- [ ] **Gravar e publicar o Projeto #1** (22/10). — você
- [ ] **Cortes no Editor do Studio** nos 6 vídeos de [melhorias-por-video.md](melhorias-por-video.md): Começando com Rust, #2,
      #3, #4 p1, #5 p1, macOS. *Por quê:* nos 4 vídeos de entrada, 30–46% da audiência sai nos primeiros 10–18 s, ainda no cumprimento e na apresentação. — você
- [ ] **Enviar as legendas corrigidas** (depois dos cortes). *Por quê:* a legenda automática erra "Rust" em ~70% das vezes. — você
- [ ] **Colar os capítulos** de [melhorias-por-video.md](melhorias-por-video.md) (13 vídeos; no #7, #11 e macOS eles substituem os atuais). — você
- [ ] **Medir o CTR atual** (Studio → Análises → Conteúdo → Taxa de cliques de impressões, últimos 90 dias)
      e anotar em [metricas.md](metricas.md). — você

## 📅 Próximas 4 semanas

- [ ] **Aplicar o Lote 2** (navegação no fim de 9 descrições + comentário fixado no "Começando com Rust").
      *Por quê:* os sugeridos trazem só 4% das views. Aula anterior/próxima em toda descrição e tela final ajudam. — você
- [ ] **Telas finais em todas as aulas** apontando pra próxima aula + playlist. — você no Studio
- [ ] **Escolher um dia fixo** e publicar 1 vídeo longo por semana (mínimo a cada 15 dias).
      *Por quê:* buracos de 4,5 meses, 3 meses e 7 semanas. As views se mantiveram, mas o crescimento não acelerou.
- [ ] **Seguir o calendário** de [../roteiros/README.md](../roteiros/README.md): #11 Parte 3 e 4, #12 Partes 1 e 2, minigrep.
      O roteiro completo de aulas e projetos está em [padroes.md](padroes.md#roteiro-do-curso-o-curso-é-a-base).
- [ ] **Republicar o #9 em 4 partes** quando sobrar uma semana (ver [melhorias-por-video.md](melhorias-por-video.md#9-e-10-republicar-divididos-em-partes-maior-esforço-maior-ganho)).
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
