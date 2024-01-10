ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

ARCH ?= x86_64
CONFIGURATION ?= debug

CARGO ?= cargo
OBJCOPY ?= objcopy
NM ?= nm
CPPFILT ?= c++filt

CARGO_ARGS :=
ifeq ($(CONFIGURATION), release)
	CARGO_ARGS += --release
endif

.PHONY: all
all: image

target/$(ARCH)-unknown-uefi/OVMF_VARS.fd: 
	cp $(OVMF_PATH)/OVMF_VARS.fd target/$(ARCH)-unknown-uefi/OVMF_VARS.fd
	chmod 644 target/$(ARCH)-unknown-uefi/OVMF_VARS.fd

.PHONY: qemu
qemu: image target/$(ARCH)-unknown-uefi/OVMF_VARS.fd
	$(QEMU) \
		-drive "if=pflash,format=raw,file=$(OVMF_PATH)/OVMF_CODE.fd,readonly=on" \
		-drive "if=pflash,format=raw,file=$(ROOT_DIR)/target/$(ARCH)-unknown-uefi/OVMF_VARS.fd" \
		-drive "format=raw,file=fat:rw:$(ROOT_DIR)/target/image" \
		-net none \
		-serial stdio

.PHONY: image
image: oxyboot oxykernel
	@test -d "$(ROOT_DIR)/target/image/efi/boot" || mkdir -p $(ROOT_DIR)/target/image/efi/boot
	cp $(ROOT_DIR)/target/$(ARCH)-unknown-uefi/$(CONFIGURATION)/oxyboot.efi $(ROOT_DIR)/target/image/efi/boot/bootx64.efi
	cp $(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel $(ROOT_DIR)/target/image/efi/boot/oxykernel

.PHONY: oxyboot
oxyboot:
	$(CARGO) build \
		$(CARGO_ARGS) \
		--target $(ARCH)-unknown-uefi \
		--package oxyboot

.PHONY: oxykernel
oxykernel:
	$(CARGO) rustc \
		$(CARGO_ARGS) \
		-Z build-std=core,alloc \
		-Z build-std-features=compiler-builtins-mem \
		--target "$(ROOT_DIR)/crates/oxykernel/targets/$(ARCH)-oxygen-kernel.json" \
		--package oxykernel \
		-- \
		-C link-arg=-T -C link-arg="$(ROOT_DIR)/crates/oxykernel/linkers/$(ARCH).ld" \
		--emit link=$(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel.all
	$(OBJCOPY) \
		--only-keep-debug \
		$(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel.all \
		$(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel.sym
	$(OBJCOPY) \
		--strip-debug \
		$(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel.all \
		$(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel
	$(NM) $(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel | $(CPPFILT) > $(ROOT_DIR)/target/$(ARCH)-oxygen-kernel/$(CONFIGURATION)/oxykernel.names
