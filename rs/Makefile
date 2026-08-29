ARMGNU ?= aarch64-linux-gnu

AOPS = -Wall -ffreestanding -nostdlib -nostartfiles -c 

all: blink.img stub.img

util.o: src/util.s
	$(ARMGNU)-gcc $(AOPS) src/util.s -o util.o

boot.o: src/boot.s
	$(ARMGNU)-gcc $(AOPS) src/boot.s -o boot.o

libblinker.a: libblinker.a
	cargo build --release

kernel.elf: boot.o util.o libblinker.a src/linker.ld
	$(ARMGNU)-ld -T src/linker.ld -o kernel.elf boot.o util.o target/aarch64-unknown-none/release/libblinker.a

blink.img: kernel.elf
	$(ARMGNU)-objcopy kernel.elf -O binary blink.img

stub.img: src/stub.s libblinker.a util.o
	$(ARMGNU)-gcc $(AOPS) -MMD src/stub.s -o stub.o
	$(ARMGNU)-ld --section-start=.text=0 -o stub.elf stub.o util.o target/aarch64-unknown-none/release/libblinker.a
	$(ARMGNU)-objcopy stub.elf -O binary stub.img

clean:
	rm -f *.o *.elf * .img
