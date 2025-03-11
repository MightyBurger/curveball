// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use bevy::prelude::*;
use glam::DVec2;
use lib_curveball::curve::extrude::ProfileOrientation;
use lib_curveball::curve::extrude::path::PathResult;
use lib_curveball::curve::extrude::profile::ProfileResult;

use lib_curveball::curve::{
    Curve, CurveResult, bank::Bank, curve_classic::CurveClassic, curve_slope::CurveSlope, extrude,
    rayto::Rayto,
};
use lib_curveball::map::geometry::Brush;

// This struct is the heart of Curveball.
// All the buttons and sliders (everything in src/gui/mod.rs) adjust the values in this struct.
// The system in src/brushes.rs monitors for changes in this struct. When it detects one, it calls brushes()
// to convert these arguments into a curve.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct CurveArgs {
    pub selected_curve: SelectedCurve,
    pub curveclassic_args: CurveClassicArgs,
    pub curveslope_args: CurveSlopeArgs,
    pub rayto_args: RaytoArgs,
    pub bank_args: BankArgs,
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

// -----------------------------------------------------------------------------
//                              Extrusion
// -----------------------------------------------------------------------------

// -------------------------------------------------------- ExtrusionArgs

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExtrusionArgs {
    pub selected_profile: SelectedProfile,
    pub profile_circle_args: ProfileCircleArgs,
    pub profile_circle_sector_args: ProfileCircleSectorArgs,
    pub profile_rectangle_args: ProfileRectangleArgs,
    pub profile_annulus_args: ProfileAnnulusArgs,
    pub profile_arbitrary_args: ProfileArbitraryArgs,
    pub selected_path: SelectedPath,
    pub path_line_args: PathLineArgs,
    pub path_revolve_args: PathRevolveArgs,
    pub path_sinusoid_args: PathSinusoidArgs,
    pub path_bezier_args: PathBezierArgs,
    pub path_catenary_args: PathCatenaryArgs,
    pub path_serpentine_args: PathSerpentineArgs,
    pub profile_orientation: ProfileOrientation,
}

impl ExtrusionArgs {
    pub fn brushes(&self) -> CurveResult<Vec<Brush>> {
        let profile: Box<dyn extrude::profile::CompoundProfile> = match self.selected_profile {
            SelectedProfile::Circle => Box::new(self.profile_circle_args.profiles()?),
            SelectedProfile::CircleSector => Box::new(self.profile_circle_sector_args.profiles()?),
            SelectedProfile::Rectangle => Box::new(self.profile_rectangle_args.profiles()?),
            SelectedProfile::Annulus => Box::new(self.profile_annulus_args.profiles()?),
            SelectedProfile::Arbitrary => Box::new(self.profile_arbitrary_args.profiles()),
        };

        let path: Box<dyn extrude::path::Path> = match self.selected_path {
            SelectedPath::Line => Box::new(self.path_line_args.path()),
            SelectedPath::Revolve => Box::new(self.path_revolve_args.path()),
            SelectedPath::Sinusoid => Box::new(self.path_sinusoid_args.path()?),
            SelectedPath::Bezier => Box::new(self.path_bezier_args.path()?),
            SelectedPath::Catenary => Box::new(self.path_catenary_args.path()?),
            SelectedPath::Serpentine => Box::new(self.path_serpentine_args.path()?),
        };

        let path_n = match self.selected_path {
            SelectedPath::Line => 1,
            SelectedPath::Revolve => self.path_revolve_args.path_n,
            SelectedPath::Sinusoid => self.path_sinusoid_args.path_n,
            SelectedPath::Bezier => self.path_bezier_args.path_n,
            SelectedPath::Catenary => self.path_catenary_args.path_n,
            SelectedPath::Serpentine => self.path_serpentine_args.path_n,
        };

        extrude::extrude_multi(path_n, &profile, &path, self.profile_orientation)
    }
}

// -------------------------------------------------------- SelectedProfile

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SelectedProfile {
    Circle,
    CircleSector,
    Rectangle,
    Annulus,
    Arbitrary,
}

impl std::fmt::Display for SelectedProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::CircleSector => write!(f, "Circle Sector"),
            Self::Rectangle => write!(f, "Rectangle"),
            Self::Annulus => write!(f, "Annulus"),
            Self::Arbitrary => write!(f, "Arbitrary"),
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

impl ProfileCircleArgs {
    pub fn profiles(&self) -> ProfileResult<extrude::profile::Circle> {
        Ok(extrude::profile::Circle::new(self.n, self.radius)?)
    }
}

// -------------------------------------------------------- ProfileCircleArgs

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ProfileCircleSectorArgs {
    pub n: u32,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Default for ProfileCircleSectorArgs {
    fn default() -> Self {
        Self {
            n: 12,
            radius: 16.0,
            start_angle: 0.0,
            end_angle: 90.0,
        }
    }
}

impl ProfileCircleSectorArgs {
    pub fn profiles(&self) -> ProfileResult<extrude::profile::CircleSector> {
        Ok(extrude::profile::CircleSector::new(
            self.n,
            self.radius,
            self.start_angle,
            self.end_angle,
        )?)
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

impl ProfileRectangleArgs {
    pub fn profiles(&self) -> ProfileResult<extrude::profile::Rectangle> {
        Ok(extrude::profile::Rectangle::new(
            self.width,
            self.height,
            self.anchor,
        )?)
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

impl ProfileAnnulusArgs {
    pub fn profiles(&self) -> ProfileResult<extrude::profile::Annulus> {
        Ok(extrude::profile::Annulus::new(
            self.n,
            self.inner_radius,
            self.outer_radius,
            self.start_angle,
            self.end_angle,
        )?)
    }
}

// -------------------------------------------------------- ProfileArbitraryArgs

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileArbitraryArgs {
    pub polygons: Vec<Vec<DVec2>>,
}

impl Default for ProfileArbitraryArgs {
    fn default() -> Self {
        Self {
            polygons: vec![vec![
                DVec2 { x: 0.0, y: 0.0 },
                DVec2 { x: 32.0, y: 0.0 },
                DVec2 { x: 32.0, y: 32.0 },
            ]],
        }
    }
}

impl ProfileArbitraryArgs {
    pub fn profiles(&self) -> extrude::profile::Arbitrary {
        extrude::profile::Arbitrary::new(self.polygons.clone())
    }
}

// -------------------------------------------------------- SelectedPath

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SelectedPath {
    Line,
    Revolve,
    Sinusoid,
    Bezier,
    Catenary,
    Serpentine,
}

impl std::fmt::Display for SelectedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line => write!(f, "Line"),
            Self::Revolve => write!(f, "Revolve"),
            Self::Sinusoid => write!(f, "Sinusoid"),
            Self::Bezier => write!(f, "Bezier"),
            Self::Catenary => write!(f, "Catenary"),
            Self::Serpentine => write!(f, "Serpentine"),
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

impl PathLineArgs {
    fn path(&self) -> extrude::path::Line {
        extrude::path::Line::new(self.x, self.y, self.z)
    }
}

// -------------------------------------------------------- PathRevolveArgs

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathRevolveArgs {
    pub path_n: u32,
    pub start_angle: f64,
    pub end_angle: f64,
    pub radius: f64,
}

impl Default for PathRevolveArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            start_angle: 0.0,
            end_angle: 90.0,
            radius: 64.0,
        }
    }
}

impl PathRevolveArgs {
    fn path(&self) -> extrude::path::Revolve {
        extrude::path::Revolve::new(self.radius, self.start_angle, self.end_angle)
    }
}

// -------------------------------------------------------- PathSinusoidArgs

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathSinusoidArgs {
    pub path_n: u32,
    pub amplitude: f64,
    pub period: f64,
    pub phase: f64,
    pub start: f64,
    pub end: f64,
}

impl Default for PathSinusoidArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            amplitude: 32.0,
            period: 128.0,
            phase: 0.0,
            start: 0.0,
            end: 128.0,
        }
    }
}

impl PathSinusoidArgs {
    fn path(&self) -> PathResult<extrude::path::Sinusoid> {
        extrude::path::Sinusoid::new(
            self.amplitude,
            self.period,
            self.phase,
            self.start,
            self.end,
        )
    }
}

// -------------------------------------------------------- PathBezierArgs

#[derive(Debug, Clone, PartialEq)]
pub struct PathBezierArgs {
    pub path_n: u32,
    pub points: Vec<glam::DVec2>,
}

impl Default for PathBezierArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            points: vec![
                glam::DVec2::new(0.0, 0.0),
                glam::DVec2::new(32.0, 0.0),
                glam::DVec2::new(64.0, 64.0),
            ],
        }
    }
}

impl PathBezierArgs {
    fn path(&self) -> PathResult<extrude::path::Bezier> {
        Ok(extrude::path::Bezier::new(self.points.clone())?)
    }
}

// -------------------------------------------------------- PathCatenaryArgs

#[derive(Debug, Clone, PartialEq)]
pub struct PathCatenaryArgs {
    pub path_n: u32,
    pub span: f64,
    pub height: f64,
    pub s: f64,
}

impl Default for PathCatenaryArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            span: 128.0,
            height: 0.0,
            s: 132.0,
        }
    }
}

impl PathCatenaryArgs {
    fn path(&self) -> PathResult<extrude::path::Catenary> {
        Ok(extrude::path::Catenary::new(
            self.span,
            self.height,
            self.s,
        )?)
    }
}

// -------------------------------------------------------- PathSerpentineArgs

#[derive(Debug, Clone, PartialEq)]
pub struct PathSerpentineArgs {
    pub path_n: u32,
    pub x: f64,
    pub z: f64,
}

impl Default for PathSerpentineArgs {
    fn default() -> Self {
        Self {
            path_n: 12,
            x: 64.0,
            z: 32.0,
        }
    }
}

impl PathSerpentineArgs {
    fn path(&self) -> PathResult<extrude::path::Serpentine> {
        Ok(extrude::path::Serpentine::new(self.x, self.z)?)
    }
}
