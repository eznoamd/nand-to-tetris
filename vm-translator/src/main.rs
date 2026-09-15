mod parser;
mod code_writer;

use std::env;
use std::fs;
use std::path::Path;

use parser::{clean_line, parse_command, CommandType};
use code_writer::CodeWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Uso: cargo run -- <arquivo.vm>");
        return;
    }

    let input_path = &args[1];

    if !input_path.ends_with(".vm") {
        eprintln!("Erro: o arquivo de entrada deve ter extensão .vm");
        return;
    }

    let content = fs::read_to_string(input_path)
        .expect("Erro ao ler o arquivo .vm");

    let file_name = Path::new(input_path)
        .file_stem()
        .expect("Erro ao obter nome do arquivo")
        .to_string_lossy()
        .to_string();

    let mut writer = CodeWriter::new(file_name);

    let mut output = String::new();

    for line in content.lines() {
        let cleaned = clean_line(line);

        if cleaned.is_empty() {
            continue;
        }

        let command = parse_command(&cleaned);

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