mod code;
mod parser;
mod symbol_table;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use parser::{clean_line, parse_c_command};
use symbol_table::SymbolTable;

fn main() -> io::Result<()> {
    // Coleta os argumentos da linha de comando e verifica se o arquivo de entrada foi fornecido
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <arquivo.asm>", args[0]);
        std::process::exit(1);
    }

    // Define os caminhos de entrada e saída, onde o arquivo de saída terá a extensão .hack
    let input_path = &args[1];
    let output_path = Path::new(input_path).with_extension("hack");

    let file = File::open(input_path)?; // Abre o arquivo de entrada para leitura
    let reader = BufReader::new(file);  // Cria um buffer para ler o arquivo linha por linha
    let mut raw_lines = Vec::new();     // Armazena as linhas limpas do arquivo de entrada
    
    for line in reader.lines() {           // Para cada linha do arquivo,
        let cleaned = clean_line(&line?);  // Limpa a linha removendo comentários e espaços em branco
        if !cleaned.is_empty() {           // Se a linha limpa não estiver vazia,
            raw_lines.push(cleaned);       // Adiciona a linha limpa à lista de linhas para processamento
        }
    }

    let mut symbol_table = SymbolTable::new(); // Cria uma nova tabela de símbolos para armazenar os símbolos e seus endereços
    let mut instructions = Vec::new();         // Armazena as instruções A e C para a segunda passagem do processo de montagem

    //
    // Passagem 1: Processar Labels
    //

    // Variavel para rastrear o endereço ROM atual, que é incrementado para cada instrução A ou C encontrada
    let mut rom_address = 0;
    for line in &raw_lines {                                        // Para cada linha limpa do arquivo de entrada,
        if line.starts_with('(') && line.ends_with(')') {           // Se a linha for um rótulo (inicia com '(' e termina com ')'),
            let label = &line[1..line.len() - 1];                   // Extrai o nome do rótulo removendo os parênteses
            symbol_table.add_entry(label.to_string(), rom_address); // Adiciona o rótulo à tabela de símbolos com o endereço ROM atual
        } else {                        // Se a linha for uma instrução A ou C
            rom_address += 1;           // Incrementa o endereço ROM para a próxima instrução
            instructions.push(line);    // Armazena a instrução para a segunda passagem do processo de montagem
        }
    }

    //
    // Passagem 2: Processar Comandos A e C
    //

    // Abre o arquivo de saída para escrita, onde as instruções binárias serão gravadas
    let mut output_file = File::create(&output_path)?;

    for line in instructions {                                      // Para cada instrução A ou C armazenada na primeira passagem,
        if line.starts_with('@') {                                  // Se a linha for um comando A (inicia com '@'),

            let symbol = &line[1..];                                // Extrai o símbolo ou valor do comando A removendo o '@'
            let address = if let Ok(val) = symbol.parse::<u16>() {  // Tenta converter o símbolo para um número, se for um valor numérico,
                val
            } else {                                                // Se o símbolo não for um valor numérico,
                symbol_table.get_address(symbol)                    // Obtém o endereço correspondente da tabela de símbolos (ou atribui um novo endereço se for uma variável)
            };
            writeln!(output_file, "0{:015b}", address)?; // Escreve a instrução A em formato binário de 16 bits no arquivo de saída
            
        } else {                                    // Se a linha for um comando C,
            let (d, c, j) = parse_c_command(line);  // Analisa a instrução C para extrair as partes dest, comp e jump
            writeln!(           // Escreve a instrução C em formato binário de 16 bits no arquivo de saída
                output_file,
                "111{}{}{}",
                code::comp(c),
                code::dest(d),
                code::jump(j)
            )?;
        }
    }

    println!("Arquivo compilado com sucesso: {:?}", output_path);
    Ok(())
}