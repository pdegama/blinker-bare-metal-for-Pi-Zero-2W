ARMGNU ?= aarch64-linux-gnu

AOPS = -Wall -ffreestanding -nostdlib -nostartfiles -c 

all: blink.img

boot.o: src/boot.s
	$(ARMGNU)-gcc $(AOPS) src/boot.s -o boot.o

libblinker.a: libblinker.a
	cargo build --release

kernel.elf: boot.o libblinker.a src/linker.ld
	$(ARMGNU)-ld -T src/linker.ld -o kernel.elf boot.o target/aarch64-unknown-none/release/libblinker.a

blink.img: kernel.elf
	$(ARMGNU)-objcopy kernel.elf -O binary blink.img

clean:
	rm -f *.o *.elf *.img
