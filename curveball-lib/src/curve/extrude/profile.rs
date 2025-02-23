// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec2;
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

pub fn circle(n: u32, radius: f64) -> ProfileResult<impl Fn(f64) -> Vec<DVec2>> {
    if n < 1 {
        return Err(ProfileError::NotEnoughPoints { n });
    }
    if n > 4096 {
        return Err(ProfileError::TooManyPoints { n });
    }
    let profile_fn = move |_| {
        0f64.lerp_iter(2.0 * PI, n as usize)
            .map(|theta| DVec2 {
                x: radius * theta.cos(),
                y: radius * theta.sin(),
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
) -> ProfileResult<impl Fn(f64) -> Vec<DVec2>> {
    use RectangleAnchor as RA;
    let hoffset = match anchor {
        RA::TopLeft | RA::CenterLeft | RA::BottomLeft => width / 2.0,
        RA::TopCenter | RA::Center | RA::BottomCenter => 0.0,
        RA::TopRight | RA::CenterRight | RA::BottomRight => -width / 2.0,
    };
    let voffset = match anchor {
        RA::TopLeft | RA::TopCenter | RA::TopRight => -height / 2.0,
        RA::CenterLeft | RA::Center | RA::CenterRight => 0.0,
        RA::BottomLeft | RA::BottomCenter | RA::BottomRight => height / 2.0,
    };
    let profile_fn = move |_| {
        vec![
            DVec2 {
                x: hoffset + width / 2.0,
                y: voffset + height / 2.0,
            },
            DVec2 {
                x: hoffset + width / 2.0,
                y: voffset - height / 2.0,
            },
            DVec2 {
                x: hoffset - width / 2.0,
                y: voffset + height / 2.0,
            },
            DVec2 {
                x: hoffset - width / 2.0,
                y: voffset - height / 2.0,
            },
        ]
    };
    Ok(profile_fn)
}

pub fn annulus(
    n: u32,
    inner_radius: f64,
    outer_radius: f64,
    mut start_angle: f64,
    mut end_angle: f64,
) -> ProfileResult<Vec<Box<dyn Fn(f64) -> Vec<DVec2>>>> {
    if n < 1 {
        return Err(ProfileError::NotEnoughPoints { n });
    }
    if n > 4096 {
        return Err(ProfileError::TooManyPoints { n });
    }
    let mut profile_fn_multi: Vec<Box<dyn Fn(f64) -> Vec<DVec2>>> = Vec::new();

    start_angle = start_angle * PI / 180.0;
    end_angle = end_angle * PI / 180.0;

    for points in start_angle
        .lerp_iter_closed(end_angle, n as usize + 1)
        .map(|theta| {
            let inner = DVec2 {
                x: inner_radius * theta.cos(),
                y: inner_radius * theta.sin(),
            };
            let outer = DVec2 {
                x: outer_radius * theta.cos(),
                y: outer_radius * theta.sin(),
            };
            [inner, outer]
        })
        .tuple_windows()
        .map(|(a1, a2)| {
            let [p1, p2] = a1;
            let [p3, p4] = a2;
            [p1, p2, p3, p4]
        })
    {
        let pvec = Box::new(move |_| points.to_vec());
        profile_fn_multi.push(pvec);
    }

    Ok(profile_fn_multi)
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("n = {n}. Number of points must be at least 1.")]
    NotEnoughPoints { n: u32 },
    #[error("n = {n}. Number of points must be no greater than 4096.")]
    TooManyPoints { n: u32 },
}
