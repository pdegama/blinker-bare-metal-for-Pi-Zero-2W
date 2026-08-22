ARMGNU ?= aarch64-linux-gnu

AOPS = -Wall -ffreestanding -nostdlib -nostartfiles -c 
COPS = -Wall -ffreestanding -nostdlib -nostartfiles -c

all: blink.img

boot.o: boot.s
	$(ARMGNU)-gcc $(AOPS) boot.s -o boot.o

kernel.o: kernel.c
	$(ARMGNU)-gcc $(COPS) kernel.c -o kernel.o

kernel.elf: boot.o kernel.o linker.ld
	$(ARMGNU)-ld -T linker.ld -o kernel.elf boot.o kernel.o

blink.img: kernel.elf
	$(ARMGNU)-objcopy kernel.elf -O binary blink.img

clean:
	rm -f *.o *.elf *.img
