push 2
push 2

print:
  signal $F2

jmp ^print

; Quit
signal $F0
