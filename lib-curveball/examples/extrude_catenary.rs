// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use lib_curveball::curve::CurveResult;
use lib_curveball::curve::extrude::path::Catenary;
use lib_curveball::curve::extrude::profile::{Anchor9Point, Rectangle};
use lib_curveball::curve::extrude::{ProfileOrientation, ProfilePlane, extrude};
use lib_curveball::map::{
    entity::SimpleWorldspawn,
    geometry::Brush,
    qmap::{QEntity, QMap},
};

fn brushes_to_string(brushes: Vec<Brush>) -> CurveResult<String> {
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    let map = QMap::new(vec![entity]).with_tb_neverball_metadata();
    Ok(String::from(format!("{map}")))
}

fn main() {
    // Create the rectangle profile
    let width = 32.0;
    let height = 8.0;
    let anchor = Anchor9Point::Center;
    let rectangle_profile = Rectangle::new(width, height, anchor).unwrap();

    // Create the catenary path
    let span = 128.0;
    let height = 0.0;
    let s = 132.0;
    let catenary_path = Catenary::new(span, height, s).unwrap();

    // Extrude to create the path
    let num_segments = 12;
    let profile_orientation = ProfileOrientation::Constant(ProfilePlane::YZ);
    let brushes = extrude(
        num_segments,
        &rectangle_profile,
        &catenary_path,
        profile_orientation,
    )
    .unwrap();

    // Print the map out
    let string = brushes_to_string(brushes).unwrap();
    println!("{}", string);
}
