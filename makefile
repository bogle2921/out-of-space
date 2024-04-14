NAME=out_of_space
WASM_FILE=$(NAME).wasm
JS_FILE=$(NAME).js
HTML_FILE=index.html
FINAL_BUNDLE=$(NAME).zip
WASM_TARGET_OUT=target/wasm32-unknown-unknown/release/out_of_space.wasm

all: $(FINAL_BUNDLE)

$(WASM_TARGET_OUT): src/main.rs
	cargo build --release --target wasm32-unknown-unknown

$(WASM_FILE) $(JS_FILE): $(WASM_TARGET_OUT)
	wasm-bindgen --no-typescript --target web \
	    --out-dir ./ \
	    --out-name "$(NAME)" \
	    $<
	cp target/wasm32-unknown-unknown/release/out_of_space.wasm $(WASM_FILE)

$(FINAL_BUNDLE): $(WASM_FILE) $(JS_FILE) $(HTML_FILE)
	zip -r $(FINAL_BUNDLE) assets $(JS_FILE) $(WASM_FILE) $(HTML_FILE)

clean:
	rm -rf $(FINAL_BUNDLE) $(JS_FILE) *.wasm imported_assets

.PHONY: all clean
.NOTPARALLEL: $(WASM_FILE) $(JS_FILE)