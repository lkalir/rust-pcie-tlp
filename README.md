# Rust-PCIe-TLP

Rust library for parsing and generating PCIe Transport Layer Packets (TLPs).
Supports `no_std` environments. Inspired by [go-pcie-tlp](https://github.com/google/go-pcie-tlp).

## Status

- [x] Device IDs
- [x] TLP headers
- [ ] Completion headers
- [ ] Configuration read/write headers
- [ ] Memory read/write headers
- [ ] IO read/write headers
- [ ] Thorough documentation
- [x] Blazingly fast 🔥
- [x] Fearless concurrency 🦀

## Usage

Add `rust-pcie-tlp` to your `Cargo.toml` file:

```toml
[dependencies]
rust-pcie-tlp = "0.1.0"
```

See the examples in `examples/` or read the documentation for more details.

## Documentation

Run `cargo doc --open` to build the documentation and open it in your browser.

## Contributing

Pull requests and suggestions are welcome! See [contributing.md](contributing.md).
