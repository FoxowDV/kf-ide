.MODEL SMALL
.STACK 256

; prints string, no newline
print_str MACRO str
    MOV AH, 09h
    LEA DX, str
    INT 21h
ENDM

; AX = 1 if greater, 0 if not
bool_g MACRO
    LOCAL b1, b2
    JG  b1
    XOR AX, AX
    JMP b2
b1: MOV AX, 1
b2:
ENDM

; AX = 1 if less, 0 if not
bool_l MACRO
    LOCAL b1, b2
    JL  b1
    XOR AX, AX
    JMP b2
b1: MOV AX, 1
b2:
ENDM

; AX = 1 if not equal, 0 if equal
bool_ne MACRO
    LOCAL b1, b2
    JNE b1
    XOR AX, AX
    JMP b2
b1: MOV AX, 1
b2:
ENDM

.DATA
    _a  DW 0
    _b  DW 0
    _t0  DW 0


    nl  DB 0Dh, 0Ah, '$'

.CODE

_suma PROC
    PUSH BP
    MOV BP, SP
    MOV AX, [BP+4]
    MOV [_a], AX
    MOV AX, [BP+6]
    MOV [_b], AX
    MOV AX, [_a]
    ADD AX, [_b]
    MOV [_t0], AX
    MOV AX, [_t0]
    POP BP
    RET 4
    POP BP
    RET 4
_suma ENDP

; ── Runtime ──

print_num PROC
    PUSH AX
    PUSH BX
    PUSH CX
    PUSH DX
    CMP AX, 0
    JGE pn_pos
    PUSH AX
    MOV DL, '-'
    MOV AH, 02h
    INT 21h
    POP AX
    NEG AX
pn_pos:
    MOV BX, 10
    XOR CX, CX
pn_div:
    XOR DX, DX
    DIV BX
    PUSH DX
    INC CX
    TEST AX, AX
    JNZ pn_div
pn_pr:
    POP DX
    ADD DL, '0'
    MOV AH, 02h
    INT 21h
    LOOP pn_pr
    print_str nl
    POP DX
    POP CX
    POP BX
    POP AX
    RET
print_num ENDP

END START