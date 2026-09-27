mod parser;
mod code_writer;

use std::env;
use std::fs;
use std::path::Path;

use parser::{clean_line, parse_command, CommandType};
use code_writer::CodeWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    // garante que tera o (./nome-arquivo arg1) com tamanho 2
    if args.len() != 2 {    
        println!("Uso: cargo run -- <arquivo.vm>");
        return;
    }

    // arquivo vm
    let input_path = &args[1];

    // verifica se o arquivo tem ".vm" no final
    if !input_path.ends_with(".vm") {
        println!("Erro: o arquivo de entrada deve ter extensão .vm");
        return;
    }

    // pega o conteudo do arquivo vm
    let content = fs::read_to_string(input_path)
        .expect("Erro ao ler o arquivo .vm");

    // OsStr -> Cow (Clone on write) -> String
    let file_name = Path::new(input_path)
        .file_stem().expect("Erro ao obter nome do arquivo") // pega apenas o que vem antes do ultimo ponto
        .to_string_lossy() // garante que seja uma String Cow<>, mesmo que contenha caracteres não UTF-8
        .to_string(); // transforma o Cow<> em String 

    // cria um objeto da minha classe CodeWriter 
    let mut writer = CodeWriter::new(file_name);

    // variavel que guarda todo conteudo de output
    let mut output = String::new();

    for line in content.lines() { // para cada linha do conteudo
        let cleaned = clean_line(line); // limpa as em branco ou comentarios

        if cleaned.is_empty() { continue; } // apenas continua se não é uma linha em branco/limpa

        let command = parse_command(&cleaned); // escolhe que tipo de comando é essa linha e monta
                                               // a estrutura de comando dele 

        let assembly = match command.command_type { // variavel que vai guardar o comando atual
                                                    // para depois enviar para o output
            CommandType::Arithmetic => { // se é aritimetico
                writer.write_arithmetic(&command.arg1)
            }

            CommandType::Push | CommandType::Pop => { // se é pop ou push
                writer.write_push_pop(
                    &command.command_type,
                    &command.arg1,
                    command.arg2.expect("Comando push/pop sem índice"),
                )
            }
        };

        // guarda o comando no output
        output.push_str(&assembly);
        output.push('\n');  
    }

    let output_path = Path::new(input_path).with_extension("asm");

    fs::write(&output_path, output)
        .expect("Erro ao escrever o arquivo .asm");

    println!(
        "Tradução concluída: {}",
        output_path.display()
    );
}
