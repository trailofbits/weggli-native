LDFLAGS := -L../target/release -lweggli_native

demo: demo.c weggli.h libweggli
	$(CC) $(LDFLAGS) -o $@ $<

.PHONY:
libweggli:
	cd .. && cargo build --release

.PHONY:
weggli.h:
	@# Annoying: cbindgen has to run in the crate root
	cd .. && cbindgen --config cbindgen.toml --crate weggli-native --output demo/$@
