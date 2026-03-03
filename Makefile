CC     = gcc
CFLAGS = -O2 -Wall -Wextra -Wpedantic
LDLIBS = -lm

.PHONY: all clean

all: wvm

wvm: vm/wvm.c
	$(CC) $(CFLAGS) -o $@ $< $(LDLIBS)

clean:
	rm -f wvm
