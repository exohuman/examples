all: libexampleC.a test


libexampleC.a: exampleC.o
	$(CC) -shared -o lib/libexampleC.so lib/exampleC.o
	ar ruv lib/libexampleC.a lib/exampleC.o
	ranlib lib/libexampleC.a


exampleC.o: src/exampleC.c
	mkdir -p lib
	$(CC) -g -c src/exampleC.c -o lib/exampleC.o


test: src/main.c libexampleC.a
	$(CC) -g -o test src/main.c -Isrc -Llib -lexampleC


clean:
	rm -fr lib/* test test.dSYM
