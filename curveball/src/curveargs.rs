// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use bevy::prelude::*;

use glam::{DVec2, DVec3};

use curveball_lib::curve::{
    Curve, CurveResult, bank::Bank, catenary::Catenary, curve_classic::CurveClassic,
    curve_slope::CurveSlope, extrude, rayto::Rayto, serpentine::Serpentine,
    serpentine::SerpentineOffsetMode,
};
use curveball_lib::map::geometry::Brush;

#[derive(Resource, Debug, Clone, PartialEq, PartialOrd)]
pub enum CurveSelect {
    CurveClassic(CurveClassicArgs),
    CurveSlope(CurveSlopeArgs),
    Rayto(RaytoArgs),
    Bank(BankArgs),
    Catenary(CatenaryArgs),
    Serpentine(SerpentineArgs),
    Extrusion(ExtrusionArgs),
}

impl Default for CurveSelect {
    fn default() -> Self {
        Self::Bank(BankArgs::default())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CurveClassicArgs {
    pub n: u32,
    pub ri0: f64,
    pub ro0: f64,
    pub ri1: f64,
    pub ro1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub t: f64,
}

impl Default for CurveClassicArgs {
    fn default() -> Self {
        Self {
            n: 24,
            ri0: 32.0,
            ro0: 64.0,
            ri1: 32.0,
            ro1: 64.0,
            theta0: 0.0,
            theta1: 90.0,
            t: 8.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CurveSlopeArgs {
    pub n: u32,
    pub ri0: f64,
    pub ro0: f64,
    pub ri1: f64,
    pub ro1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub en_const_thickness: bool,
    pub t_const_thickness: f64,
    pub height_inner_top_0: f64,
    pub height_inner_bot_0: f64,
    pub height_outer_top_0: f64,
    pub height_outer_bot_0: f64,
    pub height_inner_top_1: f64,
    pub height_inner_bot_1: f64,
    pub height_outer_top_1: f64,
    pub height_outer_bot_1: f64,
    // pub height_link_start_end: bool, // TODO
    pub height_link_inner_outer: bool,
    pub hill_inner_top: f64,
    pub hill_inner_bot: f64,
    pub hill_outer_top: f64,
    pub hill_outer_bot: f64,
    pub hill_link_inner_outer: bool,
}

impl Default for CurveSlopeArgs {
    fn default() -> Self {
        Self {
            n: 24,
            ri0: 32.0,
            ro0: 64.0,
            ri1: 32.0,
            ro1: 64.0,
            theta0: 0.0,
            theta1: 180.0,
            en_const_thickness: true,
            t_const_thickness: 8.0,
            height_inner_top_0: 0.0,
            height_inner_bot_0: 0.0,
            height_outer_top_0: 0.0,
            height_outer_bot_0: 0.0,
            height_inner_top_1: 32.0,
            height_inner_bot_1: 24.0,
            height_outer_top_1: 32.0,
            height_outer_bot_1: 24.0,
            // height_link_start_end: true, // TODO
            height_link_inner_outer: true,
            hill_inner_top: 0.0,
            hill_inner_bot: 0.0,
            hill_outer_top: 0.0,
            hill_outer_bot: 0.0,
            hill_link_inner_outer: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RaytoArgs {
    pub n: u32,
    pub r0: f64,
    pub r1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub x: f64,
    pub y: f64,
    pub h: f64,
}

impl Default for RaytoArgs {
    fn default() -> Self {
        Self {
            n: 12,
            r0: 32.0,
            r1: 32.0,
            theta0: 0.0,
            theta1: 90.0,
            x: 32.0,
            y: 32.0,
            h: 8.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BankArgs {
    pub n: u32,
    pub ri: f64,
    pub ro: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub h: f64,
    pub t: f64,
    pub fill: bool,
}

impl Default for BankArgs {
    fn default() -> Self {
        Self {
            n: 24,
            ri: 64.0,
            ro: 128.0,
            theta0: 0.0,
            theta1: 90.0,
            h: 64.0,
            t: 8.0,
            fill: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CatenaryArgs {
    pub n: u32,
    pub span: f64,
    pub height: f64,
    pub s: f64,
    pub w: f64,
    pub t: f64,
    pub initial_guess: Option<f64>,
}

impl Default for CatenaryArgs {
    fn default() -> Self {
        Self {
            n: 24,
            span: 128.0,
            height: 0.0,
            s: 132.0,
            w: 32.0,
            t: 4.0,
            initial_guess: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SerpentineArgs {
    pub n: u32,
    pub x: f64,
    pub z: f64,
    pub w: f64,
    pub t: f64,
}

impl Default for SerpentineArgs {
    fn default() -> Self {
        Self {
            n: 24,
            x: 128.0,
            z: 64.0,
            w: 32.0,
            t: 8.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExtrusionArgs {
    pub profile: ProfileSelect,
    pub path: PathSelect,
    pub profile_orientation: extrude::ProfileOrientation,
}

impl Default for ExtrusionArgs {
    fn default() -> Self {
        Self {
            profile: ProfileSelect::default(),
            path: PathSelect::default(),
            profile_orientation: extrude::ProfileOrientation::FollowPath,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ProfileSelect {
    Circle(ProfileCircleArgs),
    Rectangle(ProfileRectangleArgs),
    Annulus(ProfileAnnulusArgs),
}

impl Default for ProfileSelect {
    fn default() -> Self {
        Self::Circle(ProfileCircleArgs::default())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProfileCircleArgs {
    pub n: u32,
    pub radius: f64,
}

impl Default for ProfileCircleArgs {
    fn default() -> Self {
        Self {
            n: 12,
            radius: 16.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProfileRectangleArgs {
    pub width: f64,
    pub height: f64,
    pub anchor: extrude::profile::RectangleAnchor,
}

impl Default for ProfileRectangleArgs {
    fn default() -> Self {
        Self {
            width: 32.0,
            height: 8.0,
            anchor: extrude::profile::RectangleAnchor::BottomLeft,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProfileAnnulusArgs {
    pub n: u32,
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Default for ProfileAnnulusArgs {
    fn default() -> Self {
        Self {
            n: 12,
            inner_radius: 48.0,
            outer_radius: 64.0,
            start_angle: 0.0,
            end_angle: 90.0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PathSelect {
    Line(PathLineArgs),
    Revolve(PathRevolveArgs),
}

impl Default for PathSelect {
    fn default() -> Self {
        Self::Revolve(PathRevolveArgs::default())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathLineArgs {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for PathLineArgs {
    fn default() -> Self {
        Self {
            x: 64.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathRevolveArgs {
    pub path_n: u32,
    pub path_start: f64,
    pub path_end: f64,
    pub radius: f64,
}

impl Default for PathRevolveArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            path_start: 0.0,
            path_end: 90.0,
            radius: 32.0,
        }
    }
}

impl CurveSelect {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        let brushes = match self {
            Self::CurveClassic(args) => CurveClassic {
                n: args.n,
                ri0: args.ri0,
                ro0: args.ro0,
                ri1: args.ri1,
                ro1: args.ro1,
                theta0: args.theta0,
                theta1: args.theta1,
                t: args.t,
            }
            .bake()?,
            Self::CurveSlope(args) => CurveSlope {
                n: args.n,
                ri0: args.ri0,
                ro0: args.ro0,
                ri1: args.ri1,
                ro1: args.ro1,
                theta0: args.theta0,
                theta1: args.theta1,
                height_inner_top_0: args.height_inner_top_0,
                height_inner_bot_0: args.height_inner_bot_0,
                height_outer_top_0: args.height_outer_top_0,
                height_outer_bot_0: args.height_outer_bot_0,
                height_inner_top_1: args.height_inner_top_1,
                height_inner_bot_1: args.height_inner_bot_1,
                height_outer_top_1: args.height_outer_top_1,
                height_outer_bot_1: args.height_outer_bot_1,
                hill_inner_top: args.hill_inner_top,
                hill_inner_bot: args.hill_inner_bot,
                hill_outer_top: args.hill_outer_top,
                hill_outer_bot: args.hill_outer_bot,
            }
            .bake()?,
            Self::Rayto(args) => Rayto {
                n: args.n,
                r0: args.r0,
                r1: args.r1,
                theta0: args.theta0,
                theta1: args.theta1,
                x: args.x,
                y: args.y,
                h: args.h,
            }
            .bake()?,
            Self::Bank(args) => Bank {
                n: args.n,
                ri: args.ri,
                ro: args.ro,
                theta0: args.theta0,
                theta1: args.theta1,
                h: args.h,
                t: args.t,
                fill: args.fill,
            }
            .bake()?,
            Self::Catenary(args) => Catenary {
                n: args.n,
                span: args.span,
                height: args.height,
                s: args.s,
                w: args.w,
                t: args.t,
                initial_guess: args.initial_guess,
            }
            .bake()?,
            Self::Serpentine(args) => Serpentine {
                n_each: args.n.div_ceil(2),
                x: args.x,
                z: args.z,
                w: args.w,
                t: args.t,
                offset: SerpentineOffsetMode::Middle,
            }
            .bake()?,
            Self::Extrusion(args) => Self::extrude_brushes(args)?,
        };
        Ok(brushes)
    }

    fn extrude_brushes(args: &ExtrusionArgs) -> CurveResult<Vec<Brush>> {
        let profile: Vec<Vec<DVec2>> = match &args.profile {
            ProfileSelect::Circle(args) => {
                let profile = extrude::profile::circle(args.n, args.radius)?;
                vec![profile]
            }
            ProfileSelect::Rectangle(args) => {
                let profile = extrude::profile::rectangle(args.width, args.height, args.anchor)?;
                vec![profile]
            }
            ProfileSelect::Annulus(args) => extrude::profile::annulus(
                args.n,
                args.inner_radius,
                args.outer_radius,
                args.start_angle,
                args.end_angle,
            )?,
        };
        let (path_fn, frenet_fn): (
            Box<dyn Fn(f64) -> DVec3>,
            Box<dyn Fn(f64) -> extrude::FrenetFrame>,
        ) = match &args.path {
            PathSelect::Line(args) => {
                let (path_fn, frenet_fn) = extrude::path::line(args.x, args.y, args.z)?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
            PathSelect::Revolve(args) => {
                let (path_fn, frenet_fn) = extrude::path::revolve(args.radius)?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
        };

        let path_n = match &args.path {
            PathSelect::Line(_args) => 1,
            PathSelect::Revolve(args) => args.path_n,
        };

        let path_start = match &args.path {
            PathSelect::Line(_args) => 0.0,
            PathSelect::Revolve(args) => args.path_start,
        };

        let path_end = match &args.path {
            PathSelect::Line(_args) => 1.0,
            PathSelect::Revolve(args) => args.path_end,
        };
        extrude::extrude_multi(
            path_n,
            &profile,
            path_fn,
            frenet_fn,
            path_start,
            path_end,
            args.profile_orientation,
        )
    }
}
