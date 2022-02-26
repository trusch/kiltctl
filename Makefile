CHAIN_URL=wss://spiritnet.kilt.io:443

bin/kiltctl: target/release/kiltctl
	mkdir -p bin 
	cp target/release/kiltctl bin/kiltctl

target/release/kiltctl: metadata.scale $(shell find ./src -name "*.rs")
	cargo build --release

test: metadata.scale
	cargo clippy
	cargo test

install: bin/kiltctl
	cp bin/kiltctl /usr/local/bin/kiltctl

docs:
	cargo doc --open

metadata.scale:
	echo '{"jsonrpc":"2.0","id":"1","method":"state_getMetadata"}' | \
		websocat -B 655350 $(CHAIN_URL) | \
		jq -r .result | \
		xxd -r -p \
		> metadata.scale
