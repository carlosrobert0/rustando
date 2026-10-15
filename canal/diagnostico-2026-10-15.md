# Diagnóstico do canal — 15/10/2026

> Fonte: YouTube Analytics do canal via vidIQ (jul/2025 a 14/10/2026), pesquisa de palavras-chave do vidIQ
> (Brasil), playlist pública e descrições dos vídeos. Os números mensais e por vídeo estão em [metricas.md](metricas.md).

## Resumo

Quem chega ao canal gosta e se inscreve. O problema é que pouca gente nova chega. O canal depende de busca
e playlist, quase metade da audiência sai nos primeiros 30–40 s, e os vídeos mais longos afundaram a retenção.

- **272 inscritos**, +23/mês. Nesse ritmo, 1.000 inscritos só em meados de 2029.
- Um vídeo segura o canal: **"Começando com Rust"** trouxe 96 inscritos (~35% de todos).
- **Vídeo longo dá 18 inscritos a cada mil views; Shorts dá 1.** Os Shorts deram 37% das views e ~7 inscritos.
- Os likes ficam em ~5% das views nos episódios do curso: quem assiste gosta.

## De onde vêm as views (jul/25 a out/26)

| Origem | Views | % das views | Tempo de exibição (min) | % do tempo |
|---|---:|---:|---:|---:|
| Feed de Shorts | 5.593 | 34% | 300 | 1% |
| Busca do YouTube | 3.712 | 22% | 4.081 | 18% |
| Playlists | 3.629 | 22% | 11.753 | **51%** |
| Outros (sem link) | 877 | 5% | 1.437 | 6% |
| Página inicial / inscrições | 823 | 5% | 1.210 | 5% |
| **Vídeos sugeridos** | 694 | **4%** | 2.164 | 9% |
| Outras páginas do YouTube | 665 | 4% | 1.048 | 5% |
| Página do canal | 472 | 3% | 783 | 3% |
| Sites externos | 110 | 1% | 210 | 1% |

- **A playlist é o motor:** metade de todo o tempo de exibição. Mas ela está com o #5 p1/p2 invertido e sem o #8.
- **Os vídeos sugeridos quase não aparecem (4%).** Na maioria dos canais, essa é a fonte que mais faz crescer.
- **Buscas:** "curso de rust" (372 views), "curso rust" (241), "rust curso" (100): cerca de 80% dos termos
  identificados. Depois vêm "rust linguagem de programação" (34) e "linguagem rust" (23).
- **"Rust" no Brasil é o jogo:** as 16 buscas mais comuns com "rust" são todas do jogo (rust: 85 mil/mês,
  breaking rust: 44 mil, base rust: 30 mil...).

## Retenção

**Abertura.** Porcentagem da audiência que ainda está assistindo:

| Vídeo | 0:07 | 0:18 | 0:36 | 1:00 | metade | fim |
|---|---:|---:|---:|---:|---:|---:|
| Começando com Rust (6 min) | 75% | 54% | 40% | 35% | 31% | 11% |
| #3 Jogo de adivinhação (14 min) | 98% | 64% | 58% | 65% | 32% | 13% |
| #9 Coleções (57 min) | — | — | 89% | 32% (1:08) | 8% | 3% |

No #3 a curva **sobe** perto de 1:00: a audiência volta pra rever um trecho, bom sinal.

- No "Começando com Rust", a retenção comparada com vídeos de duração parecida fica entre os **20–30% piores**
  nos primeiros segundos e entre os **~25% melhores** do meio em diante. O conteúdo é bom; a abertura é que afasta.
- O **#3 fica acima da mediana do YouTube durante quase todo o vídeo** (até 84% no relativo).

**Duração:** 14–20 min → 20–31% assistido; 41 min → 4%; 57 min → 6,5%.

**Série:** a audiência cai a cada episódio (#1: 1.180 views → #4: 496 → #6: 183 → #8: 102 → #10: 91).
Isso é normal em curso, e é por isso que cada aula precisa funcionar sozinha na busca e apontar pra próxima.

## Melhores e piores

**Melhores:** Começando com Rust (4.486 views, 96 inscritos) · #3 Jogo de adivinhação (melhor retenção, mais
tempo de exibição: 5.777 min) · #2 Instalando no Windows (36,7% assistido) · #1 Por que aprender Rust (53%).

**Piores:**
- 9 Shorts de módulos postados em 40 minutos em 30/03: de 0 a 10 views cada.
- #10 Erros (41 min): 91 views, 4% assistido, **sem legenda**.
- #9 Coleções (57 min): 90 views; 68% saem no primeiro minuto.
- Instalar no macOS: 43 views, 10% assistido.
- Shorts de aviso ("Hoje tem vídeo no canal…") e fora do tema ("Details in nature"): views sem inscritos.

## Tendência

- O pico foi em abr–mai/26: ~1.200 views engajadas e ~3.200 min por mês. Em ago–set caiu pra ~860 views e
  ~1.900 min (−40% de tempo de exibição). A queda coincide com os vídeos de 41–57 min e com 7 semanas sem aula.
- Publicação em rajadas: buracos de 4,5 meses (ago→dez/25), 3 meses (jan→mar/26) e 7 semanas (ago→out/26).

## Audiência

98% Brasil · ~98% homens · 25–34 anos: 44% · 18–24: 24% · 35–44: 22% · 45–54: 8%.

## Concorrência e oportunidade

- Entre os 15 canais mais parecidos (vidIQ), **nenhum é focado em Rust em português**. Os mais próximos são canais
  gerais de programação de 50 a 17 mil inscritos.
- Em inglês, o formato **"Learn Rust: Build X"** (canal Will Velida, 30 mil inscritos) fez de 20 a 100 vezes a média:
  contador de palavras em CLI, clone do Redis, simulador em WebAssembly. Cursos completos "Rust 2026 for
  beginners" também estouraram (Codynn, 38 mil inscritos).

## Problemas técnicos encontrados

- Capítulos do #9 e do #10 todos em `00:00`, então o YouTube não reconhece nenhum.
- Dois vídeos com "#4" no título.
- Texto de exemplo esquecido nas descrições: `[link da playlist]` (#10), `playlist?list=...` (#4 Ownership),
  `[INSCREVA-SE]` (#8), "→ Livro Oficial" sem link (#4 Variáveis).
- Links do código do #10 e do #11 dão **404**: as pastas `erros/` e `generics/` nunca foram enviadas pro GitHub.
- #11 Parte 2 aponta pra Parte 1 com `&t=209s`, que abre o vídeo em 3:29.
- Primeira linha da descrição do #6 é "Obs.: Caso ache lento...".
- O CTR não está disponível pelo vidIQ. Precisa ver no Studio.
