use crate::parser::CommandType;

/// Responsável por traduzir comandos VM
pub struct CodeWriter {
    file_name: String,  // nome do arquivo .vm
    label_count: u32,   // contador usado para gerar rótulos únicos nas comparações e chamadas de funções
    current_function: String, // função vm atualmente sendo traduzida, usado para criar labels com escopo
                              // Exemplo: Foo.bar$LOOP
    call_count: u32, // contador usado para gerar endereços de retorno unicos
}

impl CodeWriter {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            label_count: 0,
            current_function: String::new(),
            call_count: 0,
        }
    }

    /// Atualiza o nome do arquivo atualmente sendo traduzido.
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    // ========================================================
    // PROJETO 7
    // ========================================================

    /// Traduz um comando aritmético/lógico para Assembly
    pub fn write_arithmetic(&mut self, command: &str) -> String {
        match command {
            // Comandos binários: consomem o topo da pilha (y o primeiro, e x o segundo),
            // deixando o resultado da operação no lugar de x
            "add" => "@SP\nAM=M-1\nD=M\nA=A-1\nM=D+M\n".to_string(), // x + y
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

        return format!(
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
    pub fn write_push_pop(&mut self, command_type: &CommandType, segment: &str, index: i32,) -> String {
        match command_type {
            CommandType::Push => self.write_push(segment, index),
            CommandType::Pop => self.write_pop(segment, index),
            _ => panic!("write_push_pop recebeu um comando inválido"),
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
        return format!("{load_d}@SP\nA=M\nM=D\n@SP\nM=M+1\n", load_d = load_d);
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

    // ========================================================
    // PROJETO 8
    // ========================================================

    /// Traduz: label LOOP
    /// para: (FuncaoAtual$LOOP)
    pub fn write_label(&self, label: &str) -> String {
        let label = self.scoped_label(label);

        return format!("({})\n", label);
    }

    /// Traduz: goto LOOP
    /// para: @FuncaoAtual$LOOP
    ///       0;JMP
    pub fn write_goto(&self, label: &str) -> String {
        let label = self.scoped_label(label);

        return format!("\
            @{}
            0;JMP",
            label
        )
    }

    /// Traduz: if-goto LOOP
    /// para: @SP
    ///       AM=M-1
    ///       D=M
    ///       @FuncaoAtual$LOOP
    ///       D;JNE
    pub fn write_if(&self, label: &str) -> String {
        let label = self.scoped_label(label);

        return format!("\
            @SP
            AM=M-1
            D=M
            @{}
            D;JNE",
        label)
    }

    /// Adiciona o escopo da função ao label.
    /// Exemplo: function Main.main 0
    ///          label LOOP
    ///
    /// vira:    (Main.main$LOOP)
    fn scoped_label(&self, label: &str) -> String {
        if self.current_function.is_empty() {
            return label.to_string()
        } else {
            return format!("{}${}", self.current_function, label)
        }
    }

    /// Traduz: function Foo.bar 2
    ///
    /// para: (Foo.bar)
    ///       push constant 0
    ///       push constant 0 ... 
    pub fn write_function(&mut self, name: &str, n_vars: i32) -> String {
        self.current_function = name.to_string();

        let mut output = format!("({})\n", name);

        // Cada variável local começa com 0.
        for _ in 0..n_vars {
            output.push_str(
                "@0\n\
                 D=A\n\
                 @SP\n\
                 A=M\n\
                 M=D\n\
                 @SP\n\
                 M=M+1\n",
            );
        }

        return output
    }

    /// Traduz: call Foo.bar 2
    ///
    /// para:
    ///
    /// [1] endereço de retorno
    /// [2] LCL
    /// [3] ARG
    /// [4] THIS
    /// [5] THAT
    /// [6] ARG = SP - 5 - nArgs
    /// [7] LCL = SP
    /// [8] goto Foo.bar
    pub fn write_call(&mut self, function_name: &str, n_args: i32) -> String {
        let return_label = format!("RETURN_{}", self.call_count);
        self.call_count += 1;

        let mut output = String::new();

        // push return-address
        output.push_str(&format!("\
            @{}
            D=A
            @SP
            A=M
            M=D
            @SP
            M=M+1",
            return_label
        ));

        // push LCL
        output.push_str("\
            @LCL
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
        );

        // push ARG
        output.push_str("\
            @ARG
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
        );

        // push THIS
        output.push_str("\
            @THIS
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
        );

        // push THAT
        output.push_str("\
            @THAT
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
        );

        // ARG = SP - 5 - nArgs
        output.push_str(&format!("\
            @SP
            D=M
            @5
            D=D-A
            @{}
            D=D-A
            @ARG
            M=D\n", n_args));

        // LCL = SP
        output.push_str("\
            @SP
            D=M
            @LCL
            M=D\n");

        // goto function
        output.push_str(&format!("\
            @{}
            0;JMP
            ({})
            \n", 
            function_name, 
            return_label
        ));

        return output
    }

    /// Traduz o comando return.
    ///
    /// R13 = LCL           |(FRAME)
    /// R14 = *(FRAME - 5)  |(RET)
    ///
    /// *ARG = pop()
    /// SP = ARG + 1
    /// THAT = *(FRAME - 1)
    /// THIS = *(FRAME - 2)
    /// ARG  = *(FRAME - 3)
    /// LCL  = *(FRAME - 4)
    /// goto RET
    pub fn write_return(&self) -> String {
        return "\
            @LCL
            D=M
            @R13
            M=D
            @5
            A=D-A
            D=M
            @R14
            M=D
            @SP
            AM=M-1
            D=M
            @ARG
            A=M
            M=D
            @ARG
            D=M+1
            @SP
            M=D
            @R13
            AM=M-1
            D=M
            @THAT
            M=D
            @R13
            AM=M-1
            D=M
            @THIS
            M=D
            @R13
            AM=M-1
            D=M
            @ARG
            M=D
            @R13
            AM=M-1
            D=M
            @LCL
            M=D
            @R14
            A=M
            0;JMP"
        .to_string()
    }

    /// Código inicial do programa:
    ///
    /// SP = 256
    /// call Sys.init 0
    pub fn write_init(&mut self) -> String {
        let mut output = String::new();

        // SP = 256
        output.push_str("\
            @256
            D=A
            @SP
            M=D\n");

        // call Sys.init 0
        output.push_str(
            &self.write_call("Sys.init", 0)
        );

        return output
    }

    // FUNÇÕES AUXILIARES

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
