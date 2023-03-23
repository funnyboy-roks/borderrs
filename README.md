# borderrs

<!-- To generate the readme, use `cargo-readme`: `cargo readme > README.md` -->
[![Crates.io](https://img.shields.io/crates/v/borderrs.svg)](https://crates.io/crates/borderrs)
[![Documentation](https://docs.rs/borderrs/badge.svg)](https://docs.rs/borderrs/)
[![dependency status](https://deps.rs/repo/github/funnyboy-roks/borderrs/status.svg)](https://deps.rs/repo/github/funnyboy-roks/borderrs)

This crate allows the user to format many data structures in ways that look nicer to the
end-user.

The Wikipedia page on [Box-Drawing Characters](https://en.wikipedia.org/wiki/Box-drawing_character#Box_Drawing) has been quite helpful

Currently, we support:
- [`slice`]s with [`BorderFormatter::format_slice`]
- [`Iterator`]s with [`BorderFormatter::format_iter`]
- [`HashMap`]s with [`BorderFormatter::format_hash_map`]
- impl [`Display`] with [`BorderFormatter::format_display`]
- impl [`Debug`] with [`BorderFormatter::format_debug`]


## Usage Example

```rust
use borderrs::{styles::THIN, BorderFormatter};

let slice = [0, 1, 2, 3, 4];
println!("{}", THIN.format_slice(&slice));

let mut map = HashMap::default();
map.insert("Jon", 38);
map.insert("Jake", 25);
map.insert("Josh", 17);
println!("{}", THIN.format_hash_map(&map));

println!("{}", THIN.format_display("hello"));
println!("{}", THIN.format_debug("hello"));
```
