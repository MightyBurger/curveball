// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{CustomUV, MeshGen, MeshGenError};

use bevy::{
    color::palettes::tailwind,
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

use curveball_lib::curve::{
    bank::Bank, catenary::Catenary, curve_classic::CurveClassic, curve_slope::CurveSlope, extrude,
    rayto::Rayto, serpentine::Serpentine, serpentine::SerpentineOffsetMode, Curve, CurveResult,
};
use curveball_lib::map::geometry::{Brush, Side, SideGeom};
use glam::DVec3;

#[derive(Resource, Debug, Clone)]
pub struct MeshDisplaySettings {
    pub alternating_colors: bool,
}

impl Default for MeshDisplaySettings {
    fn default() -> Self {
        Self {
            alternating_colors: true,
        }
    }
}

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
    pub profile_circle_args: ProfileCircleArgs,
    pub profile_rectangle_args: ProfileRectangleArgs,
    pub path: PathSelect,
    pub path_revolve_args: PathRevolveArgs,
    pub path_n: u32,
    pub path_start: f64,
    pub path_end: f64,
    pub profile_orientation: extrude::ProfileOrientation,
}

impl Default for ExtrusionArgs {
    fn default() -> Self {
        Self {
            profile: ProfileSelect::default(),
            profile_circle_args: ProfileCircleArgs::default(),
            profile_rectangle_args: ProfileRectangleArgs::default(),
            path: PathSelect::default(),
            path_revolve_args: PathRevolveArgs::default(),
            path_n: 12,
            path_start: 0.0,
            path_end: 90.0,
            profile_orientation: extrude::ProfileOrientation::FollowPath,
        }
    }
}

impl Default for PathSelect {
    fn default() -> Self {
        Self::Revolve
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ProfileSelect {
    Circle,
    Rectangle,
}

impl Default for ProfileSelect {
    fn default() -> Self {
        Self::Circle
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
pub enum PathSelect {
    Revolve,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathRevolveArgs {
    pub radius: f64,
}

impl Default for PathRevolveArgs {
    fn default() -> Self {
        Self { radius: 32.0 }
    }
}

impl CurveSelect {
    fn brushes(&self) -> CurveResult<Vec<Brush>> {
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
        let profile_fn: Box<dyn Fn(f64) -> Vec<DVec3>> = match args.profile {
            ProfileSelect::Circle => Box::new(extrude::profile::circle(
                args.profile_circle_args.n,
                args.profile_circle_args.radius,
            )?),
            ProfileSelect::Rectangle => Box::new(extrude::profile::rectangle(
                args.profile_rectangle_args.width,
                args.profile_rectangle_args.height,
                args.profile_rectangle_args.anchor,
            )?),
        };

        let (path_fn, frenet_fn): (
            Box<dyn Fn(f64) -> DVec3>,
            Box<dyn Fn(f64) -> extrude::FrenetFrame>,
        ) = match args.path {
            PathSelect::Revolve => {
                let (path_fn, frenet_fn) = extrude::path::revolve(args.path_revolve_args.radius)?;
                (Box::new(path_fn), Box::new(frenet_fn))
            }
        };

        extrude::extrude(
            args.path_n,
            profile_fn,
            path_fn,
            frenet_fn,
            args.path_start,
            args.path_end,
            args.profile_orientation,
        )
        // extrude::extrude(
        //     4,
        //     profile_fn,
        //     |t| DVec3::from([t, 0.0, 0.01 * t * t]),
        //     |t| extrude::FrenetFrame {
        //         tangent: DVec3::new(1.0, 0.0, 0.0),
        //         normal: DVec3::new(0.0, 1.0, 0.01 * t),
        //         binormal: DVec3::new(0.0, 0.01 * t, 1.0),
        //     },
        //     -64.0,
        //     64.0,
        //     args.profile_orientation,
        // )
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_mesh(
    mut commands: Commands,
    curve_select: Res<CurveSelect>,
    mut previous: Local<Option<CurveSelect>>,
    mesh_query_1: Query<&Mesh3d, With<CustomUV>>,
    mesh_query_2: Query<Entity, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshgen: ResMut<MeshGen>,
    meshdisp: Res<MeshDisplaySettings>,
) {
    // Check if mesh actually needs to update
    if !(curve_select.is_changed() || meshdisp.is_changed()) {
        return;
    }

    if let Some(prev) = previous.clone() {
        if prev == *curve_select && !meshdisp.is_changed() {
            return;
        }
    }

    // Remove the old mesh
    for mesh_handle in mesh_query_1.iter() {
        meshes.remove(mesh_handle);
    }
    for mesh_entity in mesh_query_2.iter() {
        commands.entity(mesh_entity).despawn();
    }

    // Create the new mesh
    match curve_select.brushes() {
        Ok(brushes) => {
            // Choose a color!

            let base_color = match *curve_select {
                CurveSelect::CurveClassic { .. } => tailwind::STONE_400,
                CurveSelect::CurveSlope { .. } => tailwind::SLATE_400,
                CurveSelect::Rayto { .. } => tailwind::RED_400,
                CurveSelect::Bank { .. } => tailwind::ORANGE_400,
                CurveSelect::Catenary { .. } => tailwind::TEAL_400,
                CurveSelect::Serpentine { .. } => tailwind::LIME_400,
                CurveSelect::Extrusion { .. } => tailwind::LIME_400,
            };

            let base_color: LinearRgba = base_color.into();
            let base_color = [
                base_color.red,
                base_color.green,
                base_color.blue,
                base_color.alpha,
            ];

            let scale = 0.8;
            let other_color = if !meshdisp.alternating_colors {
                base_color
            } else {
                [
                    base_color[0] * scale,
                    base_color[1] * scale,
                    base_color[2] * scale,
                    base_color[3],
                ]
            };

            match brushes_to_mesh(&brushes, base_color, other_color) {
                Ok(mesh) => {
                    let cube_mesh_handle: Handle<Mesh> = meshes.add(mesh);
                    *meshgen = MeshGen(Some(Ok(brushes)));

                    commands.spawn((
                        Mesh3d(cube_mesh_handle),
                        MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
                        CustomUV,
                    ));
                }
                Err(e) => {
                    warn!("{}", &e);
                    *meshgen = MeshGen(Some(Err(e)));
                }
            }

            // Create and save a handle to the mesh.
        }
        Err(e) => {
            warn!("{}", &e);
            let e = MeshGenError::from(e);
            *meshgen = MeshGen(Some(Err(e)));
        }
    }

    *previous = Some(curve_select.clone());
}

fn brushes_to_mesh<'a>(
    brushes: impl IntoIterator<Item = &'a Brush>,
    color1: [f32; 4],
    color2: [f32; 4],
) -> Result<Mesh, MeshGenError> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();

    for (i, brush) in brushes.into_iter().enumerate() {
        for [p0, p1, p2] in brush.triangles().map(
            |Side {
                 geom: SideGeom(triangle),
                 mtrl: _,
             }| { triangle },
        ) {
            // Swap the Y and Zs.
            let p0 = DVec3 {
                x: -p0.x,
                y: p0.z,
                z: p0.y,
            };
            let p1 = DVec3 {
                x: -p1.x,
                y: p1.z,
                z: p1.y,
            };
            let p2 = DVec3 {
                x: -p2.x,
                y: p2.z,
                z: p2.y,
            };

            let Some(normal) = ((p0 - p1).cross(p2 - p1)).try_normalize() else {
                warn!("Not displaying a face: {}", MeshGenError::NormalizeError);
                continue;
            };
            let normal = [normal.x as f32, normal.y as f32, normal.z as f32];

            vertices.push([p0.x as f32, p0.y as f32, p0.z as f32]);
            vertices.push([p2.x as f32, p2.y as f32, p2.z as f32]);
            vertices.push([p1.x as f32, p1.y as f32, p1.z as f32]);

            normals.push(normal);
            normals.push(normal);
            normals.push(normal);

            let color = if i % 2 == 0 { color1 } else { color2 };

            colors.push(color);
            colors.push(color);
            colors.push(color);
        }
    }

    Ok(Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors))
}
