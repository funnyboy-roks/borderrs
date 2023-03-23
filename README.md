# borders

[![Crates.io](https://img.shields.io/crates/v/borders.svg)](https://crates.io/crates/borders)
[![Documentation](https://docs.rs/borders/badge.svg)](https://docs.rs/borders/)
[![dependency status](https://deps.rs/repo/github/funnyboy-roks/borders/status.svg)](https://deps.rs/repo/github/funnyboy-roks/borders)

This crate allows the user to format many data structures in ways that look nicer to the
end-user.

Currently, we support:
- [`slice`]s with [`BorderFormatter::format_slice`]
- [`Iterator`]s with [`BorderFormatter::format_iter`]
- [`HashMap`]s with [`BorderFormatter::format_hash_map`]
- impl [`Display`] with [`BorderFormatter::format_display`]
- impl [`Debug`] with [`BorderFormatter::format_debug`]


## Usage Example

```rust
use borders::{styles::THIN, BorderFormatter};

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
