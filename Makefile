test:
	cargo run -- example.bugu
	spike --isa=RV32G env/pk example
	echo $?

test2:
	cargo run -- example2.bugu
	spike --isa=RV32G env/pk example2
gcc:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 example.S -o example_g

read:
	riscv-gcc/bin/riscv64-unknown-elf-readelf -a example

obj:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 -c example.S -o example_g.o

reado:
	riscv-gcc/bin/riscv64-unknown-elf-readelf -a example.o

link:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -nostdlib example.o /Users/buhe/code/gitHub/buguOS/user/target/riscv64gc-unknown-none-elf/release/libuser.a -o all

c:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -c -o h.o env/h.c

test3:
	cargo run -- -o example.bugu

read2:
	riscv-gcc/bin/riscv64-unknown-elf-readelf -h all

linkc:
	riscv-gcc/bin/riscv64-unknown-elf-gcc -nostdlib h.o /Users/buhe/code/gitHub/buguOS/user/target/riscv64gc-unknown-none-elf/release/libuser.a -o call