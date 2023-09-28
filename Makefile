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

# Generate language specific files from protobuf files:
# Find all .proto files relative to $(SRC_DIR)
PROTO_FILES := $(shell cd $(SRC_DIR) && find . -name '*.proto')
# Add the language specific output directory as a prefix and replace .proto file suffix by the
# language specific one
GO_GENERATED_FILES := $(patsubst %.proto, $(GO_OUT_DIR)/%.pb.go, $(PROTO_FILES))
CPP_PROTO_GENERATED_FILES := $(patsubst %.proto, $(CPP_OUT_DIR)/%.pb.cc, $(PROTO_FILES))
RUST_PROTO_GENERATED_FILES := $(patsubst %.proto, $(RUST_OUT_DIR)/%.rs, $(PROTO_FILES))

all: protos

# Builds all language specific files
.PHONY: protos
protos: $(GO_GENERATED_FILES) $(CPP_PROTO_GENERATED_FILES) $(RUST_PROTO_GENERATED_FILES)
	@# Generate Rust library crate structure
	@protoc-rs generate-crate --src=$(RUST_OUT_DIR)


# Build into Go
$(GO_OUT_DIR)/%.pb.go: $(SRC_DIR)/%.proto
	@echo "Compiling Go $<"
	@mkdir -p $(dir $@)
	@# In the protoc command the second `-I.` is required for imports to work correctly. 
	@# It is also import the `-I.` is in the second position otherwise protoc will generate files at a location like `go/beewatch/beewatch`
	protoc -I $(dir $<) -I $(SRC_DIR) --go_out=$(dir $@) --go_opt=paths=source_relative --go-grpc_out=$(dir $@) --go-grpc_opt=paths=source_relative $<

# Build into C++
$(CPP_OUT_DIR)/%.pb.cc: $(SRC_DIR)/%.proto
	@echo "Compiling C++ $<"
	@mkdir -p $(dir $@)
	protoc -I $(dir $<) -I $(SRC_DIR) --cpp_out=$(dir $@) $<

# Build into Rust
$(RUST_OUT_DIR)/%.rs: $(SRC_DIR)/%.proto
	@echo "Compiling Rust $<"
	@mkdir -p $(dir $@)
	protoc-rs compile -I $(dir $<) -I $(SRC_DIR) --out=$(dir $@) $<

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
	go install google.golang.org/protobuf/cmd/protoc-gen-go
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc
	@echo "Tools installed. Make sure your PATH contains the install directories $$HOME/.cargo/bin and $$(go env GOPATH)/bin"
