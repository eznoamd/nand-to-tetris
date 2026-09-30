# Jack — Guia Completo da Linguagem e Jack OS

> Referência prática para programação em Jack no Nand2Tetris.
>
> Jack é a linguagem de alto nível usada na segunda metade do Nand2Tetris. Ela foi projetada para ser pequena o suficiente para que seu compilador possa ser construído no Projeto 10/11, mas completa o bastante para escrever aplicações, jogos e a própria Jack OS.

---

## Sumário

1. [Visão geral](#1-visão-geral)
2. [Estrutura de um programa](#2-estrutura-de-um-programa)
3. [Comentários](#3-comentários)
4. [Tipos](#4-tipos)
5. [Variáveis](#5-variáveis)
6. [Constantes](#6-constantes)
7. [Operadores](#7-operadores)
8. [Expressões](#8-expressões)
9. [Statements](#9-statements)
10. [Classes](#10-classes)
11. [Subrotinas](#11-subrotinas)
12. [Objetos e construtores](#12-objetos-e-construtores)
13. [Arrays](#13-arrays)
14. [Strings](#14-strings)
15. [Escopo e resolução de nomes](#15-escopo-e-resolução-de-nomes)
16. [Estrutura de múltiplos arquivos](#16-estrutura-de-múltiplos-arquivos)
17. [Jack OS](#17-jack-os)
18. [Math](#18-math)
19. [String API](#19-string-api)
20. [Array API](#20-array-api)
21. [Output API](#21-output-api)
22. [Screen API](#22-screen-api)
23. [Keyboard API](#23-keyboard-api)
24. [Memory API](#24-memory-api)
25. [Sys API](#25-sys-api)
26. [Exemplo completo](#26-exemplo-completo)
27. [Game loop](#27-game-loop)
28. [Erros e armadilhas comuns](#28-erros-e-armadilhas-comuns)
29. [Relação entre Jack e VM](#29-relação-entre-jack-e-vm)
30. [Estrutura recomendada para projetos](#30-estrutura-recomendada-para-projetos)
31. [Cheat sheet](#31-cheat-sheet)

---

# 1. Visão geral

Jack é uma linguagem pequena, orientada a objetos e criada especificamente para o Nand2Tetris.


Um projeto típico pode ter:

```text
Game/
├── Main.jack
├── Player.jack
├── Enemy.jack
└── Map.jack
```

Cada classe normalmente fica em seu próprio arquivo `.jack`.

---

# 2. Estrutura de um programa

A unidade básica da linguagem é a classe:

```jack
class Main {

    function void main() {
        return;
    }

}
```

Uma classe pode conter:

```text
static variables
field variables

constructors
functions
methods
```

Exemplo:

```jack
class Player {

    field int x;
    field int y;
    field int health;

    constructor Player new(int startX, int startY) {
        let x = startX;
        let y = startY;
        let health = 100;
        return this;
    }

    method void move(int dx, int dy) {
        let x = x + dx;
        let y = y + dy;
        return;
    }

}
```

---

# 3. Comentários

## Uma linha

```jack
// comentário
```

## Depois de código

```jack
let x = x + 1; // incrementa x
```

## Múltiplas linhas

```jack
/*
    comentário
    com várias linhas
*/
```

---

# 4. Tipos

Jack possui três tipos primitivos:

```text
int
char
boolean
```

Também existe:

```text
void
```

para subrotinas sem valor de retorno.

E o nome de uma classe pode ser usado como tipo:

```jack
Player player;
Enemy enemy;
String name;
Array values;
```

## `int`

Inteiro de 16 bits:

```jack
var int x;

let x = 42;
```

Constantes inteiras escritas diretamente no código são não-negativas e vão de `0` a `32767`.

Valores negativos são expressões:

```jack
let x = -10;
```

## `boolean`

```jack
var boolean alive;

let alive = true;
let alive = false;
```

## `char`

```jack
var char c;

let c = "A";
```

## `void`

Usado para funções/métodos que não retornam valor:

```jack
function void hello() {
    do Output.printString("Hello");
    return;
}
```

---

# 5. Variáveis

Existem quatro categorias:

```text
static
field
var
parameter
```

## `static`

Pertence à classe, não a uma instância específica:

```jack
class Counter {

    static int total;

}
```

## `field`

Representa estado de cada objeto:

```jack
class Player {

    field int x;
    field int y;
    field int health;

}
```

Dois objetos possuem campos independentes:

```text
Player A                Player B
---------               ---------
x = 10                  x = 200
y = 20                  y = 100
health = 100            health = 50
```

## `var`

Variável local de uma subrotina:

```jack
function void main() {
    var int x;
    var int y;

    let x = 10;
    let y = 20;

    return;
}
```

**Todas as declarações `var` devem aparecer antes dos statements da subrotina.**

Isto é inválido:

```jack
function void main() {
    let x = 10;
    var int y; // errado
}
```

Faça:

```jack
function void main() {
    var int y;

    let x = 10;
    let y = 20;

    return;
}
```

## Parâmetros

```jack
function int add(int a, int b) {
    return a + b;
}
```

`a` e `b` são parâmetros.

---

# 6. Constantes

## Inteiro

```jack
let x = 123;
```

## Booleanos

```jack
true
false
```

## Nulo

```jack
null
```

## `this`

Representa o objeto atual dentro de um método/construtor:

```jack
return this;
```

## String literal

```jack
"Hello world"
```

Strings são objetos da classe `String`; elas não são um tipo primitivo.

---

# 7. Operadores

Jack possui:

```text
+     soma
-     subtração
*     multiplicação
/     divisão
&     AND bit a bit
|     OR bit a bit
<     menor que
>     maior que
=     igualdade
~     NOT
```

Operadores unários:

```text
- x
~ x
```

Exemplos:

```jack
let x = a + b;
let x = a - b;
let x = a * b;
let x = a / b;

let x = a & b;
let x = a | b;

let ok = x < 10;
let ok = x > 10;
let ok = x = 10;

let x = ~x;
let x = -x;
```

## Importante: precedência

Jack não possui a precedência matemática convencional entre os operadores binários.

Não assuma que:

```jack
1 + 2 * 3
```

significa:

```text
1 + (2 * 3)
```

Use parênteses explicitamente:

```jack
let x = 1 + (2 * 3);
```

Isso evita ambiguidades e deixa a intenção clara.

---

# 8. Expressões

Uma expressão pode ser:

```jack
x
123
true
false
null
this
x + y
x * 10
(x + y)
foo()
object.method()
array[i]
```

Exemplos:

```jack
let x = a + b;
let y = Math.abs(x);
let z = array[i];
let result = player.getHealth();
```

---

# 9. Statements

Jack possui cinco statements principais:

```text
let
if
while
do
return
```

## `let`

Atribuição:

```jack
let x = 10;
let x = x + 1;
```

Array:

```jack
let numbers[0] = 42;
```

## `if`

```jack
if (x > 10) {
    do Output.printString("maior");
}
```

Com `else`:

```jack
if (x > 10) {
    do Output.printString("maior");
}
else {
    do Output.printString("menor ou igual");
}
```

## `while`

```jack
while (x < 10) {
    let x = x + 1;
}
```

## `do`

Usado para chamar uma subrotina quando o valor retornado não será usado:

```jack
do Output.printInt(42);
do player.move(10, 0);
```

Uma chamada com retorno pode ser usada em expressão:

```jack
let x = Math.abs(-10);
```

## `return`

Sem valor:

```jack
return;
```

Com valor:

```jack
return x + y;
```

Uma subrotina `void` deve terminar com `return;`.

---

# 10. Classes

Uma classe agrupa estado e comportamento:

```jack
class Player {

    field int x;
    field int y;

    method void move(int dx, int dy) {
        let x = x + dx;
        let y = y + dy;
        return;
    }

}
```

Jack não possui:

```text
import
inheritance
interface
generics
packages
```

Não existe `import Player;`.

Se `Player.jack` estiver na mesma aplicação, basta usar:

```jack
var Player player;
let player = Player.new(10, 20);
```

---

# 11. Subrotinas

Existem três tipos:

```text
constructor
function
method
```

## Function

Não pertence a uma instância:

```jack
function int add(int a, int b) {
    return a + b;
}
```

Chamada:

```jack
let result = MathUtils.add(10, 20);
```

ou, dentro da mesma classe:

```jack
let result = add(10, 20);
```

## Method

Pertence a um objeto:

```jack
method void move(int dx, int dy) {
    let x = x + dx;
    let y = y + dy;
    return;
}
```

Chamada:

```jack
do player.move(10, 0);
```

Dentro de um método, `this` representa o objeto atual.

## Constructor

Cria uma nova instância:

```jack
constructor Player new(int x, int y) {
    let this.x = x; // NÃO use esta sintaxe em Jack
    let this.y = y; // NÃO use esta sintaxe em Jack
    return this;
}
```

A forma correta é:

```jack
constructor Player new(int nx, int ny) {
    let x = nx;
    let y = ny;
    return this;
}
```

O construtor deve alocar a memória do objeto e retornar `this`.

---

# 12. Objetos e construtores

Considere:

```jack
class Player {

    field int x;
    field int y;

    constructor Player new(int startX, int startY) {
        let x = startX;
        let y = startY;
        return this;
    }

}
```

Criação:

```jack
var Player player;

let player = Player.new(100, 50);
```

Conceitualmente:

```text
Player object
┌──────────────┐
│ x = 100      │
│ y = 50       │
└──────────────┘
```

Métodos:

```jack
do player.move(10, 0);
```

A referência ao objeto é usada como o `this` do método.

---

# 13. Arrays

Criar:

```jack
var Array values;

let values = Array.new(10);
```

Acessar:

```jack
let values[0] = 10;
let values[1] = 20;

let x = values[0];
```

Índices:

```text
Array.new(10)

0 1 2 3 4 5 6 7 8 9
```

A classe `Array` é essencialmente uma interface conveniente para memória alocada dinamicamente.

Liberar:

```jack
do values.dispose();
```

---

# 14. Strings

`String` é uma classe da Jack OS.

Declaração:

```jack
var String s;
```

Criação explícita:

```jack
let s = String.new(20);
```

Literal:

```jack
let s = "Hello";
```

Acesso:

```jack
let c = s.charAt(0);
```

Adicionar caractere:

```jack
let s = s.appendChar(33);
```

`33` corresponde a `!`.

---

# 15. Escopo e resolução de nomes

A prioridade prática é:

```text
variável local / parâmetro
        ↓
field
        ↓
static
```

Evite nomes iguais entre parâmetros e fields.

Prefira:

```jack
field int x;

method void setX(int newX) {
    let x = newX;
    return;
}
```

em vez de:

```jack
field int x;

method void setX(int x) {
    let x = x;
    return;
}
```

---

# 16. Estrutura de múltiplos arquivos

Uma aplicação deve ser organizada em arquivos separados:

```text
Game/
├── Main.jack
├── Game.jack
├── Player.jack
├── Enemy.jack
├── Map.jack
└── Renderer.jack
```

Não coloque várias classes em um único `.jack` esperando que isso funcione como um arquivo de módulos.

Compile a aplicação/pasta contendo as classes.

O compilador gera:

```text
Main.vm
Game.vm
Player.vm
Enemy.vm
Map.vm
Renderer.vm
```

Não existe `import`.

---

# 17. Jack OS

A Jack OS fornece funcionalidades que não fazem parte da linguagem.

As classes são:

```text
Math
String
Array
Output
Screen
Keyboard
Memory
Sys
```

A separação é importante:

```text
Jack Language
    │
    ├── classes
    ├── objects
    ├── expressions
    ├── statements
    └── subroutines
          │
          ▼
       Jack OS
          │
          ├── Math
          ├── String
          ├── Array
          ├── Output
          ├── Screen
          ├── Keyboard
          ├── Memory
          └── Sys
```

---

# 18. Math

```jack
Math.abs(x)
Math.multiply(x, y)
Math.divide(x, y)
Math.min(x, y)
Math.max(x, y)
Math.sqrt(x)
```

Exemplos:

```jack
let x = Math.abs(-50);
let x = Math.multiply(6, 7);
let x = Math.divide(20, 3);
let x = Math.min(a, b);
let x = Math.max(a, b);
let x = Math.sqrt(25);
```

`x * y` e `x / y` são operações suportadas pela infraestrutura da Jack OS.

---

# 19. String API

## Criar

```jack
let s = String.new(20);
```

## `length`

```jack
let n = s.length();
```

## `charAt`

```jack
let c = s.charAt(0);
```

## `setCharAt`

```jack
do s.setCharAt(0, 65);
```

## `appendChar`

```jack
let s = s.appendChar(33);
```

## `eraseLastChar`

```jack
do s.eraseLastChar();
```

## `intValue`

Converte o conteúdo da string para inteiro:

```jack
let n = s.intValue();
```

## `setInt`

```jack
do s.setInt(123);
```

## Caracteres especiais

```jack
String.backSpace()
String.doubleQuote()
String.newLine()
```

## Liberar

```jack
do s.dispose();
```

---

# 20. Array API

```jack
Array.new(size)
array.dispose()
```

Exemplo:

```jack
var Array data;

let data = Array.new(100);

let data[0] = 123;

do data.dispose();
```

---

# 21. Output API

A API textual é:

```jack
Output.moveCursor(i, j)
Output.printChar(c)
Output.printString(s)
Output.printInt(i)
Output.println()
Output.backSpace()
```

Exemplos:

```jack
do Output.printString("Hello");
do Output.printInt(42);
do Output.printChar(65);
do Output.println();
```

`moveCursor`:

```jack
do Output.moveCursor(5, 10);
```

---

# 22. Screen API

A tela Hack possui:

```text
512 × 256 pixels
```

Coordenadas:

```text
(0, 0) ───────────────► x
  │
  │
  │
  ▼
  y
```

API:

```jack
Screen.clearScreen()
Screen.setColor(boolean)
Screen.drawPixel(x, y)
Screen.drawLine(x1, y1, x2, y2)
Screen.drawRectangle(x1, y1, x2, y2)
Screen.drawCircle(x, y, r)
```

## Limpar

```jack
do Screen.clearScreen();
```

## Cor

```jack
do Screen.setColor(true);
```

No Hack:

```text
true  → preto
false → branco
```

## Pixel

```jack
do Screen.drawPixel(100, 50);
```

## Linha

```jack
do Screen.drawLine(10, 10, 100, 100);
```

## Retângulo

```jack
do Screen.drawRectangle(50, 50, 150, 100);
```

## Círculo

```jack
do Screen.drawCircle(200, 100, 50);
```

---

# 23. Keyboard API

```jack
Keyboard.keyPressed()
Keyboard.readChar()
Keyboard.readLine(message)
Keyboard.readInt(message)
```

## `keyPressed`

Não bloqueia esperando entrada:

```jack
var char key;

let key = Keyboard.keyPressed();
```

É apropriado para game loops.

## `readChar`

Espera uma tecla:

```jack
var char c;

let c = Keyboard.readChar();
```

## `readLine`

```jack
var String name;

let name = Keyboard.readLine("Nome: ");
```

## `readInt`

```jack
var int energy;

let energy = Keyboard.readInt("Energia: ");
```

---

# 24. Memory API

A API de memória é:

```jack
Memory.peek(address)
Memory.poke(address, value)
Memory.alloc(size)
Memory.deAlloc(object)
```

## `peek`

Ler RAM:

```jack
let value = Memory.peek(1000);
```

Conceitualmente:

```text
value = RAM[1000]
```

## `poke`

Escrever RAM:

```jack
do Memory.poke(1000, 123);
```

Conceitualmente:

```text
RAM[1000] = 123
```

## `alloc`

Alocar memória:

```jack
var Array buffer;

let buffer = Memory.alloc(100);
```

## `deAlloc`

Liberar memória:

```jack
do Memory.deAlloc(buffer);
```

Normalmente, para objetos de `Array`, use:

```jack
do buffer.dispose();
```

---

# 25. Sys API

```jack
Sys.halt()
Sys.error(errorCode)
Sys.wait(duration)
```

## `halt`

```jack
do Sys.halt();
```

Interrompe o programa.

## `wait`

```jack
do Sys.wait(1000);
```

Espera aproximadamente 1000 ms.

## `error`

```jack
do Sys.error(5);
```

Gera um erro e interrompe a execução.

---

# 26. Exemplo completo

## `Player.jack`

```jack
class Player {

    field int x;
    field int y;
    field int health;

    constructor Player new(int startX, int startY) {
        let x = startX;
        let y = startY;
        let health = 100;

        return this;
    }

    method void move(int dx, int dy) {
        let x = x + dx;
        let y = y + dy;

        return;
    }

    method void damage(int amount) {
        let health = health - amount;

        return;
    }

    method int getX() {
        return x;
    }

    method int getY() {
        return y;
    }

    method int getHealth() {
        return health;
    }

}
```

## `Main.jack`

```jack
class Main {

    function void main() {
        var Player player;

        let player = Player.new(100, 100);

        do player.move(10, 0);
        do player.damage(25);

        do Output.printString("X: ");
        do Output.printInt(player.getX());
        do Output.println();

        do Output.printString("Y: ");
        do Output.printInt(player.getY());
        do Output.println();

        do Output.printString("Health: ");
        do Output.printInt(player.getHealth());
        do Output.println();

        return;
    }

}
```

Saída:

```text
X: 110
Y: 100
Health: 75
```

---

# 27. Game loop

Para jogos, a estrutura mais importante é o game loop:

```jack
while (true) {

    // entrada
    // atualização
    // renderização

}
```

Exemplo:

```jack
class Game {

    field Player player;

    constructor Game new() {
        let player = Player.new(100, 100);
        return this;
    }

    method void run() {

        while (true) {
            do update();
            do render();
        }

        return;
    }

    method void update() {
        var char key;

        let key = Keyboard.keyPressed();

        if (key = 130) {
            do player.move(-1, 0);
        }

        if (key = 132) {
            do player.move(1, 0);
        }

        return;
    }

    method void render() {
        do Screen.clearScreen();
        return;
    }

}
```

Uma arquitetura melhor:

```text
Game loop
   │
   ├── Input
   │
   ├── Update
   │    ├── Player
   │    ├── Enemy
   │    └── World
   │
   └── Render
        └── Screen
```

Para um raycaster:

```text
Input
  ↓
Player
  ↓
World / Map
  ↓
Raycaster
  ↓
Renderer
  ↓
Screen
```

---

# 28. Erros e armadilhas comuns

## Declarar `var` depois de statements

Errado:

```jack
let x = 10;
var int y;
```

Correto:

```jack
var int y;

let x = 10;
```

---

## Esquecer `do`

Errado:

```jack
Output.printInt(x);
```

Correto:

```jack
do Output.printInt(x);
```

---

## Esquecer `return`

Errado:

```jack
method void move() {
    let x = x + 1;
}
```

Correto:

```jack
method void move() {
    let x = x + 1;
    return;
}
```

---

## Usar `this.x`

Não use a sintaxe:

```jack
let this.x = x;
```

Em Jack, dentro de um método/construtor:

```jack
let x = newX;
```

acessa o field correspondente.

---

## Confundir parâmetro com field

Evite:

```jack
field int x;

method void setX(int x) {
    let x = x;
    return;
}
```

Prefira:

```jack
field int x;

method void setX(int newX) {
    let x = newX;
    return;
}
```

---

## Criar arrays desnecessariamente

Evite:

```jack
method Array getPosition() {
    var Array p;

    let p = Array.new(2);
    let p[0] = x;
    let p[1] = y;

    return p;
}
```

se isso for chamado muitas vezes por segundo.

Prefira:

```jack
method int getX() {
    return x;
}

method int getY() {
    return y;
}
```

Isso evita alocações repetidas.

---

## Esperar precedência matemática

Não dependa de:

```jack
1 + 2 * 3
```

Use:

```jack
1 + (2 * 3)
```

---

## Confundir `function` e `method`

Function:

```jack
function void foo() {
    return;
}
```

Method:

```jack
method void foo() {
    return;
}
```

Um método pertence a um objeto e possui `this`.

---

## Esquecer de inicializar objetos

Isto:

```jack
var Player player;

do player.move(10, 0);
```

não cria um `Player`.

Você precisa:

```jack
let player = Player.new(100, 100);
```

antes de usar o objeto.

---

## Esquecer que String é objeto

Isto:

```jack
var String s;
```

apenas declara a referência.

Uma String pode ser criada explicitamente:

```jack
let s = String.new(20);
```

ou obtida por um literal:

```jack
let s = "Hello";
```

---

# 29. Relação entre Jack e VM

A linguagem Jack foi desenhada para ser traduzida para a VM do Nand2Tetris.

Uma correspondência importante é:

| Jack | VM |
|---|---|
| `static` | `static` |
| `field` | `this` |
| `var` | `local` |
| parâmetro | `argument` |

Por exemplo:

```jack
field int x;
```

corresponde conceitualmente a uma posição no segmento:

```text
this
```

Um método:

```jack
method void move(int dx) {
    let x = x + dx;
    return;
}
```

precisa tratar o objeto como seu `this`.

Uma chamada:

```jack
do player.move(10);
```

envolve conceitualmente:

```text
push referência_do_player
push 10
call Player.move 2
```

A implementação exata é responsabilidade do compilador.

---

# 30. Estrutura recomendada para projetos

Para uma aplicação simples:

```text
Project/
├── Main.jack
└── Player.jack
```

Para um jogo:

```text
WolfJack/
├── Main.jack
├── Game.jack
├── Player.jack
├── Enemy.jack
├── Map.jack
├── Raycaster.jack
├── Renderer.jack
├── Input.jack
└── MathUtils.jack
```

Uma possível responsabilidade:

```text
Main
 └── inicia o jogo

Game
 └── game loop

Player
 └── jogador, posição, vida, movimento

Enemy
 └── inimigos

Map
 └── mapa do mundo

Raycaster
 └── cálculo dos raios

Renderer
 └── desenho

Input
 └── teclado

MathUtils
 └── funções matemáticas específicas
```

Não existe `import`; as classes da mesma aplicação podem ser referenciadas diretamente pelo nome.

---

# 31. Cheat sheet

## Estrutura mínima

```jack
class Main {

    function void main() {

        return;
    }

}
```

## Variáveis

```jack
static int x;
field int y;

function void foo() {
    var int z;
    return;
}
```

## Condição

```jack
if (x > 10) {
    // ...
}
else {
    // ...
}
```

## Loop

```jack
while (x < 10) {
    let x = x + 1;
}
```

## Objeto

```jack
var Player player;

let player = Player.new(100, 100);

do player.move(10, 0);
```

## Array

```jack
var Array array;

let array = Array.new(10);
let array[0] = 42;
let x = array[0];

do array.dispose();
```

## String

```jack
var String s;

let s = "Hello";
let s = s.appendChar(33);

do Output.printString(s);
```

## Entrada

```jack
let key = Keyboard.keyPressed();

let c = Keyboard.readChar();

let text = Keyboard.readLine("Texto: ");

let number = Keyboard.readInt("Número: ");
```

## Texto

```jack
do Output.printString("Hello");
do Output.printInt(42);
do Output.printChar(65);
do Output.println();
```

## Gráficos

```jack
do Screen.clearScreen();
do Screen.setColor(true);

do Screen.drawPixel(x, y);
do Screen.drawLine(x1, y1, x2, y2);
do Screen.drawRectangle(x1, y1, x2, y2);
do Screen.drawCircle(x, y, radius);
```

## Matemática

```jack
let x = Math.abs(x);
let x = Math.multiply(a, b);
let x = Math.divide(a, b);
let x = Math.min(a, b);
let x = Math.max(a, b);
let x = Math.sqrt(x);
```

## Memória

```jack
let x = Memory.peek(address);

do Memory.poke(address, value);

let buffer = Memory.alloc(size);

do Memory.deAlloc(buffer);
```

## Sistema

```jack
do Sys.wait(100);
do Sys.halt();
do Sys.error(1);
```

---

# Referência rápida da Jack OS

```text
Math
├── abs
├── multiply
├── divide
├── min
├── max
└── sqrt

String
├── new
├── dispose
├── length
├── charAt
├── setCharAt
├── appendChar
├── eraseLastChar
├── intValue
├── setInt
├── backSpace
├── doubleQuote
└── newLine

Array
├── new
└── dispose

Output
├── moveCursor
├── printChar
├── printString
├── printInt
├── println
└── backSpace

Screen
├── clearScreen
├── setColor
├── drawPixel
├── drawLine
├── drawRectangle
└── drawCircle

Keyboard
├── keyPressed
├── readChar
├── readLine
└── readInt

Memory
├── peek
├── poke
├── alloc
└── deAlloc

Sys
├── halt
├── error
└── wait
```

---

## Referências oficiais

- Nand2Tetris — Projeto 9: High-Level Programming
- Nand2Tetris — Projeto 10: Compiler I
- Nand2Tetris — Projeto 11: Compiler II
- Nand2Tetris — Projeto 12: Operating System
- Nand2Tetris — Software / ferramentas oficiais
