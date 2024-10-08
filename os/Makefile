# Makefile args:
# - `MODE`: Run with `Mode=debug` to build in debug mode.
#  		By default, rros will run in debug mode.
# - `FEATURES`: Features to enable when build
#		["test", ] 

# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
FEATURES := default

# BOARD
BOARD := qemu
SBI ?= opensbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
BOOTLOADER_ELF := ../bootloader/$(SBI)-$(BOARD)

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# QEMU
QEMU_ARGS := 	-machine virt \
				-nographic \
				-bios ../bootloader/opensbi-qemu.bin \
				-device loader,file=target/riscv64gc-unknown-none-elf/debug/os,addr=0x80200000  

# gdb
GDB_CLIENT := riscv64-unknown-elf-gdb
GDB_ARGS := -ex 'file target/riscv64gc-unknown-none-elf/debug/os' \
    		-ex 'set arch riscv:rv64' \
			-ex 'target remote localhost:1234'

# 检查是否存在 riscv64-unknown-elf-gdb
ifeq ("$(shell command -v $(GDB_CLIENT))", "")
    GDB_CLIENT := gdb-multiarch
# 检查是否存在 gdb-multiarch
else ifeq ("$(shell command -v $(GDB_CLIENT))", "")
    $(error "No suitable GDB client found. Please install riscv64-unknown-elf-gdb or gdb-multiarch.")
endif

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

build:  
	cargo build $(MODE_ARG) --features $(FEATURES)
	
run: env build 
	qemu-system-riscv64 $(QEMU_ARGS)

clean:
	@cargo clean

gdbserver: env build
	qemu-system-riscv64 $(QEMU_ARGS) -s -S 

gdbclient: 
	@echo "Using GDB: $(GDB_CLIENT)"
	$(GDB_CLIENT) $(GDB_ARGS)

	