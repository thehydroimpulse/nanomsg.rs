deps:
	wget https://github.com/nanomsg/nanomsg/releases/download/0.6-beta/nanomsg-0.6-beta.tar.gz
	wget https://github.com/nanomsg/nanomsg/releases/download/0.8-beta/nanomsg-0.8-beta.tar.gz -O nanomsg.tar.gz
	tar -xzf nanomsg.tar.gz
	mv nanomsg-0.8-beta nanomsg
	cd nanomsg && ./configure && make && sudo make install
	sudo ldconfig

clean:
	rm -rf target
	rm -rf nanomsg
	rm nanomsg.tar.gz

.PHONY: clean deps
