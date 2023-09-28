# BeeGFS Protocol Buffers <!-- omit in toc -->

# Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Quick Start](#quick-start)
  - [Go](#go)
- [Advanced: Generate / Compile `.proto` Files](#advanced-generate--compile-proto-files)
  - [Prerequisites](#prerequisites)
  - [Generating Code for a New Language](#generating-code-for-a-new-language)
  - [Generating Code for Golang](#generating-code-for-golang)
- [References](#references)

# Overview 

This repository contains the [Protocol Buffer](https://protobuf.dev/overview/)
and gRPC service definitions (as `.proto` files) for a number of projects in the
BeeGFS ecosystem. These `.proto` files are used to generate code that allows the
data structures and gRPC clients and servers to be easily used and implemented
in a number of languages. Generated code for select languages is also provided
alongside each `.proto` file organized into per-language sub-directories.
Alternatively users can generate code themselves using the `protoc` tool and any
language specific plugins.

Languages with precompiled/generated code:

| Language | Availability | Notes                                                                                      |
| -------- | ------------ | ------------------------------------------------------------------------------------------ |
| Go       | Yes          | N/A                                                                                        |
| C++      | Yes          | A target has been added to the Makefile, but the resulting files have not been tested yet. |
| Rust     | Yes          | A target has been added to the Makefile, but the resulting files have not been tested yet. |

# Quick Start

Most users will not need to worry about generating code themselves, and can
simply import this repository into their project using whatever method is
appropriate for their language.

## Go

As long as this repository is private, some additional configuration is required
to allow this repository to be imported as a private module into your Go
project. From there you can use Go Modules and all of the normal tools such as
`go get`, `go mod tidy`, and `go mod vendor` to manage the `protobuf` as a
dependency of your project. 

IMPORTANT: These steps presume you are using SSH and the SSH key for the machine
you're on is already added to your GitHub account and has access to this
repository. 

Steps: 

(1) In your .bashrc/similar set the GOPRIVATE variable so the Go tools will
access the repository directly instead of going through the central services
(which could potentially leak information, such as the project name):
```shell
export GOPRIVATE=github.com/thinkparq/protobuf/go
```
(2) Add the following to your global `.gitconfig` file to ensure `go get` will
use SSH instead of HTTP:
```shell
[url "ssh://git@github.com/"]
	insteadOf = https://github.com/
```
(3) From here you can add any of the packages from protobuf as a dependency
with `go get`. For example to use BeeWatch in your project run `go get
github.com/thinkparq/protobuf/go/beewatch`. When changes are made to the
`protobuf` project you can update dependencies with `go get -u <URL>`

# Advanced: Generate / Compile `.proto` Files

For convenience, pregenerated/compiled code for popular language is provided in
this repository. This means most users should only need to follow the steps in
the Quick Start section. If you are needing to make changes to the `.proto`
files or generate code for a new language, the steps in this section are
intended for you.

When working with protobuf files and vscode it is recommended to install [this
extension](https://marketplace.visualstudio.com/items?itemName=zxh404.vscode-proto3).
Especially if you are using a workspace, you will need to add the absolute path
to the protobuf project in either your user or workspace `settings.json` file:

```
    "protoc": {
        "options": [
            "--proto_path=/home/joe/development/protobuf/.",
        ],
    },
```

## Important limitations when adding new .proto files

Protobuf allows naming a package differently than the file it is placed in. This, however, is not
allowed here. The `package` directive within the proto files **must** have the same name as the file
itself and can only contain alphanumeric characters and underscores (especially no `.`).

The reasons are:
* The Rust protobuf compiler uses the package name for naming the output file, which then could
  potentially be named different than the input file. Which we do not want to avoid confusion.
* Rust module names cannot contain `.` and protobuf packages are translated into Rust modules.

## Prerequisites

* Necessary development tools and / or compiler of the supported languages.
    * For Rust: https://rustup.rs/
    * For Go: https://go.dev/doc/install
* Install the Protocol buffer compiler `protoc` version 3.
  * Generally it is recommended to install the [pre-compiled
    binaries](https://grpc.io/docs/protoc-installation/#install-pre-compiled-binaries-any-os)
    instead of installing the version from your package manager as it is likely
    out of date.
* Install any language specific plugins, including anything required to work
  with gRPC.
  * For example [for Go](https://grpc.io/docs/languages/go/quickstart/) you need
    `protoc-gen-go` and `protoc-gen-go-grpc`.
* Install the custom Rust protocol buffer compiler from https://github.com/thinkparq/protoc-rs.
  This can be done using `cargo`:
  ```
  cargo install --git "https://github.com/thinkparq/protoc-rs"

  ```
* Ensure all the necessary binaries are added to your `$PATH` variable using
  `.bashrc` or similar. For `cargo`, the install directory is usually `~/.cargo/bin`.

## Generating Code for a New Language

The project Makefile is setup with targets to compile all protobuf files under the `./proto`
directory into a multiple target languages when `make` or `make protos` is
executed. Each languages output is put into a separate output directory (e.g. `./go`, `./rust`,
`./cpp`).

To add a new language amend the Makefile as follows:

1. Define the output directory of the new language by adding a new <LANGUAGE>_OUT_DIR variable
2. Add any additional options to the `.proto` files required by the language.
   1. For example for Go something like `option go_package =
      "github.com/thinkparq/protobuf/go/beewatch";` is required so cross
      package imports work correctly.
3. Add a new variable `<LANGUAGE>_GENERATED_FILES` that transforms the `.proto`
   file names returned by `PROTO_FILES` into corresponding files to generate for
   the target language within the correct output directory. 
4. Modify the `protos` target to add the generated list of files as a
   dependency.
5. Create a new pattern rule for the target language to generate the appropriate
   source files from the `.proto` files with the correct `protoc` (or other) command for
   the new language in the correct output directory. Ensure to any `_out` flags or similar are set to the
   correct language specific output directory (can be extracted from the target file using `$(dir $@)`).
6. Add the new output directory to the find command in the `test` target.

In general, you can likely just copy the code for one of the existing languages and modify it to your needs.

## Generating Code for Golang

If you plan to make changes to the protocol buffers (.proto files) or simply
want to manually generate the code you will need to
[install](https://grpc.io/docs/languages/go/quickstart/) the Protocol Buffer
compiler v3 (protoc) and Go plugins for the protocol compiler. Whenever changes
are made to the `*.protos` files ensure to run `make protos` and commit the
changes to the repository. 

## Generating Code for Rust

Protobuf and gRPC do not provide official support for Rust. Therefore, 3rd-party tools have to be
used to generate the code. We use a custom made tool [protoc-rs](https://github.com/thinkparq/protoc-rs)
based on [Tonic](https://docs.rs/tonic/latest/tonic/), which replaces `protoc`. It does not only
compile the protobuf files but also generate the necessary Rust crate structure to be able to import
and use the code as a dependency in Rust projects.

# References

* [Style Guide](https://protobuf.dev/programming-guides/style/)
* [Go Generated Code Guide](https://protobuf.dev/reference/go/go-generated/)
