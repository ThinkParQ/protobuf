# BeeGFS Protocol Buffers <!-- omit in toc -->

# Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Quick Start](#quick-start)
  - [Go](#go)
- [Advanced](#advanced)
  - [Generating Code: Go](#generating-code-go)
- [References](#references)

# Overview 

This repository contains the [Protocol Buffer](https://protobuf.dev/overview/) and gRPC service definitions (as `.proto` files) for a number of projects in the BeeGFS ecosystem. These `.proto` files are used to generate code that allows the data structures and gRPC clients and servers to be easily used from a number of languages. Generated code for select languages is also provided through this repository, or users can generate code themselves using the `protoc` tool and any language specific plugins.

# Quick Start

Most users will not need to worry about generating code themselves, and can simply import this repository into their project using whatever method is appropriate for their language. 

## Go

As long as this repository is private, some additional configuration is required to allow this repository to be imported as a private module into your Go project. These steps presume you are using SSH and the SSH key for the machine you're on is already added to your GitHub account and has access to this repository. 

(1) In your .bashrc/similar set: 

```shell
export GOPRIVATE=github.com/thinkparq/bee-protos
```
(2) Add the following to your global `.gitconfig` file to ensure `go get` will use SSH instead of HTTP:

```shell
[url "ssh://git@github.com/"]
	insteadOf = https://github.com/
```




# Advanced

When working with protobuf files and vscode it is recommended to install [this extension](https://marketplace.visualstudio.com/items?itemName=zxh404.vscode-proto3).

## Generating Code: Go

If you plan to make changes to the protocol buffers (.proto files) or simply want to manually generate the code you will need to [install](https://grpc.io/docs/languages/go/quickstart/) the Protocol Buffer compiler v3 (protoc) and Go plugins for the protocol compiler. Whenever changes are made to the `*.protos` files ensure to run `make protos` and commit the changes to the repository. 

# References

* [Style Guide](https://protobuf.dev/programming-guides/style/)
* [Go Generated Code Guide](https://protobuf.dev/reference/go/go-generated/)