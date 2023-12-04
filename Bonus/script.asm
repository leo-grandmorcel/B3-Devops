.global _start
.data
hi:.string "hi\n"
.text
_start:
    mov $0,%r8
    call hi_loop
hi_loop:
    mov $1,%rax
    mov $1,%rdi
    mov $hi,%rsi
    mov $4,%rdx
    syscall
    add $1,%r8
    cmp $3,%r8
    jz exit
    jmp hi_loop
exit:
    mov $60,%rax
    mov $0,%rdi
    syscall