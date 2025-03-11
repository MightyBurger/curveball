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
// To create a custom path, make a struct that implements the
// Path trait. Doing so requires you to define two functions
// that take in t: point(), and frame().
//
// extrude() will continually call point() and frame() with
// the parameter t gradually increasing from 0.0 up to 1.0.
//
// frame() is required to use extrude() with
// ProfileOrientation::FollowPath. If you plan to just use
// ProfileOrientation::Constant, just have frame() return
// some placeholder vectors like DVec3::default().
//
// To know what the different values of ProfileOrientation
// do, play around with the Curveball app. It'll be much
// easier to understand than if I explained it here.
// ========================================================

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
// To create a custom profile, make a struct that implements the
// Profile trait. Doing so requires you to define one function
// that takes in t and returns a Vec<DVec2>: profile().
//
// extrude() will continually call profile() with
// the parameter t gradually increasing from 0.0 up to 1.0.
//
// Often, your profile will just be a constant shape and will
// not depend on t, so you can just leave this parameter unused.
// The example profile below does this.
//
// The returned profile must be a convex polygon.
//
// You can also create a profile that returns multiple polygons
// at each step. The Annulus profile in Curveball does this.
// To do this, implement CompoundProfile instead of Profile.
// ============================================================

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
