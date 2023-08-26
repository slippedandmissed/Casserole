LIB_NAME=casserole
WEB_DIR=web
RUST_DIR=rust
MODULE_DST_DIR=$(WEB_DIR)/module/$(LIB_NAME)

.PHONY: $(MODULE_DST_DIR)/$(LIB_NAME).js

$(MODULE_DST_DIR)/$(LIB_NAME).js: $(MODULE_DST_DIR)
	cd $(RUST_DIR) && wasm-pack build --target web
	cp $(RUST_DIR)/pkg/* $(MODULE_DST_DIR)

$(MODULE_DST_DIR):
	mkdir -p $(MODULE_DST_DIR)

clean:
	rm -rf $(WEB_DIR)/modules
	rm -rf $(RUST_DIR)/pkg
	cd $(RUST_DIR) && cargo clean