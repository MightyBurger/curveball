// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use glam::DVec3;
use lib_curveball::map::{
    entity::SimpleWorldspawn,
    geometry::Brush,
    qmap::{QEntity, QMap},
};

fn brushes_to_string(brushes: Vec<Brush>) -> String {
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    let map = QMap::new(vec![entity]).with_tb_neverball_metadata();
    String::from(format!("{map}"))
}

fn main() {
    // Let's create two brushes (cube and triangular prism) and
    // form it into a map.

    let cube_vertices = [
        DVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        DVec3 {
            x: 0.0,
            y: 0.0,
            z: 128.0,
        },
        DVec3 {
            x: 0.0,
            y: 128.0,
            z: 0.0,
        },
        DVec3 {
            x: 0.0,
            y: 128.0,
            z: 128.0,
        },
        DVec3 {
            x: 128.0,
            y: 0.0,
            z: 0.0,
        },
        DVec3 {
            x: 128.0,
            y: 0.0,
            z: 128.0,
        },
        DVec3 {
            x: 128.0,
            y: 128.0,
            z: 0.0,
        },
        DVec3 {
            x: 128.0,
            y: 128.0,
            z: 128.0,
        },
    ];
    let cube = Brush::try_from_vertices(&cube_vertices, None).unwrap();

    let pyramid_vertices = [
        DVec3 {
            x: 256.0,
            y: 256.0,
            z: 0.0,
        },
        DVec3 {
            x: 256.0,
            y: 512.0,
            z: 0.0,
        },
        DVec3 {
            x: 512.0,
            y: 256.0,
            z: 0.0,
        },
        DVec3 {
            x: 256.0,
            y: 256.0,
            z: 256.0,
        },
    ];
    let pyramid = Brush::try_from_vertices(&pyramid_vertices, None).unwrap();

    let brushes = vec![cube, pyramid];
    let string = brushes_to_string(brushes);

    println!("{}", string);
}
