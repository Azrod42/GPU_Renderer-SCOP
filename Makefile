all: run

run:
	cargo run

c: check

check:
	cargo check

build:
	cargo build --release
	cp ./target/release/scop .
	./scop

clean:
	rm -rf ./target/

fclean: clean 
	rm ./scop
