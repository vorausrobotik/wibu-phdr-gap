# wibu-phdr-gap

`wibu-phdr-gap` is a Rust library that ensures the presence of a designated placeholder within compiled artifacts. This placeholder allows WIBU Systems' AxProtector to process the file during post-compilation.

## Features

- Provides the `ensure_header_space` function, which must be invoked from user code.
- Inserts a placeholder object into the compiled artifact.
- Fully tested and available on [crates.io](https://crates.io/crates/wibu-phdr-gap).

## Installation

Add `wibu-phdr-gap` to your `Cargo.toml`:

```toml
[dependencies]
wibu-phdr-gap = "*"
```

## Usage

Ensure that `ensure_header_space` is called within your code:

```rust
use wibu_phdr_gap::ensure_header_space;

fn main() {
    ensure_header_space();
    // Your application logic here
}
```

## Example

There's an example available which can be inspected for reference and is automatically built and verified using integration tests.

## Acknowledgments

Thanks to WIBU Systems AG for kindly providing the magic string under the MIT license and their fast response times.
This project was developed at voraus robotik GmbH. Special thanks to Lukas Beckmann for his contributions.

## Dependencies

This crate aims for the least possible amount of dependencies and currently uses `elf` as development dependency only.
It is used to verify the target artifact does indeed have the designed program header.

## License

This project is licensed under [MIT](LICENSE), but it does not grant any rights to WIBU Systems' trademarks or intellectual property.


