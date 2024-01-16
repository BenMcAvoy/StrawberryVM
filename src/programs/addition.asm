; 01 0A 01 F0 03 00 02 00 05 90

; Push values onto stack
Push 10
Push 240

; Add and store in A
AddStack
PopReg A

; Print A
Signal 241

; Quit
Signal 240
