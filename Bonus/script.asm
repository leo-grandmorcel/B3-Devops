section .data
    number db '0', 0
    message db 'Number: ', 0

section .text
    global _start

_start:
    mov ecx, 1 

print_loop:
    mov eax, 4
    mov ebx, 1
    mov edx, 8
    mov esi, message
    mov edi, edx
    int 0x80

    mov eax, 4
    mov ebx, 1
    mov edx, 4
    mov esi, number
    mov edi, edx
    int 0x80

    inc ecx

    cmp ecx, 10001
    jle print_loop

exit:
    mov eax, 1
    xor ebx, ebx
    int 0x80
