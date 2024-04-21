java_run: lib
	javac JavaCalcTest.java && java -Djava.library.path=../target/debug/ JavaCalcTest

.PHONY: lib

lib:
	cd javacalclib && cargo build
