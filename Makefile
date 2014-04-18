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

test: src/lib.rs
	mkdir -p target
	rustc --test -g -Ltarget -Lnanomsg-0.3-beta/.libs src/lib.rs --out-dir target
	./target/nanomsg

deps:
	wget http://download.nanomsg.org/nanomsg-0.3-beta.tar.gz
	tar -xvzf nanomsg-0.3-beta.tar.gz
	cd nanomsg-0.3-beta && ./configure && make && sudo make install
	sudo ldconfig

clean:
	rm -rf target
	rm -rf nanomsg-0.3-beta
	rm nanomsg-0.3-beta.tar.gz

.PHONY: clean deps
