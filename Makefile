.PHONY: install clean

install:
	cargo install --path .

install-dev:
	cargo install --path . --debug

clean:
	rm -f Cargo.lock
	rm -rf target