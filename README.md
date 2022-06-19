# transient

[![Crates.io](https://img.shields.io/crates/v/transient.svg)](https://crates.io/crates/transient)
[![Docs.rs](https://docs.rs/transient/badge.svg)](https://docs.rs/transient)
[![CI](https://github.com/dark-fusion/transient/workflows/CI/badge.svg)](https://github.com/dark-fusion/transient/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/transient/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/transient?branch=main)

## Project Description

Proof-of-concept lexer and parser. The actual implementation will change over time as the
end goal is to create a domain-specific programming language.

A more detailed description will be added once the project is fleshed out a bit more.

### Resources

Some of the resources utilized throughout initial development include:

- #lang-dev channel from Rust's Discord server
- "Parsing basics" [blog post][ref-parsing-basics] by [Dominic Quirl][https://github.com/domenicquirl]
- [chumsky][ref-chumsky-repo]: Rust lexer/parser crate
- [nom][ref-nom-repo]: Rust zero-copy parser combinator crate

## Installation

### Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install transient`

## License

Licensed under the [MIT License](LICENSE):

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `elucidate` by you, shall be licensed under the MIT License, without any additional
terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

<!-- External links -->
[ref-parsing-basics]: https://domenicquirl.github.io/blog/parsing-basics/
[ref-chumsky-repo]: https://github.com/zesterer/chumsky
[ref-nom-repo]: https://github.com/Geal/nom
