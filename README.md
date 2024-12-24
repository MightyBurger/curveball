<div align="center">

# Curveball

Curve generator for [Neverball]

</div>

Curveball is a set of tools for generating curved geometry. It was written to support the development of a new [Neverball] level set, as the existing [curve.c] was not able to meet this set's needs. 


## Usage

Install [Rust](https://www.rust-lang.org/).

In your command line, run `cargo install curveball`.

## Features

Curveball currently supports generating these curves:

- Catenary
- Cones (generates with half the number of lumps compared to [curve.c])
- "Rayto" curves

## Structure

If you are unfamiliar with Rust, projects are often organized in Cargo workspaces. This repository is a workspace, and it contains the following crates:

- `curveball` - Binary crate; compiles to the command-line tool `curveball`. Calls functions defined in `curveball-lib`.
- `curveball-lib` - Library crate containing functions to generate various curves.


## Project status

Curveball is in early development and is not currently useable.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

[Neverball]: https://neverball.org/
[curve.c]: https://github.com/Neverball/neverball/blob/master/contrib/curve.c