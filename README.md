# BeeGFS Protocol Buffers <!-- omit in toc -->

# Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Quick Start](#quick-start)
  - [Versioning](#versioning)
- [Advanced: Generate / Compile `.proto` Files](#advanced-generate--compile-proto-files)
  - [Important limitations when adding new .proto files](#important-limitations-when-adding-new-proto-files)
  - [Prerequisites](#prerequisites)
  - [Generating Code for a New Language](#generating-code-for-a-new-language)
  - [Generating Code for Golang](#generating-code-for-golang)
  - [Generating Code for Rust](#generating-code-for-rust)
- [Updating Existing Protocol Buffers](#updating-existing-protocol-buffers)
  - [Coding Standards](#coding-standards)
  - [Gotchas](#gotchas)
  - [Best Practices](#best-practices)
    - [Field Presence](#field-presence)

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

Most users will not need to worry about generating code themselves, and can simply import the
libraries found in this repository into their project using whatever method is appropriate for the
language. Refer to language specific dependency management documentation or the ThinkParQ wiki for
languages commonly used with these protocol buffers:

* [Go](https://github.com/ThinkParQ/beegfs-go/wiki/Getting-Started-with-Go#dependency-management)
* [Rust](https://github.com/ThinkParQ/beegfs-rs/wiki/Getting-Started-with-Rust#dependency-management)

## Versioning

The major version of the protobuf library will remain at v0, indicating these APIs are not yet
guaranteed to be stable. For each x.y BeeGFS major/minor release, the protobuf API will be versioned
as v0.x.y. An official tag will not be added for patch versions as in general new functionality
should only be made available in a major.minor versions. If functionality/fixes are needed elsewhere
between official releases, those changes should be imported using a pseudo-version (see below).

If you make changes to protobuf that need to be immediately imported elsewhere before a new
"official" version is tagged, you should import your changes by their commit hash. Depending on the
language this may either be referenced as a pseudo-version (i.e., for Go using `go get`) or by
referencing the commit hash directly (i.e., for Rust using cargo). In general it is preferable to
import the library using a tagged version if possible.

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

* Setup the development tools and / or compiler of the supported languages:
    * For Rust: https://rustup.rs/
    * For Go: https://go.dev/doc/install
* Install all required tooling:
  * The simplest way to install all the tooling is to run `make install-tools` (requires `curl`).
    This will install all the necessary tools using the correct versions to your `$HOME/.local/bin`
    (the default folder for user programs). 
    * Note that if you already have them installed in different locations (e.g. the default
      `$GOBIN`), they might have higher priority depending on your `$PATH` and still be run instead.
      Make sure to you remove those manually.
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
    * For Rust's `cargo install`, the default install directory is usually `~/.cargo/bin`.
    * For Go's `go install` it is usually put to `$GOBIN` or `$GOPATH/bin`. 

## Generating Code for a New Language

The project Makefile is setup with targets to compile all protobuf files under the `./proto`
directory into a multiple target languages when `make` or `make protos` is executed. Each languages
output is put into a separate output directory (e.g. `./go`, `./rust`, `./cpp`).

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

## Coding Standards

This project largely adheres to the existing standards for writing and maintaining protocol buffers:

* [Best Practices](https://protobuf.dev/best-practices/)
* [Style Guide](https://protobuf.dev/programming-guides/style/)

Rather than recreate that existing documentation, this section focuses on anything specific to this
project's use of protocol buffers along with any "gotchas" we have encountered and "best practices"
we have defined or are important to contextualize for this project.

## Gotchas

* Generally [field numbers](https://protobuf.dev/programming-guides/proto3/#assigning) should never
  be changed after they are merged to main. If these are changed clients and servers with different
  field numbers will no longer be compatible. This will also cause issues unmarshalling protobuf
  messages stored on-disk that are not immediately obvious because the message will likely
  unmarshall, but the fields may be populated partially or incorrectly depending on the changes.
* [Extensively document fields](https://protobuf.dev/best-practices/api/#precisely-concisely) - what
  they do, how to use them under which circumstances and their limitations. Are they required or
  optional, does this depend on other fields? What is their purpose, what application data do they
  contain? Document potential values with special meanings (e.g. "-1 means unlimited") and the valid
  range of values (unless a field can be considered opaque from the message consumers view, e.g.
  pagination handles). Documenting messages like this defines a contract that API users have to
  follow when using the message. This ensures that all implementations use the message the right way
  and are compatible to each other. For example, if a field is meant to be "required", it means that
  1. the provider of that message must provide this field and
  2. the the consumer can rely on this
  field being present, no matter what.

## Best Practices

### [Field Presence](https://protobuf.dev/programming-guides/field_presence/)

TL;DR - Generally expect fields to default to the language specific default values for each type for
any fields that were not set by the server unless the recommended `optional` keyword is used.

Unless there is a good reason not to, generally use the `optional` keyword to force fields to have
explicit presence. On a case-by-case basis developers can choose to omit the `optional` keyword if
one of the following is true:

1. The default value for a given type matches the default behavior they want when not explicitly
configured, i.e., `var execute bool` to trigger a dry run that does not make changes by default.
2. If the default value is the "invalid" variant, i.e., `var name string` could simply check name is
not an empty string. 

Generally this should be done sparingly as it is always safer to use explicit presence and is now
[officially recommended](https://protobuf.dev/programming-guides/field_presence/) as it provides a
smoother path to transition to protobuf editions.

Exceptions are if you really need the slight performance improvement of not serializing extra bits
that mark field presence and for languages like Go where you wish to avoid extra conditionals to
check for field presence (because Go uses pointers for fields with explicit presence).

Because of how Go handles field presence you may notices messages primarily consumed by Go code tend
to lean away from using `optional`. While this is fine if you are confident one or both of the above
rules hold true, for messages that may be used by multiple languages using explicit field presence
can help ensure consistency. Take special care to document any special behaviors or assumptions made
in non-optional fields, for example if "0" has a specific meaning (such as invalid) to the receiver.
