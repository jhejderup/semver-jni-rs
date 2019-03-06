java_run: lib
	javac CargoSemver.java && java -Djava.library.path=target/release/ CargoSemver

.PHONY: lib

javah:
	javah CargoSemver

lib:
	cargo build --release

clean:
	rm CargoSemver.class && cargo clean