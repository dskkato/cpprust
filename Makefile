.PHONY: all release

all:
	cargo build
	$(MAKE) --directory=cppmain

release:
	cargo build --release
	$(MAKE) --directory=cppmain
