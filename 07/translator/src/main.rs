mod parser;
mod code_writer;

use std::env;
use std::fs;
use std::path::Path;

use parser::{clean_line, parse_command, CommandType};
use code_writer::CodeWriter;

fn main() {
    // Recebe os argumentos passados pela linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se o usuário passou o arquivo .vm
    if args.len() != 2 {
        eprintln!("Uso: cargo run -- <arquivo.vm>");
        return;
    }

    let input_path = &args[1];

    // Verifica se o arquivo possui extensão .vm
    if !input_path.ends_with(".vm") {
        eprintln!("Erro: o arquivo de entrada deve ter extensão .vm");
        return;
    }

    // Lê todo o conteúdo do arquivo VM
    let content = fs::read_to_string(input_path)
        .expect("Erro ao ler o arquivo .vm");

    // Obtém o nome do arquivo sem extensão.
    // Exemplo:
    // SimpleAdd.vm -> SimpleAdd
    let file_name = Path::new(input_path)
        .file_stem()
        .expect("Erro ao obter nome do arquivo")
        .to_string_lossy()
        .to_string();

    // Cria o CodeWriter usando o nome do arquivo.
    // Isso será usado principalmente para o segmento static.
    let mut writer = CodeWriter::new(file_name);

    // String que armazenará todo o código Assembly gerado
    let mut output = String::new();

    // Processa cada linha do arquivo .vm
    for line in content.lines() {
        // Remove comentários e espaços em branco
        let cleaned = clean_line(line);

        // Ignora linhas vazias
        if cleaned.is_empty() {
            continue;
        }

        // Faz o parse do comando VM
        let command = parse_command(&cleaned);

        // Traduz o comando de acordo com seu tipo
        let assembly = match command.command_type {
            CommandType::Arithmetic => {
                writer.write_arithmetic(&command.arg1)
            }

            CommandType::Push | CommandType::Pop => {
                writer.write_push_pop(
                    &command.command_type,
                    &command.arg1,
                    command.arg2.expect("Comando push/pop sem índice"),
                )
            }
        };

        // Adiciona o Assembly gerado ao resultado final
        output.push_str(&assembly);
        output.push('\n');
    }

    // Cria o caminho do arquivo de saída.
    //
    // Exemplo:
    // StackTest.vm -> StackTest.asm
    let output_path = Path::new(input_path).with_extension("asm");

    // Escreve todo o Assembly no arquivo
    fs::write(&output_path, output)
        .expect("Erro ao escrever o arquivo .asm");

    println!(
        "Tradução concluída: {}",
        output_path.display()
    );
}