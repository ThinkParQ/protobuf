# BeeGFS Protocol Buffers <!-- omit in toc -->

# Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Quick Start](#quick-start)
  - [Versioning](#versioning)
  - [Go](#go)
- [Advanced: Generate / Compile `.proto` Files](#advanced-generate--compile-proto-files)
  - [Important limitations when adding new .proto files](#important-limitations-when-adding-new-proto-files)
  - [Prerequisites](#prerequisites)
  - [Generating Code for a New Language](#generating-code-for-a-new-language)
  - [Generating Code for Golang](#generating-code-for-golang)
  - [Generating Code for Rust](#generating-code-for-rust)
- [Updating Existing Protocol Buffers](#updating-existing-protocol-buffers)
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
| C++      | Yes          | A target has been added to the Makefile, but the resulting files have not been tested yet. |
| Go       | Yes          | N/A                                                                                        |
| Rust     | Yes          | N/A                                                                                        |

# Quick Start

Most users will not need to worry about generating code themselves, and can
simply import this repository into their project using whatever method is
appropriate for their language.

## Versioning

The major version of the protobuf library will remain at v0, indicating these APIs are not yet
guaranteed to be stable. For each v8.x.y BeeGFS release, the protobuf API will be versioned as
v0.x.y and it is preferable to import the library using a tagged version when possible.

If you make changes to protobuf that need to be immediately imported elsewhere before a new
"official" version is tagged, you should import your changes by their commit hash. Depending on the
language this may either be referenced as a pseudo-version (i.e., for Go using `go get`) or by
referencing the commit hash directly (i.e., for Rust using cargo). 

See the dependency management documentation for
[Go](https://github.com/ThinkParQ/developer-handbook/tree/main/getting_started/go#how-to-coordinate-changes-requiring-updates-in-multiple-repositories)
and
[Rust](https://github.com/ThinkParQ/developer-handbook/blob/main/getting_started/rust/README.md#working-with-internal-git-dependencies)
for more details on how to manage internal dependencies. Note this versioning strategy is also used
for other projects such as
[beegfs-go](https://github.com/ThinkParQ/beegfs-go?tab=readme-ov-file#versioning). 

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

* Before you can start, you need to setup the development tools and / or compiler of the supported
  languages:
    * For Rust: https://rustup.rs/
    * For Go: https://go.dev/doc/install
* You need read access to the private [protoc-rs](https://github.com/thinkparq/protoc-rs) repository.
* The simplest way to install all the tooling is to run `make install-tools` (requires `curl`).
  This will install all the necessary tools using the correct versions to your `$HOME/.local/bin`
  (the default folder for user programs). Note that if you already have them installed in different
  locations (e.g. the default `$GOBIN`), they might have higher priority depending on your `$PATH`
  and still be run instead. Make sure to you remove those manually.
* If you want to install the tools manually, you can do that as well.
  * The Makefile defines which tools on which versions you need - make sure you use exactly the
    same. Do not use your package manager as it will most likely provide a different version.
  * Install the Protocol buffer compiler `protoc`.
    * Generally it is recommended to install the [pre-compiled
      binaries](https://grpc.io/docs/protoc-installation/#install-pre-compiled-binaries-any-os).
  * Install any language specific plugins, including anything required to work
    with gRPC.
    * For example [for Go](https://grpc.io/docs/languages/go/quickstart/) you need
      `protoc-gen-go` and `protoc-gen-go-grpc`.
  * Install the custom Rust protocol buffer compiler from https://github.com/thinkparq/protoc-rs.
    * This can be done using `cargo`:
      ```
      cargo install --git "https://github.com/thinkparq/protoc-rs --tag "$VERSION" --locked"

      ```
* Ensure all the necessary paths are added to your `$PATH` variable using `.bashrc` or similar.
  * If you use `make install-tools`, you only need to add `$HOME/.local/bin` if it is not already
    part of your `$PATH`.
  * If you installed manually, it depends on where you installed it.
    * For Rusts `cargo install`, the default install directory is usually `~/.cargo/bin`.
    * For Gos `go install` it is usually put to `$GOBIN` or `$GOPATH/bin`. 

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
3. Modify the `protos` target to do whatever is needed to build the new language. Most likely
   this will be adding some language specific output parameters to the existing `protoc` call.
4. Add the new output directory to the find command in the `test` target.

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

# Updating Existing Protocol Buffers

* Generally [field numbers](https://protobuf.dev/programming-guides/proto3/#assigning) should never
  be changed after they are merged to main. If these are changed clients and servers with different
  field numbers will no longer be compatible. This will also cause issues unmarshalling protobuf
  messages stored on-disk that are not immediately obvious because the message will likely
  unmarshall, but the fields may be populated partially or incorrectly depending on the changes.

# References

* [Style Guide](https://protobuf.dev/programming-guides/style/)
* [Field Presence](https://protobuf.dev/programming-guides/field_presence/)
  * TL;DR - Generally expect fields to default to the language specific default values for each type for any fields that were not set by the server.
* [Go Generated Code Guide](https://protobuf.dev/reference/go/go-generated/)
