// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{CustomUV, MeshGen};

use bevy::{
    color::palettes::tailwind,
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

use curveball::curve::{
    serpentine::SerpentineOffsetMode, Bank, Catenary, Curve, CurveResult, Rayto, Serpentine,
};
use curveball::map::{Brush, Side, SideGeom};
use glam::DVec3;

#[derive(Resource, Debug, Clone, PartialEq, PartialOrd)]
pub enum CurveSelect {
    Rayto(RaytoArgs),
    Bank(BankArgs),
    Catenary(CatenaryArgs),
    Serpentine(SerpentineArgs),
}

impl Default for CurveSelect {
    fn default() -> Self {
        Self::Bank(BankArgs::default())
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
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub s: f64,
    pub w: f64,
    pub t: f64,
    pub initial_guess: Option<f64>,
}

impl Default for CatenaryArgs {
    fn default() -> Self {
        Self {
            n: 24,
            x0: 0.0,
            z0: 0.0,
            x1: 128.0,
            z1: 0.0,
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

impl CurveSelect {
    fn mesh(&self) -> CurveResult<Vec<Brush>> {
        let brushes = match self {
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
                x0: args.x0,
                z0: args.z0,
                x1: args.x1,
                z1: args.z1,
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
        };
        Ok(brushes)
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
) {
    // Check if mesh actually needs to update
    if !curve_select.is_changed() {
        return;
    }

    if let Some(prev) = previous.clone() {
        if prev == *curve_select {
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
    match curve_select.mesh() {
        Ok(mesh) => {
            info!("Updated mesh");

            // Choose a color!

            let base_color = match *curve_select {
                CurveSelect::Rayto { .. } => tailwind::ROSE_400,
                CurveSelect::Bank { .. } => tailwind::ORANGE_400,
                CurveSelect::Catenary { .. } => tailwind::TEAL_400,
                CurveSelect::Serpentine { .. } => tailwind::LIME_400,
            };

            let base_color: LinearRgba = base_color.into();
            let base_color = [
                base_color.red,
                base_color.green,
                base_color.blue,
                base_color.alpha,
            ];

            // Create and save a handle to the mesh.
            let cube_mesh_handle: Handle<Mesh> = meshes.add(brushes_to_mesh(&mesh, base_color));
            *meshgen = MeshGen(Some(Ok(mesh)));

            // Render the mesh with the custom texture, and add the marker.
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

    *previous = Some(curve_select.clone());
}

fn brushes_to_mesh<'a>(brushes: impl IntoIterator<Item = &'a Brush>, base_color: [f32; 4]) -> Mesh {
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

            vertices.push([p0.x as f32, p0.y as f32, p0.z as f32]);
            vertices.push([p2.x as f32, p2.y as f32, p2.z as f32]);
            vertices.push([p1.x as f32, p1.y as f32, p1.z as f32]);

            let normal = ((p0 - p1).cross(p2 - p1)).normalize();
            let normal = [normal.x as f32, normal.y as f32, normal.z as f32];

            normals.push(normal);
            normals.push(normal);
            normals.push(normal);

            let scale = if i % 2 == 0 { 1.0 } else { 0.8 };

            let color = [
                scale * base_color[0],
                scale * base_color[1],
                scale * base_color[2],
                base_color[3],
            ];
            colors.push(color);
            colors.push(color);
            colors.push(color);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}
