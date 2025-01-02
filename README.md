<div align="center">

# Curveball

Curve generator for [Neverball]

</div>

Curveball is a collection of curve generating tools for [Neverball] level developers. If [curve.c] does not meet your needs, Curveball might.

There are two Curveball programs:
- **curveball** - a command-line curve generating tool
- **curvebird** - a graphical curve generating tool

The output of these programs is a `.map` file you can load in a program like [Trenchbroom].

## Features

Curveball currently supports generating these curves:

- Catenary
- Bank
- Rayto
- Serpentine

## Installation

Both **curveball** and **curvebird** are hosted on [crates.io], so you can easily compile the latest release from source to install the software.

First, install [Rust](https://www.rust-lang.org/).

To install **curveball**, run `cargo install curveball`.

To install **curvebird**, run `cargo install curvebird`.

There are other options; see [here](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

## Is it any good?

Yes.

## Project Structure

Curveball is written in Rust.

This repository is a Cargo workspace, and it contains the following crates:

- `curveball` - Binary crate; compiles to the CLI tool.
- `curvebird` - Binary crate; compiles to the GUI tool.
- `curveball-lib` - Library crate containing functions to generate various curves.

## License

Curveball is licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

[crates.io]: https://crates.io/
[curve.c]: https://github.com/Neverball/neverball/blob/master/contrib/curve.c
[Neverball]: https://neverball.org/
[Trenchbroom]: https://trenchbroom.github.io/

### Notice

Future versions may be released under a different license. [Neverball] is licensed under the terms of GPLv2, so if Curveball ever makes use of [Neverball] assets or code, it will need to be released under a GPL license, too.

A license change will result in a major version bump.