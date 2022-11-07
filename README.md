**This project has moved to [jpittis/rust-control-plane](https://github.com/jpittis/rust-control-plane), which as been published to crates.io under [data-plane-api](https://crates.io/crates/data-plane-api).**



This is still a toy project, so YMMV. Bobby's
https://github.com/bpowers/rust-envoy-control-plane may be more legit. The goal is to
implement https://github.com/jpittis/rust-control-plane on top of this, as a Rust
equivalent to https://github.com/envoyproxy/go-control-plane.

## How to Build

For now, you can depend on this crate with:

```
rust-xds-grpc = { git = "https://github.com/jpittis/rust-xds-grpc" }
```

Cargo will automatically clone all the submodules, and build the protobufs.

The build depends on protoc.
