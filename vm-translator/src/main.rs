mod parser;
mod code_writer;
mod translator;

use std::env;

fn main() {
    // Pega os argumentos passados na linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se a quantidade de args esta certa
    //
    // São dois args pois:
    // em binario ele é executado ./nome args
    // o primeiro arg é o nome do programa
    // o segundo é o nosso arg principal do programa
    if args.len() != 2 {
        println!("Uso:");
        println!("  cargo run -- <arquivo.vm>");
        println!("  cargo run -- <diretorio>");
        return;
    }

    // Executa o translator::translate() 
    // e se tiver algum erro de execução ele printa o erro
    if let Err(error) = translator::translate(&args[1]) {
        eprintln!("Erro: {}", error);
    }
}