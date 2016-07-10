
deps:
	git clone -b 1.0.0 --depth 1 https://github.com/nanomsg/nanomsg.git nanomsg-1.0.0
	cd nanomsg-1.0.0 && mkdir build && cd build && cmake .. && cmake --build .
	cd nanomsg-1.0.0/build && sudo cmake --build . --target install && sudo ldconfig --verbose

clean:
	rm -rf target
	rm -rf nanomsg-1.0.0
	rm 1.0.0.tar.gz

.PHONY: clean deps
