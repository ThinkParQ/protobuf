# force the usage of /bin/bash instead of /bin/sh
SHELL := /bin/bash

# Build Targets
all: protos

.PHONY: clean

# Generate Go code from protobuf files:
PROTO_FILES := $(shell find ./. -name '*.proto')
PROTO_GENERATED_FILES := $(patsubst %.proto, %.pb.go, $(PROTO_FILES))

.PHONY: protos

protos: $(PROTO_GENERATED_FILES)

%.pb.go: %.proto
	@echo "Compiling $<"
	protoc -I $(dir $<) --go_out=$(dir $<) --go_opt=paths=source_relative --go-grpc_out=$(dir $<) --go-grpc_opt=paths=source_relative $<

# Test targets: 
# Test targets may make change to the local repository (e.g. try to generate protos) to
# verify all code required to build the project has been properly committed.
# Commonly this is done by running `make test` in CI, but could also be done locally.
# If you ran `make test` locally you may want to use `git reset` to revert the changes.
.PHONY: test test-protos
test: test-protos 

test-protos: protos
	@if [ -n "$$(git status --porcelain */pb/*)" ]; then \
		echo "Protobuf files are not up to date. Please run 'make protos' and commit the changes."; \
		exit 1; \
	fi

# Helper targets:
clean :
	rm -f bin/*
	rm -f */*.go

