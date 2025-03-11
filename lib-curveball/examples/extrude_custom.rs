// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::{DVec2, DVec3};
use lerp::Lerp;
use lib_curveball::curve::CurveResult;
use lib_curveball::curve::extrude::path::Path;
use lib_curveball::curve::extrude::profile::Profile;
use lib_curveball::curve::extrude::{FrenetFrame, ProfileOrientation, extrude};
use lib_curveball::map::{
    entity::SimpleWorldspawn,
    geometry::Brush,
    qmap::{QEntity, QMap},
};

// This example creates a custom profile and path.

fn brushes_to_string(brushes: Vec<Brush>) -> CurveResult<String> {
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    let map = QMap::new(vec![entity]).with_tb_neverball_metadata();
    Ok(String::from(format!("{map}")))
}

// ==================== MyRevolve path ====================

#[derive(Debug, Clone)]
pub struct MyRevolve {
    radius: f64,
    start_angle_rad: f64,
    end_angle_rad: f64,
}

impl MyRevolve {
    pub fn new(radius: f64, start_angle: f64, end_angle: f64) -> Self {
        Self {
            radius,
            start_angle_rad: start_angle * PI / 180.0,
            end_angle_rad: end_angle * PI / 180.0,
        }
    }
}

impl Path for MyRevolve {
    fn point(&self, t: f64) -> DVec3 {
        let theta = self.start_angle_rad.lerp(self.end_angle_rad, t);
        DVec3 {
            x: self.radius * theta.cos(),
            y: self.radius * theta.sin(),
            z: 0.0,
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let theta = self.start_angle_rad.lerp(self.end_angle_rad, t);
        FrenetFrame {
            tangent: DVec3 {
                x: -theta.sin(),
                y: theta.cos(),
                z: 0.0,
            },
            normal: DVec3 {
                x: -theta.cos(),
                y: -theta.sin(),
                z: 0.0,
            },
            binormal: DVec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }
}

// ==================== MyRectangle profile ====================

#[derive(Debug, Clone)]
pub struct MyRectangle {
    width: f64,
    height: f64,
}

impl MyRectangle {
    pub fn new(width: f64, height: f64) -> MyRectangle {
        Self { width, height }
    }
}

impl Profile for MyRectangle {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        vec![
            DVec2 {
                x: self.width / 2.0,
                y: self.height / 2.0,
            },
            DVec2 {
                x: self.width / 2.0,
                y: -self.height / 2.0,
            },
            DVec2 {
                x: -self.width / 2.0,
                y: self.height / 2.0,
            },
            DVec2 {
                x: -self.width / 2.0,
                y: -self.height / 2.0,
            },
        ]
    }
}

// ==================== Produce the curve ====================

fn main() {
    // Create the rectangle profile
    let width = 32.0;
    let height = 8.0;
    let rectangle_profile = MyRectangle::new(width, height);

    // Create the catenary path
    let radius = 128.0;
    let start_angle = 0.0;
    let end_angle = 180.0;
    let revolve_profile = MyRevolve::new(radius, start_angle, end_angle);

    // Extrude to create the path
    let num_segments = 12;
    let profile_orientation = ProfileOrientation::FollowPath;
    let brushes = extrude(
        num_segments,
        &rectangle_profile,
        &revolve_profile,
        profile_orientation,
    )
    .unwrap();

    // Print the map out
    let string = brushes_to_string(brushes).unwrap();
    println!("{}", string);
}
