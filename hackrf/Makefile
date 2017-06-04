all: executables examples doc

executables: executable-debug executable-release

executable-debug:
	cargo build

executable-release:
	cargo build --release

examples: examples-debug examples-release

examples-debug:
	cargo build --examples

examples-release:
	cargo build --examples --release

doc:
	cargo doc

clean: clean-executables

clean-executables: clean-executables-debug clean-executables-release

clean-executables-debug:
	cargo clean

clean-executables-release:
	cargo clean --release

