trace:
	strace target/debug/os
run:
	cargo run
file:
	file target/riscv64gc-unknown-none-elf/debug/os
read:
	rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os
dump:
	rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os
strip:
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
qemu:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
		-s -S
os:
	riscv64-unknown-elf-gdb \
		-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'
build:
	cargo build --release
test: build strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
