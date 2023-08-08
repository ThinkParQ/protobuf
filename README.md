# BeeGFS Protocol Buffers <!-- omit in toc -->

# Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Quick Start](#quick-start)
  - [Go](#go)
- [Advanced: Generate / Compile .proto files](#advanced-generate--compile-proto-files)
  - [Prerequisites](#prerequisites)
  - [Generating Code](#generating-code)
  - [Generating Code for Golang](#generating-code-for-golang)
- [References](#references)

# Overview 

This repository contains the [Protocol Buffer](https://protobuf.dev/overview/)
and gRPC service definitions (as `.proto` files) for a number of projects in the
BeeGFS ecosystem. These `.proto` files are used to generate code that allows the
data structures and gRPC clients and servers to be easily used from a number of
languages. Generated code for select languages is also provided through this
repository, or users can generate code themselves using the `protoc` tool and
any language specific plugins.

Languages with precompiled/generated code:

| Language | Availability | Notes                                                                                      |
| -------- | ------------ | ------------------------------------------------------------------------------------------ |
| Go       | Yes          | N/A                                                                                        |
| C++      | Yes          | A target has been added to the Makefile, but the resulting files have not been tested yet. |

# Quick Start

Most users will not need to worry about generating code themselves, and can
simply import this repository into their project using whatever method is
appropriate for their language. 

## Go

As long as this repository is private, some additional configuration is required
to allow this repository to be imported as a private module into your Go
project. From there you can use Go Modules and all of the normal tools such as
`go get`, `go mod tidy`, and `go mod vendor` to manage the `bee-protos` as a
dependency of your project. 

IMPORTANT: These steps presume you are using SSH and the SSH key for the machine
you're on is already added to your GitHub account and has access to this
repository. 

Steps: 

(1) In your .bashrc/similar set the GOPRIVATE variable so the Go tools will
access the repository directly instead of going through the central services
(which could potentially leak information, such as the project name):
```shell
export GOPRIVATE=github.com/thinkparq/bee-protos
```
(2) Add the following to your global `.gitconfig` file to ensure `go get` will
use SSH instead of HTTP:
```shell
[url "ssh://git@github.com/"]
	insteadOf = https://github.com/
```
(3) From here you can add any of the packages from bee-protos as a dependency
with `go get`. For example to use BeeWatch in your project run `go get
github.com/thinkparq/bee-protos/beewatch`. When changes are made to the
`bee-protos` project you can update dependencies with `go get -u <URL>` 

# Advanced: Generate / Compile .proto files

For convenience, pregenerated/compiled code for popular language is provided in
this repository. This means most users should only need to follow the steps in
the Quick Start section. If you are needing to make changes to the `.proto`
files or generate code for a new language, the steps in this section are
intended for you.

When working with protobuf files and vscode it is recommended to install [this
extension](https://marketplace.visualstudio.com/items?itemName=zxh404.vscode-proto3).

## Prerequisites

* Install the Protocol buffer compiler `protoc` version 3.
  * Generally it is recommended to install the [pre-compiled
    binaries](https://grpc.io/docs/protoc-installation/#install-pre-compiled-binaries-any-os)
    instead of installing the version from your package manager as it is likely
    out of date.
* Install any language specific plugins, including anything required to work
  with gRPC.
  * For example [for Go](https://grpc.io/docs/languages/go/quickstart/) you need
    `protoc-gen-go` and `protoc-gen-go-grpc`.
* Ensure all the necessary binaries are added to your `$PATH` variable using
  `.bashrc` or similar.

## Generating Code

The project Makefile is setup with targets to compile all protobuf files in the
project into a multiple target languages when `make` or `make protos` is
executed.

To add a new language amend the Makefile as follows:

1. Add a new variable `<LANGUAGE>_GENERATED_FILES` that transforms the `.proto`
   file names returned by `PROTO_FILES` into corresponding files to generate for
   the target language.
   1. For example for Go this looks like: `GO_GENERATED_FILES := $(patsubst
      %.proto, %.pb.go, $(PROTO_FILES))`.
2. Modify the `protos` target to add the generated list of files as a
   dependency.
3. Create a new pattern rule for the target language to generate the appropriate
   source files from the `.proto` files with the correct `protoc` command for
   the new language. 
   1. For example for Go this rule looks like: 
```Makefile
%.pb.go: %.proto
	@echo "Compiling Go $<"
	protoc -I $(dir $<) --go_out=$(dir $<) --go_opt=paths=source_relative --go-grpc_out=$(dir $<) --go-grpc_opt=paths=source_relative $<
```

## Generating Code for Golang

If you plan to make changes to the protocol buffers (.proto files) or simply
want to manually generate the code you will need to
[install](https://grpc.io/docs/languages/go/quickstart/) the Protocol Buffer
compiler v3 (protoc) and Go plugins for the protocol compiler. Whenever changes
are made to the `*.protos` files ensure to run `make protos` and commit the
changes to the repository. 

# References

* [Style Guide](https://protobuf.dev/programming-guides/style/)
* [Go Generated Code Guide](https://protobuf.dev/reference/go/go-generated/)