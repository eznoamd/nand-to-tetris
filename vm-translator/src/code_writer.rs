use crate::parser::CommandType;

/// Responsável por traduzir comandos VM
pub struct CodeWriter {
    file_name: String,  // nome do arquivo .vm
    label_count: u32,   // contador usado para gerar rótulos únicos nos comandos de comparação
}

impl CodeWriter {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            label_count: 0,
        }
    }

    /// Traduz um comando aritmético/lógico para Assembly
    pub fn write_arithmetic(&mut self, command: &str) -> String {
        match command {
            // Comandos binários: consomem o topo da pilha (y o primeiro, e x o segundo),
            // deixando o resultado da operação no lugar de x
            "add" => "@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n".to_string(), // x + y
            "sub" => "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n".to_string(), // x - y
            "and" => "@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n".to_string(), // x & y
            "or" => "@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n".to_string(),  // x | y

            // Comandos unários: operam diretamente sobre o topo da pilha
            "neg" => "@SP\nA=M-1\nM=-M\n".to_string(), // -y
            "not" => "@SP\nA=M-1\nM=!M\n".to_string(), // !y

            // Comandos de comparação: precisam de rótulos únicos, tratados à parte
            "eq" | "gt" | "lt" => self.write_comparison(command),

            // para qualquer outro gera um erro de execução
            _ => panic!("Comando aritmético inválido: {}", command),
        }
    }

    /// Traduz os comandos de comparação (eq, gt, lt), que exigem desvio condicional
    fn write_comparison(&mut self, command: &str) -> String {
        let jump = match command { // instrução de salto correspondente ao comando
            "eq" => "JEQ", // == 
            "gt" => "JGT", // > 
            "lt" => "JLT", // < 

            // para qualquer outro comando gera um erro de execução
            _ => unreachable!(),
            // adendo que unreachable nunca estoura exceto quando ocorre um bug
            // pois antes ocorre filtragem do que entra na função,
            // diferente do panic que vem direto da .vm, podendo então
            // existir um erro de escrita no código
        };

        // Rótulos únicos para esta ocorrência do comando 
        // (evita colisão quando o mesmo comando aparece várias vezes no arquivo .vm)
        let true_label = format!("TRUE_{}", self.label_count);
        let end_label = format!("END_{}", self.label_count);
        self.label_count += 1;

        format!(
            "@SP\n\
             AM=M-1\n\
             D=M\n\
             A=A-1\n\
             D=M-D\n\
             @{true_label}\n\
             D;{jump}\n\
             @SP\n\
             A=M-1\n\
             M=0\n\
             @{end_label}\n\
             0;JMP\n\
             ({true_label})\n\
             @SP\n\
             A=M-1\n\
             M=-1\n\
             ({end_label})\n",
            true_label = true_label,
            jump = jump,
            end_label = end_label,
        )
    }

    /// Traduz um comando push ou pop para Assembly
    pub fn write_push_pop(&mut self, command_type: &CommandType, segment: &str, index: i32) -> String {
        match command_type {
            CommandType::Push => self.write_push(segment, index), // push
            CommandType::Pop => self.write_pop(segment, index),   // pop

            // retorna erro de execução se for chamado com um comando aritmético, que não é suportado aqui
            CommandType::Arithmetic => panic!("write_push_pop chamado com um comando aritmético"),
        }
    }

    /// Traduz um comando push, empilhando o valor do segmento/índice indicado
    fn write_push(&self, segment: &str, index: i32) -> String {
        // Primeiro calcula o valor a ser empilhado e o deixa em D
        let load_d = match segment {
            "constant" => format!("@{}\nD=A\n", index), // D = índice (valor literal)
            "local" | "argument" | "this" | "that" => 
                format!("@{pointer}\nD=M\n@{index}\nA=D+A\nD=M\n", // D = *(ponteiro do segmento + índice)
                    pointer = Self::segment_pointer(segment),
                    index = index,
                ),
            "temp" => format!("@5\nD=A\n@{index}\nA=D+A\nD=M\n", index = index), // temp começa em RAM[5]
            "pointer" => format!("@{}\nD=M\n", Self::pointer_target(index)), // pointer 0 = THIS, pointer 1 = THAT
            "static" => format!("@{}.{}\nD=M\n", self.file_name, index), // variável estática Foo.i

            // retorna erro de execução se o segmento for inválido
            _ => panic!("Segmento inválido: {}", segment),
        };

        // Depois empilha o valor de D no topo da pilha e avança o Stack Pointer
        format!("{load_d}@SP\nA=M\nM=D\n@SP\nM=M+1\n", load_d = load_d)
    }

    /// Traduz um comando pop, retirando o topo da pilha e guardando no índice indicado
    fn write_pop(&self, segment: &str, index: i32) -> String {
        match segment {
            // Segmentos com ponteiro base: calcula o endereço de destino, guarda em R13,
            // desempilha o valor em D e finalmente grava em *R13
            "local" | "argument" | "this" | "that" => 
                format!(
                    "@{pointer}\nD=M\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                    pointer = Self::segment_pointer(segment),
                    index = index,
                ),
            // Segmentos com endereço fixo: calcula o endereço de destino, guarda em R13,
            "temp" => format!(
                "@5\nD=A\n@{index}\nD=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n",
                index = index,
            ),
            // Segmentos com endereço fixo: desempilha direto para o destino, sem precisar de R13
            "pointer" => format!(
                "@SP\nAM=M-1\nD=M\n@{}\nM=D\n",
                Self::pointer_target(index),
            ),
            // Segmentos com endereço fixo: desempilha direto para o destino, sem precisar de R13
            "static" => format!("@SP\nAM=M-1\nD=M\n@{}.{}\nM=D\n", self.file_name, index),

            _ => panic!("Segmento inválido: {}", segment),
        }
    }

    /// Retorna o símbolo do ponteiro base de cada segmento (local, argument, this, that)
    fn segment_pointer(segment: &str) -> &'static str {
        match segment {
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => unreachable!(), // não deveria ser chamado com outro segmento, pois isso é filtrado antes
        }
    }

    /// Retorna o símbolo apontado por "pointer" (0 = THIS, 1 = THAT)
    fn pointer_target(index: i32) -> &'static str {
        match index {
            0 => "THIS",
            1 => "THAT",
            _ => panic!("Índice inválido para pointer: {}", index),
        }
    }
}
