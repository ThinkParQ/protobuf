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
protos: check-tools
	mkdir -p $(GO_OUT_DIR) $(CPP_OUT_DIR) $(RUST_OUT_DIR)
	protoc -I $(SRC_DIR) \
		--go_opt=module="github.com/thinkparq/protobuf/go" \
		--go_opt=default_api_level=API_HYBRID \
		--go_out=$(GO_OUT_DIR) \
		--go-grpc_opt=module="github.com/thinkparq/protobuf/go" \
		--go-grpc_out=$(GO_OUT_DIR) \
		--cpp_out=$(CPP_OUT_DIR) \
		$(SRC_DIR)/*.proto
	protoc-rs -I $(SRC_DIR) --out=$(RUST_OUT_DIR) $(SRC_DIR)/*.proto


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


# The tools versions we want to use
PROTOC_VERSION := 29.2
PROTOC_GEN_GO_VERSION := 1.36.2
PROTOC_GEN_GO_GRPC_VERSION := 1.5.1
PROTOC_RS_VERSION := 0.5.0

# Checks the versions of the installed tools, making sure they are what we expect
.PHONY: check-tools
.ONESHELL: check-tools
check-tools:
	@function check() { [[ "$$1" == "$$2" ]] || { echo "tool version mismatch: expected $$1, got $$2" ; exit 1 ; } }
	check "libprotoc $(PROTOC_VERSION)" "$$(protoc --version)"
	check "protoc-gen-go v$(PROTOC_GEN_GO_VERSION)" "$$(protoc-gen-go --version)"
	check "protoc-gen-go-grpc $(PROTOC_GEN_GO_GRPC_VERSION)" "$$(protoc-gen-go-grpc --version)"
	check "protoc-rs $(PROTOC_RS_VERSION)" "$$(protoc-rs --version)"

# Installs the specialized tools needed for building protobuf on an x86_64 machine to the user
# program files directory $HOME/.local/bin . If that directory is not part of $PATH yet, the user
# must add it manually.
# Overwrites existing tools with a newer version. Requires curl, cargo, go.
.PHONY: install-tools
.ONESHELL: install-tools
install-tools:
	@set -e

	ARCH=$$(uname -m)
	if [[ "$${ARCH}" == "aarch64" ]]; then
		ARCH=aarch_64
	fi

	(
		set -x
		# protoc
		curl -LfsSo /tmp/protoc.zip https://github.com/protocolbuffers/protobuf/releases/download/v$(PROTOC_VERSION)/protoc-$(PROTOC_VERSION)-linux-$${ARCH}.zip
		rm -rf "$${HOME}/.local/bin/protoc" "$${HOME}/.local/include/google/protobuf"
		unzip -o -q -d "$${HOME}/.local" /tmp/protoc.zip "bin/protoc" "include/google/protobuf/*"
		# other tools
		GOBIN="$${HOME}/.local/bin" go install google.golang.org/protobuf/cmd/protoc-gen-go@v$(PROTOC_GEN_GO_VERSION)
		GOBIN="$${HOME}/.local/bin" go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v$(PROTOC_GEN_GO_GRPC_VERSION)
		cargo install --root "$${HOME}/.local" --git "https://github.com/thinkparq/protoc-rs" --tag "v$(PROTOC_RS_VERSION)" --locked
	)

	echo ""; echo "Tools installed. Make sure your PATH contains the install directory $${HOME}/.local/bin"
