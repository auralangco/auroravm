# Aurora Specification

Aurora is intend to run function calls into separated environments with no side-effects so parallelism can be natural. By side-effects here we consider only memory manipulation related stuff (IO operations side-effects are ignored). What is done inside a run environment remains contained into it and only the parent run environment can have access to the result.

## Run Environment

Each run environment contains:

- Program: the instructions that must be executed
- Program Counter: the pointer to the next instruction to be executed
- Stack: the data to be used for other function calls, accessed in last-in-first-out order 
- Heap: data stored to be accessed out-of-order
- HeapMap: the map from a label to a heap address

At the end of the execution, the top of the stack is the returned value (if the function do returns something)

## Data

Everything in aurora is data. The stack stores the data with it's meaning (AKA type)

- `byte`: a single byte
- `byte2`: 2 bytes
- `byte4`: 4 bytes
- `byte8`: 8 bytes
- `int`: an 8 bits signed integer
- `int2`: a 16 bits signed integer
- `int4`: a 32 bits signed integer
- `int8`: a 64 bits signed integer
- `float`: a 32 bits floating point number
- `float2`: a 64 bits floating point number
- `(...)`: a compound type
- `|...|`: an union type
- `[T]`: an array type

### Compound Type

The compound type is a tool to deal with data that may be divided into parts (fields) where field is named with `` `n`` (`` `0, `1, `2, `3, ...``)

```
    push (float2 byte8) 3.14f2 100000b8 ; pushes (3.14, 100000) into the stack
    dup `1 ; duplicates only the byte8 field to the top of the stack
    pop @0 ; pops it to the label 0
```

### Union Type

The union type is a tool to deal with data that can assume values from different types and every variant is named with `` `n`` (`` `0, `1, `2, `3, ...``)

## Expressions

- ``~n``: the `n`-th element of an array (a literal natural number)
- `` `n``: the `n`-th field of a compound or variant in a union
- ``@ptr``: a pointer to a position in the heap
- ``#label``: a label in the program
- ``'c'``: a single character
- ``"string"``: a utf-8 string
- ``[0-9]+``:  a base 10 number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`, `i`, `i2`, `i4`, `i8`
- ``0b[01]+``: a binary number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`
- ``0x[0-9a-f]+`` a hexadecimal number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`
- ``[0-9]+.[0-9]+f``: a floating point literal, (add `2` to the suffix if it's a `float2`) 

## Instruction Set

### ``mov @ptr ...``

Moves the expression to `@ptr` in the heap

### ``push ...``

Pushes the expression to the stack

### ``push `n ...``

Pushes the field `` `n`` of the expression to the stack

### ``dump``

Dumps the top of the stack

### ``pop @ptr``

Pops the top of the stack to the position `@ptr` in the heap

### ``pop `n @ptr``

Pops just the field `` `n`` to `@ptr` in the heap

### `dup`

Duplicates the top of the stack

### `dups`

Duplicates the semitop of the stack

### `swap`

Swaps the two tops of the stack

### `join`

Joins the two top values of the stack into a compound value

### ``split `n``

Splits the two top values of the stack into two compound values, fields from `n` and beyond remain at the top

### `add`

Adds the two values on the top of the stack and pushes the result to the stack

### `sub`

Subtract the two values on the top of the stack and pushes the result to the stack

### `mul`

Multiplies the two values on the top of the stack and pushes the result to the stack

### `div`

Divides the semitop by the top of the stack and pushes the result to the stack

### `mod`

Calculates semitop modulus the top of the stack and pushes the result to the stack

### `pow`

Calcultes semitop to the top-th power

### `bwand`

Bitwise and of the two top values

### `bwor`

Bitwise or of the two top values

### `bwxor`

Bitwise xor of the two top values

### `bwnot`

Bitwise negation of the top value

### `eq`

Compares if the two top values are equals

### `neq`

Compares if the two top values are not equals

### `gt`

Compares if the semitop is greater than the top

### `lt`

Compares if the semitop is less than the top

### `geq`

Compares if the semitop is greater or equals to the top

### `leq`

Compares if the semitop is less ot equals to the top

### `not`

Inverts the top of the stack (true <-> false)

### `and`

Logical and of the two top values

### `or`

Logical or of the two top values

### `jmp #label`

Jumps to the given label in the current program

### `jz #label`

Jumps if the top of the stack is zeroed

### `jnz #label`

Jumps if the top of the stack isn't zeroed

### `call`

Calls the function pointer at the top of the stack creating a new nested execution environment that will return a value

### `callnr`

Calls the function pointer at the top of the stack creating a new nested execution environment that won't return a value 

### `natv "symbol"`

Calls a native function implemented in Rust using the top of the stack as it's argument (returns a value)

### `natvnr "symbol"`

Calls a native function implemented in Rust using the top of the stack as it's argument (won't return a value)

### `exit`

Finishes the current execution environment

### `retn`

Finishes the current execution environment and returns the top of the stack