ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
UEFI_TARGET ?= x86_64-unknown-uefi
KERNEL_TARGET ?= x86_64-oxygen-kernel
CARGO ?= cargo

.PHONY: all
all: image

.PHONY: image
image: oxyboot oxykernel
	@test -d "$(ROOT_DIR)/target/image/efi/boot" || mkdir -p $(ROOT_DIR)/target/image/efi/boot
	cp $(ROOT_DIR)/target/$(UEFI_TARGET)/debug/oxyboot.efi $(ROOT_DIR)/target/image/efi/boot/bootx64.efi
	cp $(ROOT_DIR)/target/$(KERNEL_TARGET)/debug/oxykernel $(ROOT_DIR)/target/image/efi/boot/oxykernel

.PHONY: oxyboot
oxyboot:
	$(CARGO) build --package oxyboot --target $(UEFI_TARGET)

.PHONY: oxykernel
oxykernel:
	$(CARGO) build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --package oxykernel --target "$(ROOT_DIR)/$(KERNEL_TARGET).json"