.section .bootstrap, "awx"
.global _start
.intel_syntax noprefix
.code16

_start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    lea ebx, _stack_end
    mov esp, ebx

    call rust_start

spin:
    cli
    hlt
    jmp spin