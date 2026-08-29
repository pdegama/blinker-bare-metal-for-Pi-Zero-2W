.section ".text.boot"
.global _start

_start:
  mrs x1, mpidr_el1
  and x1, x1, #3
  cbz x1, main

hang: 
  wfe
  b hang

main:
  ldr x1, =_start
  mov sp, x1

# clean bss sec
  ldr x1, =__bss_start
  ldr w2, =__bss_size
cleanbss: 
  cbz w2, mainfn
  str xzr, [x1], #8
  sub w2, w2, #1
  cbnz w2, cleanbss

mainfn: 
  bl blinker
  b hang

.global delay
delay:
  subs x0, x0, #1 
  bne delay
  ret

.global get_el
get_el:
  mrs x0, CurrentEL
  lsr x0, x0, #2
  ret
