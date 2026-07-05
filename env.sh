#!/usr/bin/env bash


script_dir=$(dirname "$(realpath "${BASH_SOURCE[0]}")")
cd "$script_dir"; pwd

echo "---------- ---------- ---------- ----------"
sudo pacman -Sy rustup
rustup install nightly
rustup default nightly
rustc --version
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools-preview
rustup component add rust-src

echo "---------- ---------- ---------- ----------"
sudo pacman -Sy qemu-base qemu-common qemu-system-x86 qemu-system-riscv
qemu-system-riscv64 --version
# qemu-riscv64 --version

echo "---------- ---------- ---------- ----------"
sudo pacman -Sy clang
cargo install bindgen-cli
cargo install cargo-binutils

# echo "---------- ---------- ---------- ----------"
# sudo pacman -Sy musl-x86
# sudo pacman -Sy musl-riscv64
# sudo pacman -Sy musl-aarch64

echo "---------- ---------- ---------- ----------"
make -f Makefile.camp configure
make -f Makefile.camp build

echo "---------- ---------- ---------- ----------"
make -f Makefile.camp test-rust

