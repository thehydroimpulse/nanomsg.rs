
deps:
	wget https://github.com/nanomsg/nanomsg/archive/0.9-beta.tar.gz 
	tar -xvzf 0.9-beta.tar.gz
	cd nanomsg-0.9-beta && mkdir build && cd build && cmake .. && cmake --build .
	cd nanomsg-0.9-beta/build && sudo cmake --build . --target install

clean:
	rm -rf target
	rm -rf nanomsg-0.9-beta
	rm 0.9-beta.tar.gz

.PHONY: clean deps
