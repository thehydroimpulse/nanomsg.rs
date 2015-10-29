deps:
	wget https://github.com/nanomsg/nanomsg/releases/download/0.7-beta/nanomsg-0.7-beta.tar.gz
	tar -xvzf nanomsg-0.7-beta.tar.gz
	cd nanomsg-0.7-beta && ./configure && make && sudo make install
	sudo ldconfig

clean:
	rm -rf target
	rm -rf nanomsg-0.7-beta
	rm nanomsg-0.7-beta.tar.gz

.PHONY: clean deps
