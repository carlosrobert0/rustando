use std::collections::HashMap;

fn main() {
    // =====================================================================
    // 1. VECTOR - a lista que cresce
    // =====================================================================

    // --- 1.1 Criando ---------------------------------------------------
    {
        // vazio: aqui o Rust não consegue adivinhar o tipo, então a gente anota
        let v: Vec<i32> = Vec::new();

        // com valores iniciais: aqui o Rust infere sozinho que é Vec<i32>
        let numbers = vec![1, 2, 3];

        println!("1.1 -> {v:?} {numbers:?}");
    }

    // --- 1.2 Adicionando -------------------------------------------------
    {
        // precisa de `mut`, porque adicionar item muda o vector.
        // sem anotação de tipo: o push(5) já diz ao Rust que é Vec<i32>.
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);

        println!("1.2 -> {v:?}");
    }

    // --- 1.3 Lendo: índice vs. get ---------------------------------------
    {
        let v = vec![1, 2, 3, 4, 5];

        // jeito 1: índice -> dá panic se não existir
        let third: &i32 = &v[2];
        println!("1.3 -> O terceiro elemento é {third}");

        // jeito 2: get -> devolve Option
        let third: Option<&i32> = v.get(2);
        match third {
            Some(value) => println!("1.3 -> O terceiro elemento é {value}"),
            None => println!("1.3 -> Não existe terceiro elemento."),
        }

        // fora do intervalo
        // let does_not_exist = &v[100];  // PANIC em tempo de execução!
        let does_not_exist = v.get(100); // None, tranquilo
        println!("1.3 -> v.get(100) = {does_not_exist:?}");
    }

    // --- 1.4 Borrow checker no vector ------------------------------------
    {
        let mut v = vec![1, 2, 3, 4, 5];

        // NÃO COMPILA: empréstimo imutável seguido de empréstimo mutável.
        // O vector guarda os elementos lado a lado; ao crescer ele pode
        // realocar e copiar tudo, deixando `first` apontando pra memória
        // liberada. O borrow checker barra isso na compilação.
        //
        // let first = &v[0];        // empréstimo imutável
        // v.push(6);                // empréstimo mutável -> erro!
        // println!("O primeiro é {first}");

        // versão que compila: usa o empréstimo antes de mutar
        let first = v[0]; // copia o valor (i32 é Copy)
        v.push(6);
        println!("1.4 -> primeiro = {first}, v = {v:?}");
    }

    // --- 1.5 Percorrendo --------------------------------------------------
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("1.5 -> {i}");
        }

        // alterando cada elemento: referência mutável + `*` (desreferência)
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        println!("1.5 -> {v:?}");
    }

    // --- 1.6 Vector de "tipos diferentes" via enum ------------------------
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        // o Rust conhece todos os casos possíveis em tempo de compilação,
        // então o match precisa tratar todos eles.
        for cell in &row {
            match cell {
                SpreadsheetCell::Int(i) => println!("1.6 -> int: {i}"),
                SpreadsheetCell::Float(f) => println!("1.6 -> float: {f}"),
                SpreadsheetCell::Text(t) => println!("1.6 -> text: {t}"),
            }
        }
    }

    // =====================================================================
    // 2. STRING - mais complicada do que parece
    // =====================================================================

    // --- 2.1 Criando ------------------------------------------------------
    {
        let s = String::new();
        println!("2.1 -> string vazia: {s:?}");

        let data = "conteúdo inicial";
        let s = data.to_string();
        println!("2.1 -> {s}");

        let s = "conteúdo inicial".to_string();
        println!("2.1 -> {s}");

        // to_string() e String::from() fazem a mesma coisa: preferência de estilo
        let s = String::from("conteúdo inicial");
        println!("2.1 -> {s}");

        // como é UTF-8, cabe qualquer coisa aqui dentro
        let hello = String::from("Olá");
        println!("2.1 -> {hello}");
        let hello = String::from("こんにちは");
        println!("2.1 -> {hello}");
        let hello = String::from("Здравствуйте");
        println!("2.1 -> {hello}");
    }

    // --- 2.2 Atualizando --------------------------------------------------
    {
        let mut s = String::from("foo");
        s.push_str("bar"); // adiciona uma fatia (&str)
        s.push('!'); // adiciona UM caractere
        println!("2.2 -> {s}"); // foobar!

        // push_str não toma posse do argumento
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("2.2 -> s1 = {s1}, s2 continua sendo {s2}");
    }

    // --- 2.3 Concatenando: a pegadinha do `+` -----------------------------
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // assinatura por trás: fn add(self, s: &str) -> String
        // s1 entra POR VALOR -> foi movido e deixou de existir.
        let s3 = s1 + &s2;
        println!("2.3 -> {s3}");
        // println!("{s1}");  // ERRO: s1 foi movido para dentro do `+`

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // funciona, mas... eca (repara que s1 é movido aqui)
        let s = s1.clone() + "-" + &s2 + "-" + &s3;
        println!("2.3 -> {s}");

        // muito melhor: format! não toma posse de nada
        let s = format!("{s1}-{s2}-{s3}");
        println!("2.3 -> {s}");
    }

    // --- 2.4 Por que não dá pra indexar string ----------------------------
    {
        let s1 = String::from("hello");
        // let h = s1[0];  // ERRO: `String` cannot be indexed by `{integer}`
        println!("2.4 -> {s1} tem {} bytes", s1.len());

        let hello = String::from("Hola");
        println!("2.4 -> \"Hola\" -> {} bytes", hello.len()); // 4

        let hello = String::from("Здравствуйте");
        println!("2.4 -> \"Здравствуйте\" -> {} bytes", hello.len()); // 24, não 12!

        // dá pra fatiar por intervalo, mas é por sua conta e risco
        let hello = "Здравствуйте";
        let s = &hello[0..4]; // "Зд" - funciona
        println!("2.4 -> &hello[0..4] = {s}");
        // let s = &hello[0..1];  // PANIC em tempo de execução:
        //                        // byte index 1 is not a char boundary
    }

    // --- 2.5 O jeito certo: iterar ----------------------------------------
    {
        for c in "Зд".chars() {
            println!("2.5 -> char: {c}");
        }

        for b in "Зд".bytes() {
            println!("2.5 -> byte: {b}");
        }
    }

    // =====================================================================
    // 3. HASHMAP - chave e valor
    // =====================================================================

    // --- 3.1 Criando e inserindo ------------------------------------------
    // (HashMap não vem no prelúdio: precisa do `use std::collections::HashMap;`)
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("3.1 -> {scores:?}");

        // --- 3.2 Lendo ----------------------------------------------------
        let team_name = String::from("Blue");
        // get         -> Option<&i32>
        // copied()    -> Option<i32>
        // unwrap_or(0)-> i32
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("3.2 -> {score}");

        // --- 3.3 Percorrendo (ordem arbitrária!) --------------------------
        for (key, value) in &scores {
            println!("3.3 -> {key}: {value}");
        }
    }

    // --- 3.4 Ownership ----------------------------------------------------
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // println!("{field_name}");  // ERRO: foi movido pra dentro do map
        // i32 e outros tipos Copy são copiados; String é MOVIDA.
        println!("3.4 -> {map:?}");
    }

    // --- 3.5 Sobrescrever vs. inserir só se não existir --------------------
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // sobrescreve

        println!("3.5 -> {scores:?}"); // {"Blue": 25}

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50); // não existe -> insere
        scores.entry(String::from("Blue")).or_insert(50); // já existe -> ignora

        println!("3.5 -> {scores:?}"); // {"Blue": 10, "Yellow": 50}
    }

    // --- 3.6 O exemplo clássico: contar palavras ---------------------------
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // or_insert devolve uma referência mutável pro valor
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("3.6 -> {map:?}"); // {"hello": 1, "world": 2, "wonderful": 1}
    }
}
