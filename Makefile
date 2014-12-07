deps:
	wget http://download.nanomsg.org/nanomsg-0.5-beta.tar.gz
	tar -xvzf nanomsg-0.5-beta.tar.gz
	cd nanomsg-0.5-beta && ./configure && make && sudo make install
	sudo ldconfig

clean:
	rm -rf target
	rm -rf nanomsg-0.5-beta
	rm nanomsg-0.5-beta.tar.gz

.PHONY: clean deps
