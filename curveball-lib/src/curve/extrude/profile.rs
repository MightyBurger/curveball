// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec2;
use lerp::LerpIter;
use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

pub fn circle(n: u32, radius: f64) -> ProfileResult<Vec<DVec2>> {
    if n < 1 {
        return Err(ProfileError::NotEnoughPoints { n });
    }
    if n > 4096 {
        return Err(ProfileError::TooManyPoints { n });
    }
    let result = 0f64
        .lerp_iter(2.0 * PI, n as usize)
        .map(|theta| DVec2 {
            x: radius * theta.cos(),
            y: radius * theta.sin(),
        })
        .collect();
    Ok(result)
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

pub fn rectangle(width: f64, height: f64, anchor: RectangleAnchor) -> ProfileResult<Vec<DVec2>> {
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
    Ok(vec![
        DVec2::new(hoffset + width / 2.0, voffset + height / 2.0),
        DVec2::new(hoffset + width / 2.0, voffset - height / 2.0),
        DVec2::new(hoffset - width / 2.0, voffset + height / 2.0),
        DVec2::new(hoffset - width / 2.0, voffset - height / 2.0),
    ])
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
}
