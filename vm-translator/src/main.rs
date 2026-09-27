mod parser;
mod code_writer;
mod translator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Uso:");
        println!("  cargo run -- <arquivo.vm>");
        println!("  cargo run -- <diretorio>");
        return;
    }

    if let Err(error) = translator::translate(&args[1]) {
        eprintln!("Erro: {}", error);
    }
}