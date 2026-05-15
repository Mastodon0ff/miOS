ARCH = x86_64-mi_os
KERNEL = target/$(ARCH)/debug/miOS
ISO = miOS.iso
ISO_ROOT = iso_root

.PHONY: all run clean

all: $(ISO)

build:
	cargo build

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
	qemu-system-x86_64 -M q35 -m 128M -cdrom $(ISO)

clean:
	cargo clean
	rm -rf $(ISO_ROOT) $(ISO)
