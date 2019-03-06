java_run: lib
	javac Crates.java && java -Djava.library.path=target/release/ Crates

.PHONY: lib

javah:
	javah Crates

lib:
	cargo build --release

clean:
	rm *.class && cargo clean