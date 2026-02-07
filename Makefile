DESTDIR="/usr/bin/"

.PHONY: build
build:
	odin build .

.PHONY: install
install: build
	install ./limine-mkconfig $(DESTDIR)

.PHONY: clean
clean:
	[ -f ./limine-mkconfig ] && rm ./limine-mkconfig || echo
