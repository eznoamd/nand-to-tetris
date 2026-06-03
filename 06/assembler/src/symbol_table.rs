use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>, // Mapa de símbolos para endereços
    next_var_addr: u16,          // Próximo endereço disponível para variáveis (inicia em 16 por padrão)
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        
        // Símbolos pré-definidos
        table.insert("SP".to_string(),          0); // Stack Pointer
        table.insert("LCL".to_string(),         1); // Base do segmento local
        table.insert("ARG".to_string(),         2); // Base do segmento de argumentos
        table.insert("THIS".to_string(),        3); // Base do segmento THIS
        table.insert("THAT".to_string(),        4); // Base do segmento THAT
        table.insert("SCREEN".to_string(),  16384); // Endereço base da tela
        table.insert("KBD".to_string(),     24576); // Endereço do teclado

        // R0 até R15
        for i in 0..=15 {
            table.insert(format!("R{}", i), i); // Registradores R0 a R15
        }
        // Retorna a estrutura SymbolTable inicializada com os símbolos pré-definidos
        Self {
            table,
            next_var_addr: 16,
        }
    }

    // Adiciona um símbolo à tabela com seu endereço correspondente
    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    // Verifica se um símbolo existe na tabela
    pub fn get_address(&mut self, symbol: &str) -> u16 {
        if let Some(&addr) = self.table.get(symbol) { // Verifica se o símbolo já existe na tabela e retorna seu endereço
            addr
        } else { // Se o símbolo não existir, atribui o próximo endereço disponível para variáveis e adiciona à tabela
            let addr = self.next_var_addr;
            self.table.insert(symbol.to_string(), addr);
            self.next_var_addr += 1;
            addr
        }
    }
}