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
- ``@n``: the label `n` in the heap
- ``'c'``: a single character
- ``"string"``: a utf-8 string
- ``[0-9]+``:  a base 10 number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`, `i`, `i2`, `i4`, `i8`
- ``0b[01]+``: a binary number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`
- ``0x[0-9a-f]+`` a hexadecimal number literal, must be suffixed with it's kind `b`(`byte`), `b2`, `b4`, `b8`
- ``[0-9]+.[0-9]+f``: a floating point literal, (add `2` to the suffix if it's a `float2`) 

## Instruction Set

- `push T ...v` pushes the values to the stack with the given type