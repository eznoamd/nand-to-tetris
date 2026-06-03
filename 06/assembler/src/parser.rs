/// Limpa a linha de código
pub fn clean_line(line: &str) -> String {
    let no_comments = line.split("//").next().unwrap_or("");         // retorna a string antes de "//" ou "" se for none 
    no_comments.replace(" ", "").replace("\t", "").replace("\r", "") // remove espaços, tabs e retornos de carro
}

/// Quebra a instrução C em (dest, comp, jump)
pub fn parse_c_command(command: &str) -> (&str, &str, &str) {
    // Variaveis mutaveis para armazenar as partes da instrução
    let mut dest = "";
    let mut comp = command;
    let mut jump = "";

    if let Some(idx) = comp.find('=') { // se encontrar o caractere '=' na string comp, idx = índice da primeira ocorrência
        dest = &comp[..idx];            // dest é o intervalo de 0 até idx 
        comp = &comp[idx + 1..];        // comp é o intervalo de idx + 1 até o final da string
    }
    if let Some(idx) = comp.find(';') { // se encontrar o caractere ';' na string comp, idx = índice da primeira ocorrência
        jump = &comp[idx + 1..];        // jump é o intervalo de idx + 1 até o final da string
        comp = &comp[..idx];            // comp é o intervalo de 0 até idx
    }

    (dest, comp, jump) // retorna todos os valores 
}