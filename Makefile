LIB_NAME=casserole

SRC_DIR=src
CORE_SRC_DIR=$(SRC_DIR)/core
WASM_FRONTEND_SRC_DIR=$(SRC_DIR)/wasm-frontend
WEB_SRC_DIR=$(SRC_DIR)/web

BUILD_DIR=build
CORE_BUILD_DIR=$(BUILD_DIR)/core
CORE_LIB_BUILD_DIR=$(CORE_BUILD_DIR)/lib
WEB_BUILD_DIR=$(BUILD_DIR)/web

WASM_MODULE_DST_DIR=$(WEB_BUILD_DIR)/module/$(LIB_NAME)
WASM_TARGET_DST_DIR=$(BUILD_DIR)/wasm

all: wasm

.PHONY: core wasm always clean clean_build

core: $(CORE_LIB_BUILD_DIR)/debug/lib$(LIB_NAME).so $(CORE_LIB_BUILD_DIR)/release/lib$(LIB_NAME).so

$(CORE_LIB_BUILD_DIR)/debug/lib$(LIB_NAME).so: always
	cd $(CORE_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(CORE_LIB_BUILD_DIR)} cargo build

$(CORE_LIB_BUILD_DIR)/release/lib$(LIB_NAME).so: always
	cd $(CORE_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(CORE_LIB_BUILD_DIR)} cargo build --release

wasm: $(WASM_MODULE_DST_DIR)/$(LIB_NAME).js

$(WASM_MODULE_DST_DIR)/$(LIB_NAME).js: $(WASM_MODULE_DST_DIR) always
	cd $(WASM_FRONTEND_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(WASM_TARGET_DST_DIR)} wasm-pack build --target web --out-dir ${abspath $(WASM_MODULE_DST_DIR)}

$(WASM_MODULE_DST_DIR): always
	mkdir -p $(WEB_BUILD_DIR)
	cp -r $(WEB_SRC_DIR)/* $(WEB_BUILD_DIR)/
	mkdir -p $(WASM_MODULE_DST_DIR)

always:
	mkdir -p $(BUILD_DIR)

clean_build:
	rm -rf $(BUILD_DIR)/*
	rm -rf $(CORE_SRC_DIR)/target
	rm -rf $(WASM_FRONTEND_SRC_DIR)/target

clean: clean_build