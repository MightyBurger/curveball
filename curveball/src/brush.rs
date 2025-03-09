// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{CustomUV, MeshGen, MeshGenError, curveargs::CurveSelect};

use bevy::{
    color::palettes::tailwind,
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

use curveball_lib::map::geometry::{Brush, Side, SideGeom};
use glam::DVec3;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
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

#[allow(clippy::too_many_arguments)]
pub fn update_mesh(
    mut commands: Commands,
    curve_select: Res<CurveSelect>,
    mut previous_curve_select: Local<Option<CurveSelect>>,
    mut previous_meshdisp: Local<Option<MeshDisplaySettings>>,
    mesh_query_1: Query<&Mesh3d, With<CustomUV>>,
    mesh_query_2: Query<Entity, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshgen: ResMut<MeshGen>,
    meshdisp: Res<MeshDisplaySettings>,
) {
    let mut update_necessary = false;

    if curve_select.is_changed() {
        if *previous_curve_select != Some(curve_select.clone()) {
            update_necessary = true;
        }
    }

    if meshdisp.is_changed() {
        if *previous_meshdisp != Some(meshdisp.clone()) {
            update_necessary = true;
        }
    }

    if !update_necessary {
        return;
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

    *previous_curve_select = Some(curve_select.clone());
    *previous_meshdisp = Some(meshdisp.clone());
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
