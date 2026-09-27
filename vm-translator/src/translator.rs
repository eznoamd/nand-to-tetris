use std::fs;
use std::path::{Path, PathBuf};

use crate::code_writer::CodeWriter;
use crate::parser::{clean_line, parse_command, CommandType};

/// Recebe o caminho informado pelo usuário e decide
/// se deve traduzir um arquivo .vm ou um diretório.
pub fn translate(input: &str) -> Result<(), String> {
    let input_path = Path::new(input);

    // verifica se o caminho informado realmente existe
    if !input_path.exists() {
        return Err("caminho não encontrado".to_string());
    }

    // se for um arquivo, traduz apenas esse arquivo
    if input_path.is_file() {
        translate_file(input_path)
    }
    // se for um diretório, procura todos os arquivos .vm
    // e gera um único arquivo .asm
    else if input_path.is_dir() {
        translate_directory(input_path)
    }
    // caso não seja nem arquivo nem diretório
    else {
        Err("entrada inválida".to_string())
    }
}

/// Traduz um único arquivo .vm
fn translate_file(input_path: &Path) -> Result<(), String> {

    // verifica se o arquivo tem ".vm" no final
    if input_path.extension().and_then(|ext| ext.to_str()) != Some("vm") {
        return Err("o arquivo de entrada deve ter extensão .vm".to_string());
    }

    // pega apenas o nome do arquivo, sem a extensão
    // exemplo: Main.vm -> Main
    let file_name = input_path
        .file_stem()
        .ok_or("erro ao obter nome do arquivo")?
        .to_string_lossy()
        .to_string();

    // pega o conteúdo do arquivo .vm
    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("erro ao ler arquivo: {}", e))?;

    // cria o CodeWriter responsável por transformar
    // os comandos VM em Assembly
    let mut writer = CodeWriter::new(file_name);

    // variável que guarda todo o conteúdo do arquivo .asm
    let mut output = String::new();

    // traduz cada comando do arquivo VM
    translate_content(
        &content,
        &mut writer,
        &mut output,
    );

    // troca a extensão .vm por .asm
    let output_path = input_path.with_extension("asm");

    // escreve todo o Assembly gerado no arquivo de saída
    fs::write(&output_path, output)
        .map_err(|e| format!("erro ao escrever arquivo: {}", e))?;

    println!(
        "Tradução concluída: {}",
        output_path.display()
    );

    Ok(())
}

/// Traduz todos os arquivos .vm de um diretório
fn translate_directory(input_path: &Path) -> Result<(), String> {

    // procura todos os arquivos .vm existentes na pasta
    let vm_files = get_vm_files(input_path);

    // verifica se encontrou pelo menos um arquivo .vm
    if vm_files.is_empty() {
        return Err("nenhum arquivo .vm encontrado no diretório".to_string());
    }

    // pega o nome do diretório
    // exemplo: ./FunctionCalls -> FunctionCalls
    let directory_name = input_path
        .file_name()
        .ok_or("erro ao obter nome do diretório")?
        .to_string_lossy()
        .to_string();

    // o arquivo .asm terá o mesmo nome da pasta
    //
    // FunctionCalls/
    //     Main.vm
    //     Sys.vm
    //
    // vira:
    //
    // FunctionCalls.asm
    let output_path = input_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(format!("{}.asm", directory_name));

    // cria um único CodeWriter para todos os arquivos .vm
    let mut writer = CodeWriter::new(String::new());

    // variável que guarda todo o Assembly gerado
    // pelos vários arquivos .vm
    let mut output = String::new();

    // o bootstrap deve ser gerado apenas uma vez,
    // no começo do arquivo .asm
    //
    // ele inicializa o SP e chama Sys.init
    output.push_str(&writer.write_init());

    // percorre todos os arquivos .vm encontrados
    for vm_file in vm_files {

        // pega apenas o nome do arquivo, sem ".vm"
        // exemplo: Main.vm -> Main
        let file_name = vm_file
            .file_stem()
            .ok_or("erro ao obter nome do arquivo")?
            .to_string_lossy()
            .to_string();

        // informa ao CodeWriter qual arquivo está sendo traduzido
        //
        // isso é importante principalmente para os comandos
        // static, que usam o nome do arquivo:
        //
        // Main.0
        // Main.1
        writer.set_file_name(file_name);

        // lê o conteúdo do arquivo .vm
        let content = fs::read_to_string(&vm_file)
            .map_err(|e| {
                format!(
                    "erro ao ler {}: {}",
                    vm_file.display(),
                    e
                )
            })?;

        // traduz o conteúdo desse arquivo e adiciona
        // o Assembly gerado ao output
        translate_content(
            &content,
            &mut writer,
            &mut output,
        );
    }

    // depois de traduzir todos os arquivos,
    // escreve tudo em um único arquivo .asm
    fs::write(&output_path, output)
        .map_err(|e| format!("erro ao escrever arquivo: {}", e))?;

    println!(
        "Tradução concluída: {}",
        output_path.display()
    );

    Ok(())
}

/// Obtém todos os arquivos .vm de um diretório
fn get_vm_files(directory: &Path) -> Vec<PathBuf> {
    // vetor que vai armazenar os caminhos dos arquivos .vm
    let mut files = Vec::new();

    // lê todas as entradas existentes no diretório
    let entries = fs::read_dir(directory)
        .expect("erro ao ler diretório");

    // percorre cada entrada encontrada
    for entry in entries {

        // obtém a entrada atual
        let entry = entry.expect("erro ao ler entrada");

        // obtém o caminho da entrada
        let path = entry.path();

        // verifica se é um arquivo e se possui extensão .vm
        if path.is_file()
            && path.extension().and_then(|ext| ext.to_str()) == Some("vm")
        {
            // adiciona o arquivo à lista
            files.push(path);
        }
    }

    // ordena os arquivos para que a ordem de tradução
    // seja sempre determinística
    files.sort();

    files
}

/// Traduz o conteúdo de um arquivo VM
/// usando o CodeWriter recebido.
fn translate_content(
    content: &str,
    writer: &mut CodeWriter,
    output: &mut String,
) {
    // percorre cada linha do arquivo VM
    for line in content.lines() {

        // remove comentários e espaços em branco
        let cleaned = clean_line(line);

        // ignora linhas vazias ou que continham apenas comentários
        if cleaned.is_empty() {
            continue;
        }

        // transforma a linha em um Command
        // identificando o tipo e seus argumentos
        let command = parse_command(&cleaned);

        // verifica qual tipo de comando foi encontrado
        // e chama a função correspondente do CodeWriter
        let assembly = match command.command_type {
            CommandType::Arithmetic => {
                writer.write_arithmetic(&command.arg1)
            }
            CommandType::Push | CommandType::Pop => {
                writer.write_push_pop(
                    &command.command_type,
                    &command.arg1,
                    command.arg2.expect("push/pop sem índice"),
                )
            }
            CommandType::Label => {
                writer.write_label(&command.arg1)
            }
            CommandType::Goto => {
                writer.write_goto(&command.arg1)
            }
            CommandType::IfGoto => {
                writer.write_if(&command.arg1)
            }
            CommandType::Function => {
                writer.write_function(
                    &command.arg1,
                    command.arg2.expect("function sem nVars"),
                )
            }
            CommandType::Call => {
                writer.write_call(
                    &command.arg1,
                    command.arg2.expect("call sem nArgs"),
                )
            }
            CommandType::Return => {
                writer.write_return()
            }
        };

        output.push_str(&assembly);

        output.push('\n');
    }
}