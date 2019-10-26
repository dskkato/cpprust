.PHONY: all release

LIBS_debug		= -L./target/debug/deps -lrustlib
LIBS_release	= -L./target/release/deps -lrustlib

all:
	cargo build
	$(MAKE) --directory=cppmain
	$(CXX) $(LIBS) cppmain/main.o

release:
	cargo build --release
	$(MAKE) --directory=cppmain
	$(CXX) $(LIBS) cppmain/main.o

clean:
	cargo clean
	make clean --directory=cppmain
