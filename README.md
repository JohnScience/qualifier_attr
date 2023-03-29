# qualifier_attr

[![Latest Version](https://img.shields.io/crates/v/qualifier_attr.svg)][`qualifier_attr`]
[![Downloads](https://img.shields.io/crates/d/qualifier_attr.svg)][`qualifier_attr`]
[![Documentation](https://docs.rs/qualifier_attr/badge.svg)][`qualifier_attr`/docs]
[![License](https://img.shields.io/crates/l/qualifier_attr.svg)][`qualifier_attr`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/qualifier_attr/status.svg)][`qualifier_attr`/dep_status]

> Procedural macro attributes for adding "qualifiers" to various items.

At the moment, the crate supports only functions and the following "qualifiers":

* `pub`, `pub(crate)`, ... - visibility qualifiers
* `async` - async qualifier
* `unsafe` - unsafe qualifier
* `const` - const qualifier
* `extern "ABI"` - ABI qualifier

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`qualifier_attr`]: https://crates.io/crates/qualifier_attr
[`qualifier_attr`/docs]: https://docs.rs/qualifier_attr
[`qualifier_attr`/license]: https://github.com/JohnScience/qualifier_attr#license
[`qualifier_attr`/dep_status]: https://deps.rs/repo/github/JohnScience/qualifier_attr
