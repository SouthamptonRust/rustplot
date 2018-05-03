# Rustplot
A simple GUI based plotting library for Rust using GTK and Cairo.

## Documentation

Documentation of the API can be found on lib.rs.

## Usage

Requires installation of GTK+, GLib and Cairo for use of the gtk-rs crates.
Installation instructions can be found on the gtk-rs [requirements page](http://gtk-rs.org/docs/requirements.html).

Add this to `Cargo.toml`:
```toml
[dependencies]
rustplot = "0.1.0"
```

and this to crate root:
```rust
extern crate rustplot;

// For all functionality
use rustplot::*;
```

## Testing

To run unit tests:
cargo test --lib

To rust integration tests:
cargo test --test \<test module name\>

To run rustdoc tests:
cargo test --doc

## License

Rustplot is distributed under the terms of the MIT license.

See  [LICENSE](LICENSE) for details.
