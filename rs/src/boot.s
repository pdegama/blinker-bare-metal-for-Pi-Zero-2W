.section ".text.boot"
.global _start

_start:
  mrs x1, mpidr_el1
  and x1, x1, #3
  cbz x1, root_fn

hang: 
  wfe
  b hang

root_fn:
    ldr x1, =_start
    mov sp, x1

    ldr x1, =__bss_start
    ldr w2, =__bss_size
cleanbss: 
    cbz w2, fall_to_el1
    str xzr, [x1], #8
    sub w2, w2, #1
    cbnz w2, cleanbss

fall_to_el1:
    bl blink_el_with_delay
    ldr x0, =SCTLR_VALUE_MMU_DISABLED
    ldr x0, [x0]
    msr sctlr_el1, x0

    ldr x0, =HCR_VALUE
    ldr x0, [x0]
    msr hcr_el2, x0

    bl get_el

    cmp x0, #3
    beq from_el3_to_el

    cmp x0, #2
    beq from_el2_to_el

    b main_el1

from_el3_to_el:
    ldr x0, =SCR_VALUE
    ldr x0, [x0]
    msr scr_el3, x0

    ldr x0, =SPSR_VALUE
    ldr x0, [x0]
    msr spsr_el3, x0

    adr x0, main_el1
    msr elr_el3, x0

    eret

from_el2_to_el:
    ldr x0, =SPSR_VALUE
    ldr x0, [x0]
    msr spsr_el2, x0

    adr x0, main_el1
    msr elr_el2, x0

    eret

main_el1:
  ldr x1, =_start
  mov sp, x1

# clean bss sec
  ldr x1, =__bss_start
  ldr w2, =__bss_size
cleanbss_el1: 
  cbz w2, mainfn
  str xzr, [x1], #8
  sub w2, w2, #1
  cbnz w2, cleanbss_el1

mainfn:
  bl blinker
  b hang

