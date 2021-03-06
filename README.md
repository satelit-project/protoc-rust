# protoc-rust

`protoc` plugin for Rust code generation based on top of
[`prost`](https://github.com/danburkert/prost) and
[`tonic`](https://github.com/hyperium/tonic).

## Dependencies

- Rust _1.39_
- Protobuf _3.7.1_

## Instalation

Plugin can be installed via `cargo install`: 

```
cargo install protoc-rust \
  --git "https://github.com/satelit-project/protoc-rust" \
  --tag 0.2.0-beta.1 \
  --force
```

You can also specify custom binary location via `--root` argument.

## Usage

Invoke `protoc` directly:

```
protoc \
  --plugin=protoc-gen-rust="$(which protoc-gen-rust)" \
  --proto_path=<protos include path> \
  --rust_out=<output directory> \
  <proto file>...
```

Or use custom tools like [`protogen`](https://github.com/satelit-project/protogen) like in [`satelit-proto`](https://github.com/satelit-project/satelit-proto) project.

### Customization

You can customize plugin behaviour by passing flags directly to the
plugin via `--rust_opt` `protoc`'s argument. For example: `protoc
--rust_opt=grpc,no-flat-modules`.

Available flags are:

* `grpc` – generate code for gRPC services.
* `grpc-client` – generate client side code for gRPC services.
* `grpc-server` – generate server side code for gRPC services.
* `no-flat-modules` – map protobuf packages to Rust modules (See
  `Config` documentation for pitfalls).
* `extern-path` – [map Protobuf types to Rust types](https://docs.rs/prost-build/0.5.0/prost_build/struct.Config.html#method.extern_path)
