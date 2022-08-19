BUILD_DIR ?= target
PREFIX_INSTALL ?= /
BUILD_TYPE ?= release
BINARY := apmpkg

$(BUILD_DIR):
	cargo build --target-dir $(BUILD_DIR) $(CFLAGS)

install: $(BUILD_DIR)
	mkdir -p $(PREFIX_INSTALL)
	install -Dm755 $(BUILD_DIR)/$(BUILD_TYPE)/$(BINARY) $(PREFIX_INSTALL)/usr/bin/$(BINARY)
	mkdir -p $(PREFIX_INSTALL)/etc/$(BINARY)/iiabc
	cp -r src/iiabc $(PREFIX_INSTALL)/etc/$(BINARY)
	mkdir -p $(PREFIX_INSTALL)/etc/$(BINARY)/paquetes
	install -Dm 644 "man/$(BINARY).1" -t $(PREFIX_INSTALL)/usr/share/man/man1
	install -Dm 644 "man/$(BINARY)-en.1" -t \
		$(PREFIX_INSTALL)/usr/share/man/man1
	install -Dm 644 "completions/_$(BINARY)" -t \
		$(PREFIX_INSTALL)/usr/share/zsh/site-functions
	install -Dm 644 "completions/$(BINARY).bash-completion" -t \
		$(PREFIX_INSTALL)/usr/share/bash-completion/completions
	install -Dm 644 "completions/$(BINARY).fish" -t \
		$(PREFIX_INSTALL)/usr/share/fish/vendor_completions.d/

clean:
	rm -rf $(BUILD_DIR)

cleaninstall:
	rm -f $(PREFIX_INSTALL)/usr/bin/$(BINARY)
	rm -rf $(PREFIX_INSTALL)/etc/$(BINARY)
	rm -f $(PREFIX_INSTALL)/usr/share/man/man1/$(BINARY)*
	rm -f $(PREFIX_INSTALL)/usr/share/zsh/site-functions/_$(BINARY)
	rm -f $(PREFIX_INSTALL)/usr/share/bash-completion/completions/$(BINARY).bash-completion
	rm -f $(PREFIX_INSTALL)/usr/share/fish/vendor_completions.d/$(BINARY).fish

test:
	cargo test --target-dir $(BUILD_DIR) $(CFLAGS)

.PHONY: clean test install cleaninstall
