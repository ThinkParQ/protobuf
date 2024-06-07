# force the usage of /bin/bash instead of /bin/sh
SHELL := /bin/bash

# Proto source file directory
SRC_DIR := proto
# Output directory for compiled Go files
GO_OUT_DIR := go
# Output directory for compiled C++ files
CPP_OUT_DIR := cpp
# Output directory for compiled Rust files
RUST_OUT_DIR := rust

all: protos

.PHONY: protos
protos:
	@mkdir -p $(GO_OUT_DIR) $(CPP_OUT_DIR) $(RUST_OUT_DIR)
	protoc -I $(SRC_DIR) \
		--go_opt=module="github.com/thinkparq/protobuf/go" \
		--go_out=$(GO_OUT_DIR) \
		--go-grpc_opt=module="github.com/thinkparq/protobuf/go" \
		--go-grpc_out=$(GO_OUT_DIR) \
		--cpp_out=$(CPP_OUT_DIR) \
		$(SRC_DIR)/*.proto
	protoc-rs compile -I $(SRC_DIR) --out=$(RUST_OUT_DIR) $(SRC_DIR)/*.proto
	@protoc-rs generate-crate --src=$(RUST_OUT_DIR)

# Test targets: 
# Test targets may make change to the local repository (e.g. try to generate protos) to
# verify all code required to build the project has been properly committed.
# Commonly this is done by running `make test` in CI, but could also be done locally.
# If you ran `make test` locally you may want to use `git reset` to revert the changes.
.PHONY: test test-protos
test: test-protos 

test-protos: protos
	@out="$$(git status --porcelain $$(find $(GO_OUT_DIR) $(CPP_OUT_DIR) $(RUST_OUT_DIR)))"; \
	if [ -n "$$out" ]; then \
		echo "Protobuf files are not up to date. Please run 'make protos' and commit the changes."; \
		echo "The following files are not up to date:"; \
		echo "$$out"; \
		exit 1; \
	fi

# Helper targets:

# Clean up
.PHONY: clean
clean:
	rm -rf $(GO_OUT_DIR) $(CPP_OUT_DIR) $(RUST_OUT_DIR)
	rm -rf target/
	rm -f Cargo.lock

# Install the specialized tools needed for building protobuf. Does not add the install directories
# to PATH.
.PHONY: install-tools
install-tools:
	sudo apt install --yes protobuf-compiler
	cargo install --git "https://github.com/thinkparq/protoc-rs"
	go get google.golang.org/protobuf/cmd/protoc-gen-go
	go install google.golang.org/protobuf/cmd/protoc-gen-go
	go get google.golang.org/grpc/cmd/protoc-gen-go-grpc
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc
	@echo "Tools installed. Make sure your PATH contains the install directories $$HOME/.cargo/bin and $$(go env GOPATH)/bin"
