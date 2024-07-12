RUST_FLAGS=--edition=2021 -Z threads=10

bin: main.rs
	rustc -o $@ $(RUST_FLAGS) $<
