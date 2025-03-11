// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec2;
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

pub trait Profile {
    fn profile(&self, t: f64) -> Vec<DVec2>;
}

// Make Box<dyn Profile> implement Profile
impl Profile for Box<dyn Profile + '_> {
    fn profile(&self, t: f64) -> Vec<DVec2> {
        (**self).profile(t)
    }
}

// A profile consisting of multiple convex polygons.
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
    RectangleError(#[from] RectangleError),
    #[error("{0}")]
    AnnulusError(#[from] AnnulusError),
}

// ==================== Circle ====================

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

// ==================== Rectangle ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RectangleAnchor {
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

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
    anchor: RectangleAnchor,
}

impl Rectangle {
    pub fn new(width: f64, height: f64, anchor: RectangleAnchor) -> Result<Self, RectangleError> {
        Ok(Self {
            width,
            height,
            anchor,
        })
    }
}

impl Profile for Rectangle {
    fn profile(&self, _t: f64) -> Vec<DVec2> {
        use RectangleAnchor as RA;
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

// ==================== Annulus ====================

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
