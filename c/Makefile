#Programs
CC=clang

#Flags
CFLAGS=-v -g -flto=thin
INCLUDE_FLAGS=-I ./src

.PHONY: clean clean_editor
.DEFAULT: use-sorter

use-sorter: src/main.o src/atom.o src/memory_helper.o
	$(CC) $(CFLAGS) -fuse-ld=lld --output $@ $^

src/main.o: src/main.c src/atom.h
	$(CC) $(CFLAGS) --compile --output $@ $<

src/atom.o: src/atom.c src/atom.h src/memory_helper.h
	$(CC) $(CFLAGS) --compile --output $@ $<

src/memory_helper.o: src/memory_helper.c src/memory_helper.h
	$(CC) $(CFLAGS) --compile --output $@ $<

clean: clean_editor
	rm -f use-sorter
	rm -f src/*.o

clean_editor:
	rm -f *~
	rm -f src/*~
