
all: nanocli

nanocli: nanocli.rs
	rust build nanocli.rs

run: nanocli.rs
	rust run nanocli.rs
