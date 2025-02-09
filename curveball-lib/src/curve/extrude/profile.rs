// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec3;
use lerp::LerpIter;
use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

pub fn circle(n: u32, radius: f64) -> ProfileResult<impl Fn(f64) -> Vec<DVec3>> {
    if n < 1 {
        return Err(ProfileError::NotEnoughPoints { n });
    }
    if n > 4096 {
        return Err(ProfileError::TooManyPoints { n });
    }
    let profile_fn = move |_| {
        0f64.lerp_iter(2.0 * PI, n as usize)
            .map(|theta| DVec3 {
                x: 0.0,
                y: radius * theta.cos(),
                z: radius * theta.sin(),
            })
            .collect()
    };
    Ok(profile_fn)
}

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

pub fn rectangle(
    width: f64,
    height: f64,
    anchor: RectangleAnchor,
) -> ProfileResult<impl Fn(f64) -> Vec<DVec3>> {
    use RectangleAnchor as RA;
    let hoffset = match anchor {
        RA::TopLeft | RA::CenterLeft | RA::BottomLeft => -width / 2.0,
        RA::TopCenter | RA::Center | RA::BottomCenter => 0.0,
        RA::TopRight | RA::CenterRight | RA::BottomRight => width / 2.0,
    };
    let voffset = match anchor {
        RA::TopLeft | RA::TopCenter | RA::TopRight => -height / 2.0,
        RA::CenterLeft | RA::Center | RA::CenterRight => 0.0,
        RA::BottomLeft | RA::BottomCenter | RA::BottomRight => height / 2.0,
    };
    let profile_fn = move |_| {
        vec![
            DVec3::new(0.0, hoffset + width / 2.0, voffset + height / 2.0),
            DVec3::new(0.0, hoffset + width / 2.0, voffset - height / 2.0),
            DVec3::new(0.0, hoffset - width / 2.0, voffset + height / 2.0),
            DVec3::new(0.0, hoffset - width / 2.0, voffset - height / 2.0),
        ]
    };
    Ok(profile_fn)
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
}
