// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use lib_curveball::curve::Curve;
use lib_curveball::curve::curve_classic::CurveClassic;
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
    let curve_classic = CurveClassic {
        n: 12,
        ri0: 32.0,
        ro0: 64.0,
        ri1: 32.0,
        ro1: 64.0,
        theta0: 0.0,
        theta1: 90.0,
        t: 8.0,
    };

    let brushes = curve_classic.bake().unwrap();
    let string = brushes_to_string(brushes);

    println!("{}", string);
}
