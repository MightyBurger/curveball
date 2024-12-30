<div align="center">

# Curveball

Curve generator for [Neverball]

</div>

Curveball is a set of tools for generating curved geometry. It was written to support the development of a new [Neverball] level set by producing curves [curve.c] cannot generate. 


## Usage

Install [Rust](https://www.rust-lang.org/).

In your command line, run `cargo install curveball`.

Run `curveball --help`.

## Features

Curveball currently supports generating these curves:

- Catenary
- Bank (generates with half the number of lumps compared to [curve.c])
- "Rayto" - triangular prisms from a circular arc to a point
- Serpentine

## Structure

If you are unfamiliar with Rust, projects are often organized in Cargo workspaces. This repository is a workspace, and it contains the following crates:

- `curveball` - Binary crate; compiles to the CLI tool.
- `curvebird` - Binary crate; compiles to the GUI tool.
- `curveball-lib` - Library crate containing functions to generate various curves.

## Project status

The CLI interface, `curveball`, is useable.

The GUI interface, `curvebird`, is a WIP.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

[Neverball]: https://neverball.org/
[curve.c]: https://github.com/Neverball/neverball/blob/master/contrib/curve.c
