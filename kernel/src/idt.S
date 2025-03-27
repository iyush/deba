.text
    .global all_interrupts_handler

    # Macro to generate interrupt wrappers
    .macro interrupt_wrapper num
    # .global int_wrapper_\num
int_wrapper_\num:
    # Push interrupt number
    push \num

    # Save flags
    pushfq

    # Save all registers in opposite order of struct regs
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsp
    push rdi
    push rsi
    push rbp
    push rdx
    push rcx
    push rbx
    push rax

    # Prepare for handler
    cld
    mov rdi, rsp    # Pass pointer to register structure as first argument
    call all_interrupts_handler

    # Restore registers
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rbp
    pop rsi
    pop rdi
    pop rsp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15

    # Restore flags
    popfq

    # Clean up interrupt number from stack
    add rsp, 8

    # Return from interrupt
    iretq
    .endm

    # Generate interrupt wrappers for specific interrupt numbers
    .irp num, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 32, 33
        interrupt_wrapper \num
    .endr

    # Special case for interrupt 99
    # .global int_wrapper_99
int_wrapper_99:
    push 99
    pushfq
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsp
    push rdi
    push rsi
    push rbp
    push rdx
    push rcx
    push rbx
    push rax

    cld
    mov rdi, rsp
    call all_interrupts_handler

    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rbp
    pop rsi
    pop rdi
    pop rsp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15

    popfq
    add rsp, 8
    sysretq
