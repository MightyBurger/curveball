use crate::CustomUV;
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

use bevy_egui::{egui, EguiContexts, EguiPlugin};

use curveball_lib::curve::{Bank, Catenary, Curve, CurveResult, Rayto, Serpentine};
use curveball_lib::map::{Brush, QEntity, QMap, Side, SideGeom, SimpleWorldspawn};
use glam::DVec3;

#[derive(Resource, Debug, Clone, PartialEq, PartialOrd)]
pub enum CurveSelect {
    Rayto(RaytoArgs),
    Bank(BankArgs),
    // catenary
    // serpentine
    // easy-serp
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
            n: 8,
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
    pub ri0: f64,
    pub ro0: f64,
    pub ri1: f64,
    pub ro1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub h: f64,
    pub t: f64,
    pub fill: bool,
}

impl Default for BankArgs {
    fn default() -> Self {
        Self {
            n: 8,
            ri0: 16.0,
            ro0: 32.0,
            ri1: 16.0,
            ro1: 32.0,
            theta0: 0.0,
            theta1: 90.0,
            h: 64.0,
            t: 32.0,
            fill: false,
        }
    }
}

impl CurveSelect {
    fn mesh(&self) -> CurveResult<Mesh> {
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
                ri0: args.ri0,
                ro0: args.ro0,
                ri1: args.ri1,
                ro1: args.ro1,
                theta0: args.theta0,
                theta1: args.theta1,
                h: args.h,
                t: args.t,
                fill: args.fill,
            }
            .bake()?,
        };
        Ok(brushes_to_mesh(&brushes))
    }
}

pub fn update_mesh(
    mut commands: Commands,
    curve_select: Res<CurveSelect>,
    mesh_query_1: Query<&Mesh3d, With<CustomUV>>,
    mesh_query_2: Query<Entity, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if curve_select.is_changed() {
        info!("mesh changed");
        match curve_select.mesh() {
            Ok(mesh) => {
                info!("mesh made");

                for mesh_handle in mesh_query_1.iter() {
                    meshes.remove(mesh_handle);
                }
                for mesh_entity in mesh_query_2.iter() {
                    commands.entity(mesh_entity).despawn();
                }

                // Create and save a handle to the mesh.
                let cube_mesh_handle: Handle<Mesh> = meshes.add(mesh);

                // Render the mesh with the custom texture, and add the marker.
                commands.spawn((
                    Mesh3d(cube_mesh_handle),
                    MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
                    CustomUV,
                ));
            }
            Err(_) => {
                info!("ERROR MAKING MESH!!");
            }
        }
    }
}

fn brushes_to_mesh<'a>(brush: impl IntoIterator<Item = &'a Brush>) -> Mesh {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();

    for [p0, p1, p2] in brush.into_iter().flat_map(|brush| brush.triangles()).map(
        |Side {
             geom: SideGeom(triangle),
             mtrl: _,
         }| { triangle },
    ) {
        // The order here is intentional - it must be counter-clockwise looking at the face.
        vertices.push([p0.x as f32, p0.y as f32, p0.z as f32]);
        vertices.push([p2.x as f32, p2.y as f32, p2.z as f32]);
        vertices.push([p1.x as f32, p1.y as f32, p1.z as f32]);

        let normal = ((p0 - p1).cross(p2 - p1)).normalize();
        let normal = [normal.x as f32, normal.y as f32, normal.z as f32];

        normals.push(normal);
        normals.push(normal);
        normals.push(normal);

        let color = [
            (1.0 - normal[0]) / 2.0,
            (1.0 - normal[1]) / 2.0,
            (1.0 - normal[2]) / 2.0,
            1.0,
        ];
        colors.push(color);
        colors.push(color);
        colors.push(color);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}
