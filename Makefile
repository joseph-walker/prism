SHELL := bash

# Build the application
build: target/release/prism
.PHONY: build

# Build the debug target
build-debug: target/debug/prism
.PHONY: build-debug

# Nuke all built artifacts
clean: clean-js clean-rs
.PHONY: clean

# Delete JS artifacts
clean-js:
	rm -rf client/dist
.PHONY: clean-js

# Delete Rust artifacts
clean-rs:
	rm -rf target
.PHONY: clean-rs

# Install Node Modules when package.json changes
client/node_modules/.npm-install.sentinel: client/package.json
	cd client && npm install
	touch $@

# Build the UI
client/dist/index.html: client/index.html \
	client/node_modules/.npm-install.sentinel \
	$(shell find client/src -type f) \
	$(shell find client/node_modules -name package.json)
	cd client && npm run build

# Prod build
target/release/prism: client/dist/index.html
	cargo build -r

# Debug build
target/debug/prism: client/dist/index.html
	cargo build
