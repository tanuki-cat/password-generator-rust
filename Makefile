platform-linux-x86 = x86_64-unknown-linux-gnu
platform-apple-x86 = x86_64-apple-darwin
platform-apple-arm = aarch64-apple-darwin

UNAME_S := $(shell uname -s)

.PHONY: release clean

release: clean
ifeq ($(UNAME_S),Darwin)
	@echo "==> build macOS"
	cargo build --release
	cargo build --target=$(platform-apple-arm) --release
	cargo build --target=$(platform-apple-x86) --release
endif

ifeq ($(UNAME_S),Linux)
	@echo "==> build Linux"
	cargo build --release
	cargo build --target=$(platform-linux-x86) --release
endif

clean:
	@echo "==> clean"
	cargo clean
	@rm -rf x86_64-* aarch64-*
