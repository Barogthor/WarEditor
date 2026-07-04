# war3map.j - The JASS2 Script

This is the main map script file. It's a text file that can be opened with any text editor (like notepad). Sometimes it's renamed to `Scripts\war3map.j` by map protectors to hide it from users.

## Overview

The language used is called **JASS2** and was developed by Blizzard. It's a **case-sensitive** language.

### Execution Flow

1. **Map Selection**: When selecting a map in game creation, Warcraft III looks for the `config` function and executes it to set up player slots
2. **Game Start**: When the game starts, Warcraft III looks for the `main` function and executes it

## Language Keywords

| Keyword | Description |
|---------|-------------|
| `function` | Define a new function |
| `takes` | Define the number of arguments for a function |
| `returns` | Set the type of value returned by a function |
| `return` | Make a function exit and return a value |
| `endfunction` | End a function definition |
| `call` | Call a function that returns nothing |
| `globals` | Define the list of global variables |
| `endglobals` | End the global variables definition |
| `local` | Define a local variable |
| `set` | Assign a value to a variable |
| `if`, `elseif`, `else`, `then`, `endif` | Conditional statements (Basic-style) |
| `loop`, `exitwhen`, `endloop` | Loop constructs |
| `constant` | Define a constant |
| `type` | Define a new type/class |
| `extends` | Specify parent type inheritance |
| `native` | Define function header for external built-in function (implemented in Game.DLL) |
| `array` | Define array variables |

## Operators

| Operator | Description |
|----------|-------------|
| `( )` | Parentheses for priorities |
| `+` | Addition (concatenation for strings) |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `=` | Assignment |
| `==`, `<`, `<=`, `>`, `>=`, `!=` | Comparison operators |
| `not` | Invert a boolean value |
| `and` | Boolean AND (both must be true) |
| `or` | Boolean OR (one must be true) |
| `[]` | Array brackets |

## Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `boolean` | Can be true or false | `local boolean B = true` |
| `integer` | Whole numbers | `local integer I = 0` |
| `real` | Floating point numbers | `local real R = 0.25` |
| `string` | Text strings, or null | `local string S = "hello world"` |
| `code` | Pointers to functions, or null | `local code C = function main` |
| `handle` | Pointers to objects, or null | `local handle H = null` |

## Type System

Those six types are the only natives types. All other types are derived from `handle` using the `extends` keyword.

- **Complete type definitions or functions**: Found in `Scripts\Common.j` in War3.mpq
- **GUI trigger functions**: Available in `Scripts\Blizzard.j`
- **AI functions**: Available in `Scripts\Common.ai` and `Scripts\Common.j` for AI scripts

### Handle Type Values

The handle type can be assigned several constant values:
- `null` (generic null value)
- Any integer
- Any float  
- Any string or trigger string
- `true` and `false`

## Example Function

```jass
function myfunction takes nothing returns integer
  local string str = "blah blah blah"
  local integer i
  // comments line
  set i = 0
  loop
    set i = i + 1
    if (i == 27) then
      call DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 60, str)
    endif
    exitwhen i == 30
  endloop
  return i
endfunction
```

## Advanced Features

### Type Casting (Return Bug)

All types have the same length (4 bytes), enabling "type casting" through syntax checker exploitation. Only the last return statement needs to conform to the declared return type:

```jass
function Int2Handle takes integer I returns handle
  return I
  return null
endfunction

function Handle2Int takes handle H returns integer
  return H
  return 0
endfunction
```

### Memory Layout

- **Value types** (boolean, integer, real): 4 bytes hold the actual value
- **Reference types** (string, code, handle, derived types): 4-byte pointers
  - Strings: Pointers to internal string array
  - Code/handles: Pointers to functions and objects in memory

### Game Cache Usage

All variables can be stored in game cache using `StoreInteger` function. Originally for inter-map data transfer, now mainly used as:
- Hash function for direct memory access
- Communication bridge between map script and AI files

## Integer Representations

Three ways to write integers:

1. **Ordinary**: Standard decimal notation (`65`)
2. **Hexadecimal**: With `0x` prefix (`0x3f`) 
3. **Literal**: ASCII character in single quotes (`'A'`)

### ID Literals

Most game IDs (units, items, destructibles) use 4-character literals: `'Ahbz'`

## String Escape Characters

- `\n` and `\r`: Line breaks
- `\"`: Literal quote character in strings
- `\\`: Literal backslash character in strings

## Additional Resources

For comprehensive JASS documentation and syntax checking: [http://jass.sourceforge.net/doc/](http://jass.sourceforge.net/doc/)