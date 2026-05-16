# miOS
A simple rust-based x86_64 operating system.

## Contents:
- [Features](#features)
- [Dependencies](#dependencies)
- [Build Proccess](#build-process)

## Features:
- Uses limine bootloader
- Framebuffer text rendering (Cozette font)
- Serial Output for debugging in development

## Dependencies:
- Rust Nightly (doesnt work with standard rust)
- qemu-system-x86_64 (to boot the kernel binary for testing)
- xorriso (to create bootable iso image file)

## Build Proccess:
- clone the repo, then
- run 'make setup' in the parent folder
- run 'make run' to compile, create iso image, and boot it in qemu.
