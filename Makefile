# Soundness Layer dev workflow.
# Requires the SP1 toolchain: curl -L https://sp1.succinct.xyz | bash
# and rustc 1.81+.

.PHONY: build prove verify submit clean

PROGRAM_ELF := program/elf/fibonacci-program

build:
	cargo build --release --manifest-path program/Cargo.toml
	mkdir -p program/elf
	# After `cargo prove build` runs, the ELF is in:
	#   target/elf-compilation/riscv32im-succinct-zkvm-elf/release/fibonacci-program
	# For convenience we copy it to program/elf/ so include_bytes! can find it.
	cargo prove build --manifest-path program/Cargo.toml --output-directory program/elf

prove: build
	cargo run --release --bin prove --manifest-path script/Cargo.toml

verify:
	cargo run --release --bin verify --manifest-path script/Cargo.toml

submit:
	cargo run --release --bin submit --manifest-path script/Cargo.toml

clean:
	rm -rf program/elf program/target script/target out
