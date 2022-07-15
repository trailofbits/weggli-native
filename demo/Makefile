LDFLAGS := -L../target/release -lweggli_native

UNAME := $(shell uname)
ifeq ($(UNAME),Linux)
	LDFLAGS := $(LDFLAGS) -Wl,-R../target/release
endif

demo: demo.c weggli.h libweggli
	$(CC) -o $@ $< $(LDFLAGS)

.PHONY:
libweggli:
	cd .. && cargo build --release

.PHONY:
weggli.h: libweggli
	@# Annoying: cbindgen has to run in the crate root
	cd .. && cbindgen --config cbindgen.toml --crate weggli-native --output demo/$@
