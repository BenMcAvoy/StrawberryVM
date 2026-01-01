; Setup registers
Push 0        ; 00-01: initial counter = 0
Pop A         ; 02-03
Push 1        ; 04-05: increment = 1
Pop B         ; 06-07
Push 125      ; 08-09: stop value = 1000/8
Pop C         ; 10-11
Push 3        ; 12-13: shift amount for stop value (x8)
Pop D         ; 14-15
Shl C D       ; 16-17: stop = stop << shift (=1000)

Push 10
Pop A
Mul C A

; LoopStart (PC=18)
Add A B       ; 18-19: counter += 1
Signal $F1    ; 20-21: print A
Cmp A C       ; 22-23: compare counter vs stop
Je 1          ; 24-25: jump to EndLoop if counter == stop
Jmp -5        ; 26-27: jump back to LoopStart

; EndLoop (PC=28)
Signal $F0    ; 28-29: halt