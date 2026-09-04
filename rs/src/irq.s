.globl irq_init_vectors
irq_init_vectors:
    #stp x29, x30, [sp, -16]!
    #mov x29, sp
    #bl blink_error_led
    adr x0, vectors
    msr vbar_el1, x0
    #ldp x29, x30, [sp], 16
    ret

.globl irq_enable
irq_enable:
    msr daifclr, #2
    ret

.globl irq_disable
irq_disable:
    msr daifset, #2
    ret

# macro

.macro  ventry  label
.align  7
    b \label
.endm

.macro kernel_entry
  sub sp, sp, #256
  stp x0, x1, [sp, #16 * 0]
  stp x2, x3, [sp, #16 * 1]
  stp	x4, x5, [sp, #16 * 2]
	stp	x6, x7, [sp, #16 * 3]
	stp	x8, x9, [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]
	str	x30, [sp, #16 * 15] 
.endm

.macro kernel_exit
  ldp x0, x1, [sp, #16 * 0]
  ldp x2, x3, [sp, #16 * 1]
  ldp	x4, x5, [sp, #16 * 2]
	ldp	x6, x7, [sp, #16 * 3]
	ldp	x8, x9, [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]
	ldr	x30, [sp, #16 * 15] 
	add	sp, sp, #256
  eret
.endm

handle_el1_irq:
	kernel_entry 
	bl	handle_irq
	kernel_exit

error_irq:
	kernel_entry 
	bl blink_error_led
	kernel_exit 

# vectors table
.align  11
.globl vectors 
vectors:
  ventry error_irq
  ventry error_irq
  ventry error_irq
  ventry error_irq

	ventry error_irq
	ventry handle_el1_irq
  ventry error_irq
  ventry error_irq

	ventry error_irq
  ventry error_irq
  ventry error_irq
  ventry error_irq

	ventry error_irq
  ventry error_irq
  ventry error_irq
  ventry error_irq

