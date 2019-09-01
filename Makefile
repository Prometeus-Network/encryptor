start_rust_install: rust_install dependecies app_run

start: dependecies app_run

dependecies:
	sudo apt-get install pkg-config libssl-dev

app_run:
	cargo build
	cargo run

rust_install:
	curl https://sh.rustup.rs -sSf | sh