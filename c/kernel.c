#define MMIO_BASE 0x3F000000
#define GPIO_BASE (MMIO_BASE + 0x200000)

#define GPFSEL2 (unsigned int *)(GPIO_BASE + 0x08)
#define GPSET0 (unsigned int *)(GPIO_BASE + 0x1C)
#define GPCLR0 (unsigned int *)(GPIO_BASE + 0x28)

void main() {
  // set pins
  unsigned int *reg = GPFSEL2;
  *reg = 1 << 3;

  while (1) {
    *GPSET0 = 1 << 21;
    for (int x = 0; x < 400000; x++) {
      asm volatile("nop");
    }

    *GPCLR0 = 1 << 21;
    for (int x = 0; x < 400000; x++) {
      asm volatile("nop");
    }
  }

  return;
}
