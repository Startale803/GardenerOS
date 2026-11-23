TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/release/os
KERNEL_BIN := $(KERNEL_ELF).bin
BOOTLOADER ?= ../bootloader/opensbi.bin
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: kernel
	$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

kernel:
	cargo build --release

run: build
	qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=0x80200000

check: build
	rust-readobj -h $(KERNEL_ELF)

disasm: kernel
	rust-objdump --arch-name=riscv64 -d $(KERNEL_ELF)

debug: build
	qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=0x80200000 -s -S

clean:
	cargo clean

.PHONY: build kernel run check disasm debug clean
