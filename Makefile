
deps:
	wget https://github.com/nanomsg/nanomsg/archive/1.0.0.tar.gz
	tar -xvzf 1.0.0.tar.gz
	cd nanomsg-1.0.0 && mkdir build && cd build && cmake .. && cmake --build .
	cd nanomsg-1.0.0/build && sudo cmake --build . --target install

clean:
	rm -rf target
	rm -rf nanomsg-1.0.0
	rm 1.0.0.tar.gz

.PHONY: clean deps
