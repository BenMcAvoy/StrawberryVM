# Assembly pass plan
Input:
```asm
Jump ^DebugTwo      ; Jump to our debug subroutine

Debug 1             ; Debug our value one
Signal $F0          ; Quit

DebugTwo:           ; Our debug subroutine
    Debug 2         ; Debug our value two
    Signal $F1      ; Print A

    Return          ; Jump back
```

Pass 1, label names:
```asm
LoadAImm $[DebugTwoValue]$
JumpAndStore A

Debug 1             ; Debug our value one
Signal $F0          ; Quit

; $[DebugTwoLabel]$
Debug 2         ; Debug our value two
Signal $F1      ; Print A

Return          ; Jump back
```

Pass 2, returns passing:
```asm
LoadAImm $[DebugTwoValues]$
JumpAndStore A

Debug 1             ; Debug our value one
Signal $F0          ; Quit

; $[DebugTwoLabel]$
Debug 2         ; Debug our value two
Signal $F1      ; Print A
PopReg A
JumpAndStore A
```

Pass 3, label value substitution
```asm
LoadAImm 8
JumpAndStore A

Debug 1             ; Debug our value one
Signal $F0          ; Quit

; $[DebugTwoLabel]$
Debug 2         ; Debug our value two
Signal $F1      ; Print A
PopReg A
JumpAndStore A
```

Pass 4, remove comments
```asm
LoadAImm 8
JumpAndStore A

Debug 1
Signal $F0

Debug 2
Signal $F1
PopReg A
JumpAndStore A
```
