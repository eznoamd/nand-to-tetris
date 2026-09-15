# Nand2Tetris

Implementação completa do curso **Nand2Tetris** (*From NAND to Tetris*): um computador construído em camadas, partindo de uma porta NAND e chegando a um sistema capaz de compilar e rodar programas de alto nível.

O repositório é organizado **por camada do sistema**, não por número de capítulo do curso — cada pasta na raiz representa uma peça funcional da máquina, na ordem em que uma constrói em cima da outra.

## Camadas

### `hardware/` — Parte I: lógica e arquitetura (capítulos 1-5)

Chips descritos em HDL. Cada subpasta de capítulo guarda os `.hdl`/`.asm` na raiz e os testes (`.tst`) e saídas esperadas (`.cmp`) em uma subpasta `hdl-tst/`, no formato exigido pelo Hardware Simulator do curso — por exemplo, `hardware/01/And.hdl` é testado por `hardware/01/hdl-tst/And.tst`.

| Pasta | Conteúdo |
|---|---|
| `hardware/01` | Portas lógicas básicas (And, Or, Not, Xor, Mux, DMux e variantes) |
| `hardware/02` | Aritmética combinacional (Half/FullAdder, Add16, ALU) |
| `hardware/03` | Lógica sequencial (Bit, Register, RAM8 até RAM16K, PC) |
| `hardware/04` | Linguagem de máquina Hack (programas `Mult` e `Fill` em Assembly) |
| `hardware/05` | Arquitetura do computador (CPU, Memory, Computer) |

**Como testar:** abrir o `.tst` correspondente (dentro de `hdl-tst/`) no Hardware Simulator ou CPU Emulator oficiais — ele já carrega o `.hdl`/`.asm` da pasta pai automaticamente.

### `assembler/` — Parte II: tradutor Assembly → binário (capítulo 6)

Montador Hack escrito em Rust, que traduz `.asm` em `.hack`.

- `assembler/src` — código-fonte (Cargo/Rust)
- `assembler/tests` — programas de exemplo (`Add`, `Max`, `Pong`, `Rect`, incluindo as versões com símbolos `L`) usados para validar a saída do montador

**Como testar:** `cd assembler && cargo run -- tests/Pong.asm` e comparar o `.hack` gerado com o de referência.

### `vm-translator/` — Parte II: máquina virtual (capítulos 7-8)

Tradutor de bytecode VM → Assembly Hack, escrito em Rust. Os capítulos 7 e 8 do curso são a mesma ferramenta evoluindo em dois estágios, por isso vivem no mesmo projeto em vez de pastas separadas:

- `vm-translator/src` — código-fonte (parser + code writer)
- `vm-translator/tests/vm1-stack-and-memory` — estágio 1: operações aritméticas/lógicas e acesso à memória (`BasicTest`, `PointerTest`, `SimpleAdd`, `StackTest`, `StaticTest`)
- `vm-translator/tests/vm2-branching-and-functions` — estágio 2: branching e chamadas de função (`BasicLoop`, `FibonacciElement`, `FibonacciSeries`, `NestedCall`, `SimpleFunction`, `StaticsTest`)

**Como testar:** `cd vm-translator && cargo run -- tests/vm2-branching-and-functions/NestedCall` e validar o `.asm` gerado no CPU Emulator com o `.tst`/`.cmp` da pasta.

### `jack-compiler/` — Parte II: linguagem e compilador Jack (capítulos 9-11) — *planejado*

Vai seguir o mesmo princípio do `vm-translator`: um único projeto que evolui em estágios.

- `samples/` — programas `.jack` de exemplo (capítulo 9, sem ferramenta própria)
- `src/` — tokenizer + parser (capítulo 10, saída XML) evoluindo para geração de código VM completo (capítulo 11)
- `tests/` — programas de teste oficiais (Square, ArrayTest, ExpressionLessSquare, etc.)

### `jack-os/` — Parte II: sistema operacional (capítulo 12) — *planejado*

Classes da biblioteca padrão (`Math`, `String`, `Array`, `Output`, `Screen`, `Keyboard`, `Memory`, `Sys`) implementadas em Jack, compiladas com o `jack-compiler` e testadas no CPU Emulator.