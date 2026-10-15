# Rustando — central do canal

Tudo o que é sobre **crescer o canal** fica nesta pasta. O código das aulas continua nas pastas de cada aula,
e os roteiros em `../roteiros/`.

| Arquivo | Pra quê |
|---|---|
| [plano-de-melhorias.md](plano-de-melhorias.md) | **Comece aqui.** O que fazer, em que ordem, e o que já foi feito |
| [padroes.md](padroes.md) | Como fazer título, descrição, abertura, thumbnail, Shorts e telas finais |
| [videos-existentes.md](videos-existentes.md) | Correções prontas (título/descrição) pros vídeos já publicados |
| [metricas.md](metricas.md) | Números mês a mês e por vídeo: onde olhar e o que é bom |
| [diagnostico-2026-10-15.md](diagnostico-2026-10-15.md) | A análise completa que originou o plano (linha de base) |

## Onde o canal está (15/10/2026)

| Indicador | Agora | Meta até 15/01/2027 |
|---|---|---|
| Inscritos | 272 | 400 |
| Inscritos líquidos por mês | +23 | +45 |
| Views engajadas por mês | ~860 (ago–set) | 1.500 |
| Tempo de exibição por mês | ~1.900 min (ago–set) | 3.500 min |
| Audiência que fica até 0:30 (vídeos novos) | ~40% (melhor vídeo) | 55% ou mais |
| Views vindas de vídeos sugeridos | 4% | 10% |
| CTR de impressões (vídeos longos) | ver no Studio → preencher em `metricas.md` | 4% ou mais |
| Ritmo de publicação | em rajadas, com buracos de até 4,5 meses | 1 vídeo longo por semana (mínimo a cada 15 dias) |

**O diagnóstico em três linhas:** o conteúdo converte bem (18 inscritos a cada mil views nos vídeos longos),
mas o canal vive de busca ("curso de rust") e de playlist. Quase metade da audiência sai nos primeiros 30–40 s,
e os vídeos de 41–57 min afundaram a retenção. O formato que mais segura gente é o **projeto** (#3).

## Rotina

**A cada vídeo**
1. Roteiro em `../roteiros/` seguindo [padroes.md](padroes.md): abertura de 30 s escrita palavra por palavra.
2. Gravar em até 15 min. Se passar, dividir em partes.
3. Publicar com o checklist de [padroes.md](padroes.md#checklist-de-publicação).
4. **D+7 e D+28:** anotar os números do vídeo em [metricas.md](metricas.md#por-vídeo).

**Todo mês (dia 15, quando os créditos do vidIQ renovam)**
- Pedir pro Claude: *"atualiza as métricas do Rustando"*. Ele puxa pelo vidIQ (~30 créditos de 150) e
  preenche a tabela mensal de [metricas.md](metricas.md). O CTR não vem pelo vidIQ: copie do Studio.
- Revisar o [plano](plano-de-melhorias.md): marcar o que foi feito e escolher as próximas 3 tarefas.

## Ferramentas

- **vidIQ (MCP no Claude)**: conta crobertoqs@gmail.com, plano grátis, 150 créditos/mês, renova dia 15.
  Analytics, capítulos, transcrição e edição de título/descrição custam 5 créditos cada. Não traz CTR.
- **thumbgen** (`~/Documents/thumbgen`): gera as thumbnails a partir dos `.json` de cada aula.
- **Código**: https://github.com/carlosrobert0/rustando (as descrições apontam pras pastas daqui).
