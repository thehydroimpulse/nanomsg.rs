
NANO_SRC_DIR := /usr/cn/nanomsg/src
RUSTBINDGEN := /usr/cn/rust-bindgen/bindgen

NANO_HEADERS := \
$(NANO_SRC_DIR)/nn.h \
$(NANO_SRC_DIR)/pair.h \
$(NANO_SRC_DIR)/pubsub.h \
$(NANO_SRC_DIR)/tcp.h


all: libnanomsg samples

libnanomsg:
	rust build -Z debug-info nanomsg.rs

samples: rustnano-samp cnano-samp

rustnano-samp: nanocli nanoserv

nanocli: libnanomsg
	rust build -Z debug-info -L . samples/nanocli.rs

nanoserv: libnanomsg
	rust build -Z debug-info -L . samples/nanoserv.rs

run:
	rust run nanocli.rs

clean: clean-samples

clean-samples:
	rm -f samples/nanocli samples/nanoserv samples/clinano samples/servnano

cnano-samp:
	gcc -g -o clinano samples/clinano.c -lnanomsg -I${NANO_SRC_DIR}
	gcc -g -o servnano samples/servnano.c -lnanomsg -I${NANO_SRC_DIR}
