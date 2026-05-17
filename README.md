# miOS
A simple rust-based x86_64 operating system.

## Contents:
- [Features](#features)
- [Dependencies](#dependencies)
- [Build Proccess](#build-proccess)

## Features:
- Uses Limine bootloader
- Framebuffer text rendering (Cozette font)
- Serial Output for debugging in development
- IDT and PIC modules
- Keyboard driver for typing

## Dependencies:
- Rust Nightly (doesnt work with standard rust)
- qemu-system-x86_64 (to boot the kernel binary for testing)
- xorriso (to create bootable iso image file)

## Build Proccess:
- clone the repo, then
- run 'make setup' in the parent folder
- run 'make run' to compile, create iso image, and boot it in qemu
- run 'make clean' to remove build artifacts.
