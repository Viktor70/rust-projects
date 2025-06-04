org 0x7c00	 ; Set origin to 0x7c00
bits 16		; 16-bit real mode

start:
    jmp main

main:
    ; Initialize segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00    ; Stack grows downwards from 0x7C00
    
    ; Clear screen
    mov ax, 0x0003    ; 80x25 text mode
    int 0x10
    
    mov si, hello_msg   ; Print a welcome message
    call print_string
    
    mov si, prompt_msg  ; Print a prompt message
    call print_string
    
    ; Wait for any key press (no need to store the character)
    mov ah, 0x00       ; BIOS keyboard input function
    int 0x16           ; Call BIOS interrupt (waits for key)
    
    mov si, newline     ; Move cursor to the next line
    call print_string
    
    mov si, echo_msg    ; Print the shutdown message
    call print_string
    
    call shutdown       ; Perform shutdown
    
    jmp $               ; Infinite loop (just in case)

print_string:
    lodsb               ; Load the next character
    or al, al           ; Check for null terminator
    jz end_print_string ; If null, end of string
    
    call print_char     ; Print the character
    jmp print_string    ; Repeat for the next character
    
end_print_string:
    ret

print_char:
    mov ah, 0x0E        ; BIOS teletype function
    mov bx, 0x0007      ; Page 0, attribute 7 (light gray)
    int 0x10            ; Call BIOS interrupt
    ret

shutdown:
    mov ax, 0x2000      ; ACPI shutdown command
    mov dx, 0x604       ; Port for ACPI shutdown
    out dx, ax
    ret



; Data section
hello_msg db "Welcome to Bootloader!", 0x0D, 0x0A, 0
prompt_msg db "Press any key to shutdown...", 0
echo_msg db "System is shutting down...", 0x0D, 0x0A, 0
newline db 0x0D, 0x0A, 0

times 510-($-$$) db 0   ; Pad the bootloader to 510 bytes
dw 0xAA55               ; Boot signature