ARCH = x86_64-mi_os
KERNEL = target/$(ARCH)/release/miOS
ISO = miOS.iso
ISO_ROOT = iso_root

.PHONY: all run clean setup

all: $(ISO)

setup:
	git clone https://github.com/limine-bootloader/limine --branch=v8.x-binary --depth=1 limine
	make -C limine

build:
	cargo build --release

$(ISO): build
	mkdir -p $(ISO_ROOT)/boot/limine
	cp $(KERNEL) $(ISO_ROOT)/boot/
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin $(ISO_ROOT)/boot/limine/
	cp limine.conf $(ISO_ROOT)/boot/limine/
	xorriso -as mkisofs \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO)
	./limine/limine bios-install $(ISO)

run: $(ISO)
	qemu-system-x86_64 -M q35 -m 128M -cdrom $(ISO) -serial stdio

clean:
	cargo clean
	rm -rf $(ISO_ROOT) $(ISO)
