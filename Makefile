ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

ARCH ?= x86_64
TARGET ?= $(ARCH)-unknown-oxygen
CONFIGURATION ?= debug

OUT_DIR := $(ROOT_DIR)/os/target/$(TARGET)/$(CONFIGURATION)

CARGO ?= cargo
OBJCOPY ?= objcopy
NM ?= nm
RUSTFILT ?= rustfilt

CARGO_ARGS :=
ifeq ($(CONFIGURATION), release)
	CARGO_ARGS += --release
endif

.PHONY: all
all: image

# Copy the OVMF_VARS file, because it could be in read-only storage if it's coming from Nix.
os/target/$(ARCH)_OVMF_VARS.fd:
	mkdir -p $(ROOT_DIR)/os/target
	cp $(OVMF_PATH)/OVMF_VARS.fd os/target/$(ARCH)_OVMF_VARS.fd
	chmod 644 os/target/$(ARCH)_OVMF_VARS.fd

.PHONY: qemu
qemu: image os/target/$(ARCH)_OVMF_VARS.fd
	$(QEMU) \
		-drive "if=pflash,format=raw,file=$(OVMF_PATH)/OVMF_CODE.fd,readonly=on" \
		-drive "if=pflash,format=raw,file=$(ROOT_DIR)/os/target/$(ARCH)_OVMF_VARS.fd" \
		-drive "format=raw,file=$(OUT_DIR)/oxygen.img" \
		-net none \
		-serial stdio

.PHONY: image
image: oxy-kernel
	$(CARGO) run $(CARGO_ARGS) \
		--manifest-path $(ROOT_DIR)/tools/build-image/Cargo.toml \
		-- \
		$(OUT_DIR)/oxy-kernel \
		$(OUT_DIR)/oxygen.img
		
.PHONY: oxy-kernel
oxy-kernel:
	$(CARGO) build $(CARGO_ARGS) \
		--manifest-path $(ROOT_DIR)/os/Cargo.toml \
		-Z build-std=core,alloc \
		-Z build-std-features=compiler-builtins-mem \
		--target $(ROOT_DIR)/os/$(TARGET).json \
		--package oxy-kernel
	$(NM) $(OUT_DIR)/oxy-kernel | $(RUSTFILT) > $(OUT_DIR)/oxy-kernel.names
