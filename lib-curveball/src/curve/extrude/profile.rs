// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [Profile] and [CompoundProfile] traits and a number of structs implementing those traits.

use std::f64::consts::PI;

use glam::DVec2;
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

/// A trait representing a 2D profile consisting of a single convex polygon.
pub trait Profile {
    fn profile(&self, t: f64) -> Vec<DVec2>;
}

// Make Box<dyn Profile> implement Profile
impl Profile for Box<dyn Profile + '_> {
    fn profile(&self, t: f64) -> Vec<DVec2> {
        (**self).profile(t)
    }
}

/// A trait reresenting a 2D profile consisting of multiple convex polygons.
pub trait CompoundProfile {
    fn compound_profile(&self, t: f64) -> Vec<Vec<DVec2>>;
}

// Make Box<dyn CompoundProfile> implement CompoundProfile
impl CompoundProfile for Box<dyn CompoundProfile + '_> {
    fn compound_profile(&self, t: f64) -> Vec<Vec<DVec2>> {
        (**self).compound_profile(t)
    }
}

// Every profile is also a compound profile of length 1.
impl<T> CompoundProfile for T
where
    T: Profile,
{
    fn compound_profile(&self, t: f64) -> Vec<Vec<DVec2>> {
        vec![self.profile(t)]
    }
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("{0}")]
    CircleError(#[from] CircleError),
    #[error("{0}")]
    CircleSectorError(#[from] CircleSectorError),
    #[error("{0}")]
    RectangleError(#[from] RectangleError),
    #[error("{0}")]
    ParallelogramError(#[from] ParallelogramError),
    #[error("{0}")]
    AnnulusError(#[from] AnnulusError),
}

// ==================== Circle ====================

/// A circle with a specified radius.
#[derive(Debug, Clone)]
pub struct Circle {
    n: u32,
    radius: f64,
}

impl Circle {
    pub fn new(n: u32, radius: f64) -> Result<Self, CircleError> {
        if n < 1 {
            return Err(CircleError::NotEnoughPoints { n });
        }
        if n > 4096 {
            return Err(CircleError::TooManyPoints { n });
        }
        Ok(Self { n, radius })
    }
}

impl Profile for Circle {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        0f64.lerp_iter(2.0 * PI, self.n as usize)
            .map(|theta| DVec2 {
                x: self.radius * theta.cos(),
                y: self.radius * theta.sin(),
            })
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum CircleError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
}

// ==================== Circular Sector ====================

/// A [circular sector](https://en.wikipedia.org/wiki/Circular_sector) with a specified radius,
/// start angle, and end angle.
#[derive(Debug, Clone)]
pub struct CircleSector {
    n: u32,
    radius: f64,
    start_angle_rad: f64,
    end_angle_rad: f64,
}

impl CircleSector {
    pub fn new(
        n: u32,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<Self, CircleSectorError> {
        if n < 1 {
            return Err(CircleSectorError::NotEnoughPoints { n });
        }
        if n > 4096 {
            return Err(CircleSectorError::TooManyPoints { n });
        }
        if start_angle >= end_angle {
            return Err(CircleSectorError::OrderedAngle);
        }
        if end_angle - start_angle > 180.0 {
            return Err(CircleSectorError::Limit180);
        }
        Ok(Self {
            n,
            radius,
            start_angle_rad: start_angle * PI / 180.0,
            end_angle_rad: end_angle * PI / 180.0,
        })
    }
}

impl Profile for CircleSector {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        self.start_angle_rad
            .lerp_iter_closed(self.end_angle_rad, self.n as usize + 1)
            .map(|theta| DVec2 {
                x: self.radius * theta.cos(),
                y: self.radius * theta.sin(),
            })
            .chain([DVec2::ZERO])
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum CircleSectorError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
    #[error("End angle must be greater than start angle.")]
    OrderedAngle,
    #[error("The difference between the start and end angle must be no greater than 180 degrees.")]
    Limit180,
}

// ==================== Rectangle ====================

/// An enumeration describing how a profile should be placed
/// relative to the path it is being extruded along.
///
/// For example, a value of `TopLeft` means the top-left corner
/// of a rectangle would be placed in the profile's origin, and
/// thus the top-left corner would be touching the path the profile
/// is being extruded along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anchor9Point {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// A rectangle with a specific width, height, and anchor point.
#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
    anchor: Anchor9Point,
}

impl Rectangle {
    pub fn new(width: f64, height: f64, anchor: Anchor9Point) -> Result<Self, RectangleError> {
        Ok(Self {
            width,
            height,
            anchor,
        })
    }
}

impl Profile for Rectangle {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        use Anchor9Point as RA;
        let hoffset = match self.anchor {
            RA::TopLeft | RA::CenterLeft | RA::BottomLeft => self.width / 2.0,
            RA::TopCenter | RA::Center | RA::BottomCenter => 0.0,
            RA::TopRight | RA::CenterRight | RA::BottomRight => -self.width / 2.0,
        };
        let voffset = match self.anchor {
            RA::TopLeft | RA::TopCenter | RA::TopRight => -self.height / 2.0,
            RA::CenterLeft | RA::Center | RA::CenterRight => 0.0,
            RA::BottomLeft | RA::BottomCenter | RA::BottomRight => self.height / 2.0,
        };
        let profile_fn = vec![
            DVec2 {
                x: hoffset + self.width / 2.0,
                y: voffset + self.height / 2.0,
            },
            DVec2 {
                x: hoffset + self.width / 2.0,
                y: voffset - self.height / 2.0,
            },
            DVec2 {
                x: hoffset - self.width / 2.0,
                y: voffset + self.height / 2.0,
            },
            DVec2 {
                x: hoffset - self.width / 2.0,
                y: voffset - self.height / 2.0,
            },
        ];
        profile_fn
    }
}

#[derive(Error, Debug)]
pub enum RectangleError {}

// ==================== Parallelogram ====================

/// A parallelogram.
#[derive(Debug, Clone)]
pub struct Parallelogram {
    width: f64,
    height: f64,
    offset_x: f64,
    offset_z: f64,
    anchor: Anchor9Point,
}

impl Parallelogram {
    pub fn new(
        width: f64,
        height: f64,
        offset_x: f64,
        offset_z: f64,
        anchor: Anchor9Point,
    ) -> Result<Self, ParallelogramError> {
        Ok(Self {
            width,
            height,
            offset_x,
            offset_z,
            anchor,
        })
    }
}

impl Profile for Parallelogram {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        use Anchor9Point as RA;
        let bottomleft = DVec2 { x: 0.0, y: 0.0 };
        let bottomright = DVec2 {
            x: self.width,
            y: self.height,
        };
        let topleft = DVec2 {
            x: self.offset_x,
            y: self.offset_z,
        };
        let topright = DVec2 {
            x: self.offset_x + self.width,
            y: self.offset_z + self.height,
        };
        let anchorpoint = match self.anchor {
            RA::BottomLeft => bottomleft,
            RA::BottomCenter => (bottomleft + bottomright) / 2.0,
            RA::BottomRight => bottomright,
            RA::CenterLeft => (bottomleft + topleft) / 2.0,
            RA::Center => (bottomleft + topright) / 2.0,
            RA::CenterRight => (bottomright + topright) / 2.0,
            RA::TopLeft => topleft,
            RA::TopCenter => (topleft + topright) / 2.0,
            RA::TopRight => topright,
        };

        vec![
            bottomleft - anchorpoint,
            bottomright - anchorpoint,
            topleft - anchorpoint,
            topright - anchorpoint,
        ]
    }
}

#[derive(Error, Debug)]
pub enum ParallelogramError {}

// ==================== Annulus ====================

/// A sector of an [annular ring](https://en.wikipedia.org/wiki/Annulus_(mathematics)).
pub struct Annulus {
    n: u32,
    inner_radius: f64,
    outer_radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl Annulus {
    pub fn new(
        n: u32,
        inner_radius: f64,
        outer_radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Result<Self, AnnulusError> {
        if n < 1 {
            return Err(AnnulusError::NotEnoughPoints { n })?;
        }
        if n > 4096 {
            return Err(AnnulusError::TooManyPoints { n })?;
        }

        Ok(Self {
            n,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
        })
    }
}

impl CompoundProfile for Annulus {
    fn compound_profile(&self, _t: f64) -> Vec<Vec<DVec2>> {
        let start_angle = self.start_angle * PI / 180.0;
        let end_angle = self.end_angle * PI / 180.0;

        start_angle
            .lerp_iter_closed(end_angle, self.n as usize + 1)
            .map(|theta| {
                let inner = DVec2 {
                    x: self.inner_radius * theta.cos(),
                    y: self.inner_radius * theta.sin(),
                };
                let outer = DVec2 {
                    x: self.outer_radius * theta.cos(),
                    y: self.outer_radius * theta.sin(),
                };
                [inner, outer]
            })
            .tuple_windows()
            .map(|(a1, a2)| {
                let [p1, p2] = a1;
                let [p3, p4] = a2;
                vec![p1, p2, p3, p4]
            })
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum AnnulusError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
}

// ==================== Arbitrary ====================

/// A profile defined by arbitrary sets of points defining convex polygons.
pub struct Arbitrary {
    points: Vec<Vec<DVec2>>,
}

impl Arbitrary {
    pub fn new(points: Vec<Vec<DVec2>>) -> Arbitrary {
        Self { points }
    }
}

impl CompoundProfile for Arbitrary {
    fn compound_profile(&self, _t: f64) -> Vec<Vec<DVec2>> {
        self.points.clone()
    }
}
