/// Possiveis comando VM 
#[derive(Debug)]
pub enum CommandType {
    Arithmetic, // add, sub, neg, eq, gt, lt, and, or, not
    Push,       // push <segmento> <índice>
    Pop,        // pop <segmento> <índice>
}

/// Comando VM decomposto
#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub arg1: String,
    pub arg2: Option<i32>,
}

/// Limpa comentarios e espaços em branco de uma linha 
pub fn clean_line(line: &str) -> String {
    let no_comments = line.split("//")
                                .next()
                                .unwrap_or("");
    no_comments.trim().to_string()
}

/// Recebe uma linha de comando VM e retorna um struct Command
pub fn parse_command(line: &str) -> Command {
    let tokens: Vec<&str> = line.split_whitespace().collect(); // separa a linha pelos " "

    match tokens[0] { // switch do primeiro token
        "push" => Command { // comando de push
            command_type: CommandType::Push,
            arg1: tokens[1].to_string(),               // segmento de memória
            arg2: Some(tokens[2].parse().unwrap()),     // indice
        },
        "pop" => Command { // comando de pop
            command_type: CommandType::Pop,
            arg1: tokens[1].to_string(),               // segmento de memória
            arg2: Some(tokens[2].parse().unwrap()),     // indice
        },
        cmd => Command { // qualquer outro token é um comando aritmético/lógico
            command_type: CommandType::Arithmetic,
            arg1: cmd.to_string(),
            arg2: None,
        },
    }
}
