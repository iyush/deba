#!/bin/bash
set -e

make
qemu-system-x86_64 -cdrom template-x86_64.iso -s -S -m 256 -serial stdio &
rust-gdb kernel/kernel -x kernel/init.gdb
wait
