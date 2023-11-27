KERNEL_VERSION := $(shell uname -r)
KERNEL_MAJOR := $(shell echo $(KERNEL_VERSION) | cut -d. -f1)
KERNEL_MINOR := $(shell echo $(KERNEL_VERSION) | cut -d. -f2)

FEATURE := $(shell echo "6.2 6.0 5.13 5.12 5.11 5.9 5.8 5.7 5.5 5.4" | tr ' ' '\n' | sort -Vr | awk -v kv="$(KERNEL_MAJOR).$(KERNEL_MINOR)" '{if ($$1 <= kv) {print $$1; exit}}')

build:
	cargo build --no-default-features --features kernel-$(FEATURE)

test:
	cargo test --no-default-features --features kernel-$(FEATURE)

clean:
	cargo clean
	rm -f ./src/syscall/bindings/bindgen.rs

