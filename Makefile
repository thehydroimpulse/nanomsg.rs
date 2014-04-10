
NANO_SRC_DIR := /usr/cn/nanomsg/src
RUSTBINDGEN := /usr/cn/rust-bindgen/bindgen

NANO_HEADERS := \
$(NANO_SRC_DIR)/nn.h \
$(NANO_SRC_DIR)/pair.h \
$(NANO_SRC_DIR)/pubsub.h \
$(NANO_SRC_DIR)/tcp.h

all: libnanomsg samples

libnanomsg:
	mkdir -p target
	rustc -g nanomsg.rs --out-dir target

samples: rustnano-samp cnano-samp

rustnano-samp: nanocli nanoserv

nanocli: libnanomsg
	mkdir -p target
	rustc -g -Ltarget samples/nanocli.rs --out-dir target

nanoserv: libnanomsg
	mkdir -p target
	rustc -g -Ltarget samples/nanoserv.rs --out-dir target

run: nanocli
	./nanocli

clean:
	rm -rf target

cnano-samp: build
	gcc -g -o build/clinano samples/clinano.c -lnanomsg -I${NANO_SRC_DIR}
	gcc -g -o build/servnano samples/servnano.c -lnanomsg -I${NANO_SRC_DIR}

.PHONY: clean