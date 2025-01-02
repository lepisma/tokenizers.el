all: tokenizers.so

tokenizers.so: ./target/debug/libtokenizers_el.so
	cp ./target/debug/libtokenizers_el.so tokenizers.so

./target/debug/libtokenizers_el.so: ./src/lib.rs
	cargo build
