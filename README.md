<div align="center">
  <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  <h1>netutil</h1>
</div>

> This crate relies on Netinet libraries available in most Linux distributions.

## Requirements

- Make sure Netinet libraries are available in your system
- Clang as `bindgen` leverages `libclang` to preprocess, parse, and type check C and C++ header files. [Rust Bindgen Requirements](https://rust-lang.github.io/rust-bindgen/requirements.html#clang)

## Development

You must build the binary first and the execute it with `sudo` in order
to allow the socket connection implementation

```bash
cargo build && sudo ./target/debug/netutil
```

## References

- [The Single UNIX ® Specification, Version 2](https://pubs.opengroup.org/onlinepubs/7908799/xns/netinetin.h.html)