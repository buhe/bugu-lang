test:
	cargo run -- example.bugu
	spike --isa=RV32G env/pk example

test2:
	cargo run -- example2.bugu
	spike --isa=RV32G env/pk example2