push 2
push 2

print:
  signal $F1

jmp ^print

; Quit
signal $F0
