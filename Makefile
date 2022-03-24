all: install

install:
	cargo build --release
	sudo cp target/release/lsd-print /usr/bin/lsd-print

uninstall:
	sudo rm -f /usr/bin/lsd-print