//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.

#![allow(unused_imports)]

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

use crate::brush::{BankArgs, CurveSelect, RaytoArgs};
use crate::gui::ui;
use crate::gui::OccupiedScreenSpace;

use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod gui;

mod brush;
use brush::update_mesh;
mod camera_controller;
use camera_controller::{CameraController, CameraControllerPlugin};

use curveball_lib::curve::{Bank, Catenary, Curve, CurveResult, Rayto, Serpentine};
use curveball_lib::map::{Brush, QEntity, QMap, Side, SideGeom, SimpleWorldspawn};
use glam::DVec3;

#[derive(Component)]
struct CustomUV;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "curvebird".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 2000.0,
        })
        .init_gizmo_group::<Grid>()
        .init_gizmo_group::<Axis>()
        .add_plugins(CameraControllerPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmos)
        // .add_systems(Update, input_handler)
        .add_systems(Update, ui)
        .add_systems(Update, update_mesh)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<CurveSelect>()
        .init_resource::<MeshGen>()
        .run();
}

#[derive(Resource, Default)]
struct MeshGen(Option<CurveResult<Vec<Brush>>>);

#[derive(Default, Reflect, GizmoConfigGroup)]
struct Grid {}
#[derive(Default, Reflect, GizmoConfigGroup)]
struct Axis {}

fn draw_gizmos(mut grid: Gizmos<Grid>, mut axis: Gizmos<Axis>) {
    const COUNT: u32 = 64;
    const SPACING: f32 = 16.0;
    const AXIS_LEN: f32 = COUNT as f32 * SPACING;
    let grid_color = LinearRgba::gray(0.65);

    for i in (-(COUNT as i32)..=-1).chain(1..=(COUNT as i32)) {
        grid.line(
            Vec3::new(SPACING * i as f32, 0.0, AXIS_LEN),
            Vec3::new(SPACING * i as f32, 0.0, -AXIS_LEN),
            grid_color,
        );
        grid.line(
            Vec3::new(AXIS_LEN, 0.0, SPACING * i as f32),
            Vec3::new(-AXIS_LEN, 0.0, SPACING * i as f32),
            grid_color,
        );
    }

    // Trenchbroom axis, not Bevy axis.

    const X_POINT_1: Vec3 = Vec3::new(AXIS_LEN, 0.0, 0.0);
    const X_POINT_2: Vec3 = Vec3::new(-AXIS_LEN, 0.0, 0.0);
    const X_COLOR: LinearRgba = LinearRgba {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };
    axis.line(X_POINT_1, X_POINT_2, X_COLOR);

    const Y_POINT_1: Vec3 = Vec3::new(0.0, 0.0, AXIS_LEN);
    const Y_POINT_2: Vec3 = Vec3::new(0.0, 0.0, -AXIS_LEN);
    const Y_COLOR: LinearRgba = LinearRgba {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
        alpha: 1.0,
    };
    axis.line(Y_POINT_1, Y_POINT_2, Y_COLOR);

    const Z_POINT_1: Vec3 = Vec3::new(0.0, AXIS_LEN, 0.0);
    const Z_POINT_2: Vec3 = Vec3::new(0.0, -AXIS_LEN, 0.0);
    const Z_COLOR: LinearRgba = LinearRgba {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    };
    axis.line(Z_POINT_1, Z_POINT_2, Z_COLOR);
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    // Configure gizmos
    let (grid_config, _) = config_store.config_mut::<Grid>();
    grid_config.line_width = 0.2;
    let (grid_config, _) = config_store.config_mut::<Axis>();
    grid_config.line_width = 0.8;

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(256.0, 256.0, -384.0).looking_at(Vec3::ZERO, Vec3::Y);

    let cc = CameraController::default();

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform, cc));

    // Light up the scene.
    commands.spawn((
        PointLight {
            intensity: 100_000_000.0,
            range: 400.0,
            ..default()
        },
        camera_and_light_transform,
    ));
}
