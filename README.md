<div align="center">

# Curveball

Curve generator for [Neverball] levels

![cuveball logo](resources/curveball.png)

👉 [Click to run the web tool][Curveball on the Web] 👈

</div>

Curveball is a curve generator tool for [Neverball] level developers.

This repository contains the Curveball curve generator engine **lib-curveball** and two programs to access it:

- **curveball** - a graphical tool
- **curveball-cli** - a command-line tool

Curveball produces Quake3 map data you can copy and paste into a program like [Trenchbroom].

## Curves

Curveball generates a wider variety of curves than what is possible with [curve.c].

Curveball supports generating these curves:

- Curve Classic
- Curve Slope
- Rayto
- Extrusion

### Curve Classic

Curve Classic contains the same functionality as `curve.c` when `slope` is disabled. It produces circular arcs.

### Curve Slope

Curve Classic contains the same functionality as `curve.c` when `slope` is enabled, with a little more flexibility.

### Rayto

An oddity in the set of curves Curveball produces, **Rayto** fills in the "negative space" left by a circular arc. The resulting shape is useful for avoiding [T-intersections](https://icculus.org/neverball/mapping/) when constructing maps.

### Extrusion

The **Extrusion** tool generates curves in a manner inspired by mechanical CAD sofware. The tool allows you to select one of the following 2D profiles:

- Circle
- Circle sector
- Rectangle
- Parallelogram
- Annulus
- Any arbitrary set of convex polygons

The tool will extrude these 2D profiles along one of the following paths in 3D space:

- Line
- Revolve
- Sinusoid
- Bezier
- Catenary
- Serpentine

## Installation

First, see if [Curveball on the Web] meets your needs.

For a local installation, consider downloading one of the releases in Github. Alternatively, you may
compile from source.

## Compiling

First, install [Rust](https://www.rust-lang.org/).

Then, use [Cargo](https://doc.rust-lang.org/cargo/) to build the software.

For example, `cargo run --bin curveball --release` will compile and run **curveball** in release mode.

You may also use `cargo install` to install the software. See [here](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information.

## Compiling for distribution

Building for distribution requires you to be set up for cross-compilation and compiling for the web. Dependencies include:

- [just](https://github.com/casey/just) - a more convenient Makefile
- [Trunk](https://trunkrs.dev/) - to create a website with WASM
- [cross](https://github.com/cross-rs/cross) - for simple cross-compilation; requires Docker or Podman

You will also need to add targets for `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-gnu` with `rustup target add <TARGET>`.

Distribution simply requires running the `justfile`:

```
just build-linux
just build-windows
just build-web
```

## Project Structure

Curveball is written in Rust.

This repository is a Cargo workspace with the following crates:

- `curveball` - Binary crate; compiles to the GUI tool.
- `curveball-cli` - Binary crate; compiles to the CLI tool.
- `curveball-lib` - Library crate containing functions to generate the various curves.

This organization makes `curveball-cli` faster to compile since Cargo does not include all the dependencies of the GUI application.

## Is it any good?

Yes.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

### Notice

Future versions may be released under a different license. [Neverball] is licensed under the terms of GPLv2, so if Curveball ever makes use of [Neverball] assets or code, it will need to be released under a GPL license, too.

A license change will result in a major version bump.

[crates.io]: https://crates.io/
[curve.c]: https://github.com/Neverball/neverball/blob/master/contrib/curve.c
[Curveball on the Web]: https://mightyburger.github.io/curveball-web/
[Neverball]: https://neverball.org/
[Trenchbroom]: https://trenchbroom.github.io/
