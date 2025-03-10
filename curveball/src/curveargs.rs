// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use bevy::prelude::*;
use curveball_lib::curve::extrude::ProfileOrientation;

use glam::{DVec2, DVec3};

use curveball_lib::curve::{
    Curve, CurveResult, bank::Bank, catenary::Catenary, curve_classic::CurveClassic,
    curve_slope::CurveSlope, extrude, rayto::Rayto, serpentine::Serpentine,
    serpentine::SerpentineOffsetMode,
};
use curveball_lib::map::geometry::Brush;

// This struct is the heart of Curveball.
// All the buttons and sliders (everything in src/gui/mod.rs) adjust the values in this struct.
// The system in src/brushes.rs monitors for changes in this struct. When it detects one, it calls brushes()
// to convert these arguments into a curve.
#[derive(Resource, Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct CurveArgs {
    pub selected_curve: SelectedCurve,
    pub curveclassic_args: CurveClassicArgs,
    pub curveslope_args: CurveSlopeArgs,
    pub rayto_args: RaytoArgs,
    pub bank_args: BankArgs,
    pub catenary_args: CatenaryArgs,
    pub serpentine_args: SerpentineArgs,
    pub extrusion_args: ExtrusionArgs,
}

impl CurveArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        use SelectedCurve as SC;
        match self.selected_curve {
            SC::CurveClassic => self.curveclassic_args.brushes(),
            SC::CurveSlope => self.curveslope_args.brushes(),
            SC::Rayto => self.rayto_args.brushes(),
            SC::Bank => self.bank_args.brushes(),
            SC::Catenary => self.catenary_args.brushes(),
            SC::Serpentine => self.serpentine_args.brushes(),
            SC::Extrusion => self.extrusion_args.brushes(),
        }
    }
}

// -------------------------------------------------------- SelectedCurve

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SelectedCurve {
    CurveClassic,
    CurveSlope,
    Rayto,
    Bank,
    Catenary,
    Serpentine,
    Extrusion,
}

impl Default for SelectedCurve {
    fn default() -> Self {
        Self::Bank
    }
}

impl std::fmt::Display for SelectedCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurveClassic => write!(f, "Curve Classic"),
            Self::CurveSlope => write!(f, "Curve Slope"),
            Self::Rayto => write!(f, "Rayto"),
            Self::Bank => write!(f, "Bank"),
            Self::Catenary => write!(f, "Catenary"),
            Self::Serpentine => write!(f, "Serpentine"),
            Self::Extrusion => write!(f, "Extrusion"),
        }
    }
}

// -------------------------------------------------------- CurveClassicArgs

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

impl CurveClassicArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        CurveClassic {
            n: self.n,
            ri0: self.ri0,
            ro0: self.ro0,
            ri1: self.ri1,
            ro1: self.ro1,
            theta0: self.theta0,
            theta1: self.theta1,
            t: self.t,
        }
        .bake()
    }
}

// -------------------------------------------------------- CurveSlopeArgs

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
            height_link_inner_outer: true,
            hill_inner_top: 0.0,
            hill_inner_bot: 0.0,
            hill_outer_top: 0.0,
            hill_outer_bot: 0.0,
            hill_link_inner_outer: true,
        }
    }
}

impl CurveSlopeArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        CurveSlope {
            n: self.n,
            ri0: self.ri0,
            ro0: self.ro0,
            ri1: self.ri1,
            ro1: self.ro1,
            theta0: self.theta0,
            theta1: self.theta1,
            height_inner_top_0: self.height_inner_top_0,
            height_inner_bot_0: self.height_inner_bot_0,
            height_outer_top_0: self.height_outer_top_0,
            height_outer_bot_0: self.height_outer_bot_0,
            height_inner_top_1: self.height_inner_top_1,
            height_inner_bot_1: self.height_inner_bot_1,
            height_outer_top_1: self.height_outer_top_1,
            height_outer_bot_1: self.height_outer_bot_1,
            hill_inner_top: self.hill_inner_top,
            hill_inner_bot: self.hill_inner_bot,
            hill_outer_top: self.hill_outer_top,
            hill_outer_bot: self.hill_outer_bot,
        }
        .bake()
    }
}

// -------------------------------------------------------- RaytoArgs

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

impl RaytoArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        Rayto {
            n: self.n,
            r0: self.r0,
            r1: self.r1,
            theta0: self.theta0,
            theta1: self.theta1,
            x: self.x,
            y: self.y,
            h: self.h,
        }
        .bake()
    }
}

// -------------------------------------------------------- BankArgs

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

impl BankArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        Bank {
            n: self.n,
            ri: self.ri,
            ro: self.ro,
            theta0: self.theta0,
            theta1: self.theta1,
            h: self.h,
            t: self.t,
            fill: self.fill,
        }
        .bake()
    }
}

// -------------------------------------------------------- CatenaryArgs

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

impl CatenaryArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        Catenary {
            n: self.n,
            span: self.span,
            height: self.height,
            s: self.s,
            w: self.w,
            t: self.t,
            initial_guess: self.initial_guess,
        }
        .bake()
    }
}

// -------------------------------------------------------- SerpentineArgs

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

impl SerpentineArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        Serpentine {
            n_each: self.n.div_ceil(2),
            x: self.x,
            z: self.z,
            w: self.w,
            t: self.t,
            offset: SerpentineOffsetMode::Middle,
        }
        .bake()
    }
}

// -----------------------------------------------------------------------------
//                              Extrusion
// -----------------------------------------------------------------------------

// -------------------------------------------------------- ExtrusionArgs

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct ExtrusionArgs {
    pub selected_profile: SelectedProfile,
    pub profile_circle_args: ProfileCircleArgs,
    pub profile_rectangle_args: ProfileRectangleArgs,
    pub profile_annulus_args: ProfileAnnulusArgs,
    pub selected_path: SelectedPath,
    pub path_line_args: PathLineArgs,
    pub path_revolve_args: PathRevolveArgs,
    pub path_sinusoid_args: PathSinusoidArgs,
    pub profile_orientation: ProfileOrientation,
}

impl ExtrusionArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        let profile: Vec<Vec<DVec2>> = match self.selected_profile {
            SelectedProfile::Circle => {
                let profile = extrude::profile::circle(
                    self.profile_circle_args.n,
                    self.profile_circle_args.radius,
                )?;
                vec![profile]
            }
            SelectedProfile::Rectangle => {
                let profile = extrude::profile::rectangle(
                    self.profile_rectangle_args.width,
                    self.profile_rectangle_args.height,
                    self.profile_rectangle_args.anchor,
                )?;
                vec![profile]
            }
            SelectedProfile::Annulus => extrude::profile::annulus(
                self.profile_annulus_args.n,
                self.profile_annulus_args.inner_radius,
                self.profile_annulus_args.outer_radius,
                self.profile_annulus_args.start_angle,
                self.profile_annulus_args.end_angle,
            )?,
        };

        let (path_fn, frenet_fn): (
            Box<dyn Fn(f64) -> DVec3>,
            Box<dyn Fn(f64) -> extrude::FrenetFrame>,
        ) = match self.selected_path {
            SelectedPath::Line => {
                let (path_fn, frenet_fn) = extrude::path::line(
                    self.path_line_args.x,
                    self.path_line_args.y,
                    self.path_line_args.z,
                )?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
            SelectedPath::Revolve => {
                let (path_fn, frenet_fn) = extrude::path::revolve(self.path_revolve_args.radius)?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
            SelectedPath::Sinusoid => {
                let (path_fn, frenet_fn) = extrude::path::sinusoid(
                    self.path_sinusoid_args.amplitude,
                    self.path_sinusoid_args.period,
                    self.path_sinusoid_args.phase,
                )?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
        };

        let path_n = match self.selected_path {
            SelectedPath::Line => 1,
            SelectedPath::Revolve => self.path_revolve_args.path_n,
            SelectedPath::Sinusoid => self.path_sinusoid_args.path_n,
        };

        let path_start = match self.selected_path {
            SelectedPath::Line => 0.0,
            SelectedPath::Revolve => self.path_revolve_args.path_start,
            SelectedPath::Sinusoid => self.path_sinusoid_args.path_start,
        };

        let path_end = match self.selected_path {
            SelectedPath::Line => 1.0,
            SelectedPath::Revolve => self.path_revolve_args.path_end,
            SelectedPath::Sinusoid => self.path_sinusoid_args.path_end,
        };
        extrude::extrude_multi(
            path_n,
            &profile,
            path_fn,
            frenet_fn,
            path_start,
            path_end,
            self.profile_orientation,
        )
    }
}

// -------------------------------------------------------- SelectedProfile

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SelectedProfile {
    Circle,
    Rectangle,
    Annulus,
}

impl std::fmt::Display for SelectedProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Rectangle => write!(f, "Rectangle"),
            Self::Annulus => write!(f, "Annulus"),
        }
    }
}

impl Default for SelectedProfile {
    fn default() -> Self {
        Self::Circle
    }
}

// -------------------------------------------------------- ProfileCircleArgs

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

// -------------------------------------------------------- ProfileRectangleArgs

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

// -------------------------------------------------------- ProfileAnnulusArgs

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
            inner_radius: 12.0,
            outer_radius: 16.0,
            start_angle: 0.0,
            end_angle: 360.0,
        }
    }
}

// -------------------------------------------------------- SelectedPath

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SelectedPath {
    Line,
    Revolve,
    Sinusoid,
}

impl std::fmt::Display for SelectedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line => write!(f, "Line"),
            Self::Revolve => write!(f, "Revolve"),
            Self::Sinusoid => write!(f, "Sinusoid"),
        }
    }
}

impl Default for SelectedPath {
    fn default() -> Self {
        Self::Revolve
    }
}

// -------------------------------------------------------- PathLineArgs

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

// -------------------------------------------------------- PathRevolveArgs

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
            radius: 64.0,
        }
    }
}

// -------------------------------------------------------- PathSinusoidArgs

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathSinusoidArgs {
    pub path_n: u32,
    pub path_start: f64,
    pub path_end: f64,
    pub amplitude: f64,
    pub period: f64,
    pub phase: f64,
}

impl Default for PathSinusoidArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            path_start: 0.0,
            path_end: 128.0,
            amplitude: 32.0,
            period: 128.0,
            phase: 0.0,
        }
    }
}
