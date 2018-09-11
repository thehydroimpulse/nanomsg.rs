
deps:
	git clone -b 1.1.4 --depth 1 https://github.com/nanomsg/nanomsg.git nanomsg-1.1.4
	cd nanomsg-1.1.4 && mkdir build && cd build && cmake .. && cmake --build .
	cd nanomsg-1.1.4/build && sudo cmake --build . --target install && sudo ldconfig --verbose

clean:
	rm -rf target
	rm -rf nanomsg-1.1.4
	rm 1.1.4.tar.gz

.PHONY: clean deps
