# protoc-rust
`protoc` plugin for Rust sources generation

## Dependencies

- Rust (latest _stable_)
- Protobuf (_v.3.7.1_)

## Instalation

Plugin can be installed via `cargo install`: 

```
cargo install \
  --git "https://github.com/satelit-project/protoc-rust" \
  --force
```

**TODO**: will it install latest version?

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

Or use custom tools like [`prototool`](https://github.com/uber/prototool) like in [`satelit-proto`](https://github.com/satelit-project/satelit-proto) project.
