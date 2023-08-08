# force the usage of /bin/bash instead of /bin/sh
SHELL := /bin/bash

# Build Targets
all: protos

.PHONY: clean

# Generate Go code from protobuf files:
PROTO_FILES := $(shell find ./. -name '*.proto')
GO_GENERATED_FILES := $(patsubst %.proto, %.pb.go, $(PROTO_FILES))
CPP_PROTO_GENERATED_FILES := $(patsubst %.proto, %.pb.cc, $(PROTO_FILES))

.PHONY: protos

protos: $(GO_GENERATED_FILES) $(CPP_PROTO_GENERATED_FILES)

%.pb.go: %.proto
	@echo "Compiling Go $<"
	protoc -I $(dir $<) --go_out=$(dir $<) --go_opt=paths=source_relative --go-grpc_out=$(dir $<) --go-grpc_opt=paths=source_relative $<

%.pb.cc: %.proto
	@echo "Compiling C++ $<"
	protoc -I $(dir $<) --cpp_out=$(dir $<) $<

# Test targets: 
# Test targets may make change to the local repository (e.g. try to generate protos) to
# verify all code required to build the project has been properly committed.
# Commonly this is done by running `make test` in CI, but could also be done locally.
# If you ran `make test` locally you may want to use `git reset` to revert the changes.
.PHONY: test test-protos
test: test-protos 

test-protos: protos
	@out="$$(git status --porcelain $$(find . -name '*.pb.*'))"; \
	if [ -n "$$out" ]; then \
		echo "Protobuf files are not up to date. Please run 'make protos' and commit the changes."; \
		echo "The following files are not up to date:"; \
		echo "$$out"; \
		exit 1; \
	fi

# Helper targets:
clean :
	rm -f bin/*
	rm -f */*.go

