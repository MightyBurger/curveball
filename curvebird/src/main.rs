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
        .add_plugins(CameraControllerPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .add_systems(Update, input_handler)
        .add_systems(Update, ui)
        .add_systems(Update, update_mesh)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<CurveSelect>()
        .run();
}

fn draw_grid(mut gizmos: Gizmos) {
    const COUNT: u32 = 200;
    const SPACING: f32 = 16.0;
    gizmos.grid(
        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
        UVec2::splat(COUNT),
        Vec2::new(SPACING, SPACING),
        LinearRgba::gray(0.65),
    );
}

fn setup(mut commands: Commands) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(-128.0, 128.0, -128.0).looking_at(Vec3::ZERO, Vec3::Y);

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

// System to receive input from the user,
// check out examples/input/ for more examples about user input.
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mesh_query: Query<&Mesh3d, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // let mesh_handle = mesh_query.get_single().expect("Query not successful");
        // let mesh = meshes.get_mut(mesh_handle).unwrap();
        // toggle_texture(mesh);
    }
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}
