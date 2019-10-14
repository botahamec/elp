
/usr/bin/elp: target/release/elp
	sudo cp target/release/elp /usr/bin/elp
	sudo chmod u+x /usr/bin/elp

target/release/elp: src/main.rs
	cargo build --release

clean:
	rm -rf target/release