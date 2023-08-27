LIB_NAME=casserole
BIN_NAME=casserole

SRC_DIR=src
CORE_SRC_DIR=$(SRC_DIR)/core
WASM_FRONTEND_SRC_DIR=$(SRC_DIR)/wasm-frontend
WEB_SRC_DIR=$(SRC_DIR)/web
SDL2_SRC_DIR=$(SRC_DIR)/sdl2-frontend

BUILD_DIR=build
CORE_BUILD_DIR=$(BUILD_DIR)/core
CORE_LIB_BUILD_DIR=$(CORE_BUILD_DIR)/lib
WEB_BUILD_DIR=$(BUILD_DIR)/web
SDL2_BUILD_DIR=$(BUILD_DIR)/sdl2

WASM_MODULE_DST_DIR=$(WEB_BUILD_DIR)/module/$(LIB_NAME)
WASM_TARGET_DST_DIR=$(BUILD_DIR)/wasm

BIN_EXTENSION=
ifeq ($(OS),Windows_NT)
	BIN_EXTENSION+=.exe
endif

DOWNLOADS_DIR=downloads
SDL2_DEVEL_VERSION=2.28.2

all: wasm sdl2

.PHONY: run core wasm sdl2 always clean clean_build clean_downloads

ifeq ($(OS),Windows_NT)
run: $(SDL2_SRC_DIR)/gnu-mingw/dll/32/SDL2.dll
	cd $(SDL2_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(CORE_LIB_BUILD_DIR)} cargo run
else
run:
	cd $(SDL2_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(CORE_LIB_BUILD_DIR)} cargo run
endif


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

ifeq ($(OS),Windows_NT)
sdl2: $(SDL2_SRC_DIR)/gnu-mingw/dll/32/SDL2.dll $(SDL2_BUILD_DIR)/debug/$(BIN_NAME)$(BIN_EXTENSION) $(SDL2_BUILD_DIR)/release/$(BIN_NAME)$(BIN_EXTENSION)
else
sdl2: $(SDL2_BUILD_DIR)/debug/$(BIN_NAME)$(BIN_EXTENSION) $(SDL2_BUILD_DIR)/release/$(BIN_NAME)$(BIN_EXTENSION)
endif

$(SDL2_BUILD_DIR)/debug/$(BIN_NAME)$(BIN_EXTENSION):
	cd $(SDL2_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(SDL2_BUILD_DIR)} cargo build

$(SDL2_BUILD_DIR)/release/$(BIN_NAME)$(BIN_EXTENSION):
	cd $(SDL2_SRC_DIR) && CARGO_TARGET_DIR=${abspath $(SDL2_BUILD_DIR)} cargo build --release

$(SDL2_SRC_DIR)/gnu-mingw/dll/32/SDL2.dll: $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw.tar.gz $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC.zip
	mkdir -p $(SDL2_SRC_DIR)/gnu-mingw/dll/32
	mkdir -p $(SDL2_SRC_DIR)/gnu-mingw/dll/64
	mkdir -p $(SDL2_SRC_DIR)/gnu-mingw/lib/32
	mkdir -p $(SDL2_SRC_DIR)/gnu-mingw/lib/64
	mkdir -p $(SDL2_SRC_DIR)/msvc/dll/32
	mkdir -p $(SDL2_SRC_DIR)/msvc/dll/64
	mkdir -p $(SDL2_SRC_DIR)/msvc/lib/32
	mkdir -p $(SDL2_SRC_DIR)/msvc/lib/64
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw/SDL2-$(SDL2_DEVEL_VERSION)/i686-w64-mingw32/bin/* $(SDL2_SRC_DIR)/gnu-mingw/dll/32
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw/SDL2-$(SDL2_DEVEL_VERSION)/x86_64-w64-mingw32/bin/* $(SDL2_SRC_DIR)/gnu-mingw/dll/64
	cp -r -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw/SDL2-$(SDL2_DEVEL_VERSION)/i686-w64-mingw32/lib/* $(SDL2_SRC_DIR)/gnu-mingw/lib/32
	cp -r -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw/SDL2-$(SDL2_DEVEL_VERSION)/x86_64-w64-mingw32/lib/* $(SDL2_SRC_DIR)/gnu-mingw/lib/64
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC/SDL2-$(SDL2_DEVEL_VERSION)/lib/x86/*.dll $(SDL2_SRC_DIR)/msvc/dll/32
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC/SDL2-$(SDL2_DEVEL_VERSION)/lib/x64/*.dll $(SDL2_SRC_DIR)/msvc/dll/64
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC/SDL2-$(SDL2_DEVEL_VERSION)/lib/x86/*.lib $(SDL2_SRC_DIR)/msvc/lib/32
	cp -f $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC/SDL2-$(SDL2_DEVEL_VERSION)/lib/x64/*.lib $(SDL2_SRC_DIR)/msvc/lib/64

$(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw.tar.gz:
	mkdir -p $(DOWNLOADS_DIR)
	wget https://github.com/libsdl-org/SDL/releases/download/release-$(SDL2_DEVEL_VERSION)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw.tar.gz -P $(DOWNLOADS_DIR)
	mkdir -p $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw
	tar -xvf $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw.tar.gz -C $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-mingw

$(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC.zip:
	mkdir -p $(DOWNLOADS_DIR)
	wget https://github.com/libsdl-org/SDL/releases/download/release-$(SDL2_DEVEL_VERSION)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC.zip -P $(DOWNLOADS_DIR)
	mkdir -p $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC
	unzip -o $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC.zip -d $(DOWNLOADS_DIR)/SDL2-devel-$(SDL2_DEVEL_VERSION)-VC

always:
	mkdir -p $(BUILD_DIR)

clean_build:
	rm -rf $(BUILD_DIR)/*
	rm -rf $(CORE_SRC_DIR)/target
	rm -rf $(WASM_FRONTEND_SRC_DIR)/target

clean_downloads:
	rm -rf $(DOWNLOADS_DIR)/*
	rm -rf $(SDL2_SRC_DIR)/gnu-mingw
	rm -rf $(SDL2_SRC_DIR)/msvc

clean: clean_build clean_downloads