NANO_SRC_DIR := /usr/cn/nanomsg/src
RUSTBINDGEN := /usr/cn/rust-bindgen/bindgen

NANO_HEADERS := \
$(NANO_SRC_DIR)/nn.h \
$(NANO_SRC_DIR)/pair.h \
$(NANO_SRC_DIR)/pubsub.h \
$(NANO_SRC_DIR)/tcp.h

all: libnanomsg

libnanomsg: src/lib.rs
	mkdir -p target
	rustc -g src/lib.rs --out-dir target

nanocli: libnanomsg
	mkdir -p target
	rustc -g -Ltarget lib/bin.rs --out-dir target

#nanoserv: libnanomsg
#	mkdir -p target
#	rustc -g -Ltarget samples/nanoserv.rs --out-dir target

test: src/lib.rs
	mkdir -p target
	rustc --test -g -Ltarget src/lib.rs --out-dir target
	./target/nanomsg

clean:
	rm -rf target

.PHONY: clean
