release: tokenizers-core.so

tokenizers-core.so: ./target/release/libtokenizers_el.so
	cp ./target/release/libtokenizers_el.so tokenizers-core.so

./target/release/libtokenizers_el.so: ./src/lib.rs
	cargo build --release
