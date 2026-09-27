/// Possiveis comando VM 
#[derive(Debug)]
pub enum CommandType {
    Arithmetic, // add, sub, neg, eq, gt, lt, and, or, not
    Push,       // push <segmento> <índice>
    Pop,        // pop <segmento> <índice>

    Label,      // label <rotulo>
    Goto,       // goto <rotulo>
    IfGoto,     // if-goto <rotulo>

    Function,   // function <nome> <nVars>
    Call,       // call <nome> <nArgs>
    Return,     // return
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
    let no_comments = line
        .split("//")
        .next()
        .unwrap_or("");
    no_comments.trim().to_string()
}

/// Recebe uma linha de comando VM e retorna um struct Command
pub fn parse_command(line: &str) -> Command {
    let tokens: Vec<&str> = line.split_whitespace().collect(); // separa a linha pelos " "

    match tokens[0] {
        "push" => {
            if tokens.len() != 3 {
                panic!("Uso: push <segmento> <índice>");
            }

            Command {
                command_type: CommandType::Push,
                arg1: tokens[1].to_string(),
                arg2: Some(
                    tokens[2]
                        .parse()
                        .expect("Índice inválido no comando push"),
                ),
            }
        }

        "pop" => {
            if tokens.len() != 3 {
                panic!("Uso: pop <segmento> <índice>");
            }

            Command {
                command_type: CommandType::Pop,
                arg1: tokens[1].to_string(),
                arg2: Some(
                    tokens[2]
                        .parse()
                        .expect("Índice inválido no comando pop"),
                ),
            }
        }

        "label" => {
            if tokens.len() != 2 {
                panic!("Uso: label <rótulo>");
            }

            Command {
                command_type: CommandType::Label,
                arg1: tokens[1].to_string(),
                arg2: None,
            }
        }

        "goto" => {
            if tokens.len() != 2 {
                panic!("Uso: goto <rótulo>");
            }

            Command {
                command_type: CommandType::Goto,
                arg1: tokens[1].to_string(),
                arg2: None,
            }
        }

        "if-goto" => {
            if tokens.len() != 2 {
                panic!("Uso: if-goto <rótulo>");
            }

            Command {
                command_type: CommandType::IfGoto,
                arg1: tokens[1].to_string(),
                arg2: None,
            }
        }

        "function" => {
            if tokens.len() != 3 {
                panic!("Uso: function <nome> <nVars>");
            }

            Command {
                command_type: CommandType::Function,
                arg1: tokens[1].to_string(),
                arg2: Some(
                    tokens[2]
                        .parse()
                        .expect("Número de variáveis inválido"),
                ),
            }
        }

        "call" => {
            if tokens.len() != 3 {
                panic!("Uso: call <nome> <nArgs>");
            }

            Command {
                command_type: CommandType::Call,
                arg1: tokens[1].to_string(),
                arg2: Some(
                    tokens[2]
                        .parse()
                        .expect("Número de argumentos inválido"),
                ),
            }
        }

        "return" => {
            if tokens.len() != 1 {
                panic!("Uso: return");
            }

            Command {
                command_type: CommandType::Return,
                arg1: String::new(),
                arg2: None,
            }
        }

        // Comandos aritméticos/lógicos
        "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
            if tokens.len() != 1 {
                panic!("Comando aritmético não aceita argumentos");
            }

            Command {
                command_type: CommandType::Arithmetic,
                arg1: tokens[0].to_string(),
                arg2: None,
            }
        }

        _ => panic!("Comando VM inválido: {}", tokens[0]),
    }
}
