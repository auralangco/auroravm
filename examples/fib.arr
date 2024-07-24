$main:
    push 4i ; pushes 4 to the stack
    push $fib ; pushes the functin $fib to the stack
    call ; calls fib with 10 as its argument
    natv `to_string ; Stringifies the result of the call
    natv `println ; Prints the result
    retn

$fib: ; The only argument is n
    jz #zero        ; If n == 0
    dup             ; | n n
    push 1i         ; | n n 1
    eq              ; | n (n == 1)
    jnz #one        ; If n == 1
    dump            ; Dumps the cmp result
    
    dup             ; | n n
    push 1i         ; | n n 1
    sub             ; | n (n - 1)
    push $fib       ; | n (n - 1) $fib
    call            ; | n fib(n-1)
    
    swap            ; | fib(n-1) n
    push 2i         ; | fib(n-1) n 2
    sub             ; | fib(n-1) (n - 2)
    push $fib       ; | fib(n-1) (n - 2) $fib
    call            ; | fib(n-1) fib(n-2)
    add             ; | (fib(n-1) + fib(n-2))
    retn

    #zero:
    #one:
    push 1 ; If n is 0 or 1 the fibbonacci number is 1 
    retn