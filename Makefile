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
