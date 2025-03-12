build-all:
    just build-linux
    just build-windows
    just build-web

build-linux: generate-license
    cross build --release --bin curveball --target x86_64-unknown-linux-gnu
    zip -j target/x86_64-unknown-linux-gnu/release/curveball-x86_64-unknown-linux-gnu target/x86_64-unknown-linux-gnu/release/curveball doc/legal/licenses.html

build-windows: generate-license
    cross build --release --bin curveball --target x86_64-pc-windows-gnu
    zip -j target/x86_64-pc-windows-gnu/release/curveball-x86_64-pc-windows-gnu target/x86_64-pc-windows-gnu/release/curveball.exe doc/legal/licenses.html

build-web:
    (cd curveball && trunk build --release)

generate-license:
    cargo-about generate --workspace about.hbs > doc/legal/licenses.html
