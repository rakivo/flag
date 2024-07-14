RUST_FLAGS=--edition=2021 -Z threads=10

bin: src/main.rs src/try_parse.rs
	rustc -o $@ $(RUST_FLAGS) $<
