all: libnanomsg

libnanomsg: src/lib.rs
	mkdir -p target
	rustc -g src/lib.rs --out-dir target

nanocli: libnanomsg
	mkdir -p target
	rustc -g -Ltarget lib/bin.rs --out-dir target

test: src/lib.rs
	mkdir -p target
	rustc --test -g -Ltarget -Lnanomsg-0.4-beta/.libs src/lib.rs --out-dir target
	./target/nanomsg

deps:
	wget http://download.nanomsg.org/nanomsg-0.4-beta.tar.gz
	tar -xvzf nanomsg-0.4-beta.tar.gz
	cd nanomsg-0.4-beta && ./configure && make && sudo make install
	sudo ldconfig

clean:
	rm -rf target
	rm -rf nanomsg-0.4-beta
	rm nanomsg-0.4-beta.tar.gz

.PHONY: clean deps
