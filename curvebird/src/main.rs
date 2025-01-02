// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::time::Duration;

use bevy::{prelude::*, winit::WinitSettings};

use bevy_egui::EguiPlugin;

mod brush;
mod gui;
use brush::{update_mesh, CurveSelect};
use gui::{ui, OccupiedScreenSpace};
mod camera_controller;
use camera_controller::{CameraController, CameraControllerPlugin};

use curveball::curve::CurveResult;
use curveball::map::Brush;

#[derive(Component)]
struct CustomUV;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "curvebird".to_string(),
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 200.0,
        })
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10)),
        })
        .init_gizmo_group::<GridMinor>()
        .init_gizmo_group::<GridMajor>()
        .init_gizmo_group::<Axis>()
        .add_plugins(CameraControllerPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gizmos)
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
struct GridMinor {}
#[derive(Default, Reflect, GizmoConfigGroup)]
struct GridMajor {}
#[derive(Default, Reflect, GizmoConfigGroup)]
struct Axis {}

fn draw_gizmos(
    mut grid_minor: Gizmos<GridMinor>,
    mut grid_major: Gizmos<GridMajor>,
    mut axis: Gizmos<Axis>,
) {
    const COUNT: u32 = 64;
    const SPACING: f32 = 8.0;
    const AXIS_LEN: f32 = COUNT as f32 * SPACING;
    let grid_color = LinearRgba::gray(0.65);

    for i in (-(COUNT as i32)..=-1).chain(1..=(COUNT as i32)) {
        if i % 4 == 0 {
            grid_major.line(
                Vec3::new(SPACING * i as f32, 0.0, AXIS_LEN),
                Vec3::new(SPACING * i as f32, 0.0, -AXIS_LEN),
                grid_color,
            );
            grid_major.line(
                Vec3::new(AXIS_LEN, 0.0, SPACING * i as f32),
                Vec3::new(-AXIS_LEN, 0.0, SPACING * i as f32),
                grid_color,
            );
        } else {
            grid_minor.line(
                Vec3::new(SPACING * i as f32, 0.0, AXIS_LEN),
                Vec3::new(SPACING * i as f32, 0.0, -AXIS_LEN),
                grid_color,
            );
            grid_minor.line(
                Vec3::new(AXIS_LEN, 0.0, SPACING * i as f32),
                Vec3::new(-AXIS_LEN, 0.0, SPACING * i as f32),
                grid_color,
            );
        }
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
    let (grid_config, _) = config_store.config_mut::<GridMinor>();
    grid_config.line_width = 0.05;
    let (grid_config, _) = config_store.config_mut::<GridMajor>();
    grid_config.line_width = 0.2;
    let (grid_config, _) = config_store.config_mut::<Axis>();
    grid_config.line_width = 0.4;

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform =
        Transform::from_xyz(256.0, 256.0, -384.0).looking_at(Vec3::ZERO, Vec3::Y);

    let cc = CameraController::default();

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_transform, cc));

    // Key light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            ..default()
        },
        Transform::from_xyz(512.0, 1024.0, 512.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Fill light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: light_consts::lux::OVERCAST_DAY,
            ..default()
        },
        Transform::from_xyz(-256.0, 256.0, -512.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
