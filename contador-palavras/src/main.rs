// Projeto Rust #1 — Contador de palavras
// Lê um arquivo .txt, conta quantas vezes cada palavra aparece e mostra o ranking.
//
// Uso:
//   cargo run --release -- dom-casmurro.txt
//   cargo run --release -- dom-casmurro.txt 20

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::time::Instant;

// Palavras que aparecem em qualquer texto e não dizem nada sobre ele.
// Se alguma ainda aparecer no topo do ranking, é só adicionar aqui.
const PALAVRAS_VAZIAS: &[&str] = &[
    "a", "à", "ao", "aos", "aquela", "aquele", "aqui", "as", "às", "até", "com", "como", "da",
    "das", "de", "dela", "dele", "depois", "do", "dos", "e", "é", "ela", "elas", "ele", "eles",
    "em", "entre", "era", "essa", "esse", "esta", "está", "este", "eu", "foi", "há", "isso",
    "isto", "já", "lhe", "mais", "mas", "me", "mesmo", "meu", "minha", "muito", "na", "nas", "não",
    "nem", "no", "nos", "num", "numa", "o", "os", "ou", "para", "pela", "pelo", "por", "porque",
    "pra", "quando", "que", "se", "sem", "ser", "seu", "sua", "só", "também", "te", "tinha", "um",
    "uma", "você",
];

fn main() {
    if let Err(erro) = executar() {
        eprintln!("Erro: {erro}");
        process::exit(1);
    }
}

fn executar() -> Result<(), String> {
    // --- 1. Argumentos: qual arquivo e quantas palavras mostrar ----------------
    let mut args = env::args().skip(1);
    let caminho = args
        .next()
        .ok_or("uso: cargo run -- <arquivo.txt> [quantidade]")?;
    let quantidade = match args.next() {
        Some(n) => n.parse().map_err(|_| format!("'{n}' não é um número"))?,
        None => 10,
    };

    // --- 2. Ler o arquivo inteiro e passar pra minúsculas ----------------------
    let inicio = Instant::now();
    let texto = fs::read_to_string(&caminho)
        .map_err(|e| format!("não consegui ler '{caminho}': {e}"))?
        .to_lowercase();

    // --- 3. Contar e ordenar ---------------------------------------------------
    let contagem = contar_palavras(&texto);
    let total: usize = contagem.values().sum();
    let diferentes = contagem.len();
    let ranking = mais_frequentes(contagem, quantidade);
    let tempo = inicio.elapsed();

    // --- 4. Mostrar o resultado ------------------------------------------------
    println!("{total} palavras ({diferentes} diferentes) em {tempo:.2?}\n");

    let maior = ranking.first().map_or(1, |(_, vezes)| *vezes);
    for (posicao, (palavra, vezes)) in ranking.iter().enumerate() {
        let barra = "█".repeat(vezes * 30 / maior);
        println!("{:>2}. {palavra:<12} {vezes:>5}  {barra}", posicao + 1);
    }

    Ok(())
}

// Cada palavra é uma fatia (&str) do próprio texto: nada é copiado.
fn contar_palavras(texto: &str) -> HashMap<&str, usize> {
    let mut contagem = HashMap::new();

    for palavra in texto.split(|c: char| !c.is_alphanumeric()) {
        if palavra.is_empty() {
            continue;
        }
        *contagem.entry(palavra).or_insert(0) += 1;
    }

    contagem
}

// Recebe o HashMap por valor (ele é "consumido" aqui) e devolve as N mais frequentes.
fn mais_frequentes(contagem: HashMap<&str, usize>, quantidade: usize) -> Vec<(&str, usize)> {
    let mut lista: Vec<(&str, usize)> = contagem
        .into_iter()
        .filter(|(palavra, _)| !deve_ignorar(palavra))
        .collect();

    // mais vezes primeiro; empate fica em ordem alfabética
    lista.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    lista.truncate(quantidade);
    lista
}

fn deve_ignorar(palavra: &str) -> bool {
    palavra.chars().count() < 2 || PALAVRAS_VAZIAS.contains(&palavra)
}
