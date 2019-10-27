.PHONY: all release

LIBS_debug		= -L./target/debug -lrustlib
LIBS_release	= -L./target/release -lrustlib

all:
	cargo build
	$(MAKE) --directory=cppmain
	$(CXX) $(LIBS_debug) cppmain/main.o

release:
	cargo build --release
	$(MAKE) --directory=cppmain
	$(CXX) $(LIBS_release) cppmain/main.o

clean:
	cargo clean
	make clean --directory=cppmain
	rm -f a.out
