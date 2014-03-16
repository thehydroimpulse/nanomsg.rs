
NANO_SRC_DIR := /usr/cn/nanomsg/src
RUSTBINDGEN := /usr/cn/rust-bindgen/bindgen

NANO_HEADERS := \
$(NANO_SRC_DIR)/nn.h \
$(NANO_SRC_DIR)/pair.h \
$(NANO_SRC_DIR)/pubsub.h \
$(NANO_SRC_DIR)/tcp.h


all: libnanomsg samples

libnanomsg:
	rustc -g nanomsg.rs

samples: rustnano-samp cnano-samp

rustnano-samp: nanocli nanoserv

nanocli: libnanomsg
	rustc -g -L . samples/nanocli.rs

nanoserv: libnanomsg
	rustc -g -L . samples/nanoserv.rs

run: nanocli
	./nanocli

clean: clean-samples

clean-samples:
	rm -f build/nanocli build/nanoserv build/clinano build/servnano
	rmdir build

cnano-samp: build
	gcc -g -o build/clinano samples/clinano.c -lnanomsg -I${NANO_SRC_DIR}
	gcc -g -o build/servnano samples/servnano.c -lnanomsg -I${NANO_SRC_DIR}

build:
	mkdir -p $@
