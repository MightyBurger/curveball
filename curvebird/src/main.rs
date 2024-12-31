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

use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod brush;
use brush::update_mesh;
mod camera_controller;
use camera_controller::{CameraController, CameraControllerPlugin};

use curveball_lib::curve::{Bank, Catenary, Curve, CurveResult, Rayto, Serpentine};
use curveball_lib::map::{Brush, QEntity, QMap, Side, SideGeom, SimpleWorldspawn};
use glam::DVec3;

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for
// filtering entities in queries with `With`, they're usually not queried directly since they don't
// contain information within them.
#[derive(Component)]
struct CustomUV;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .add_systems(Update, ui_example)
        .add_systems(Update, update_mesh)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<CurveSelect>()
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // // Create and save a handle to the mesh.
    // let cube_mesh_handle: Handle<Mesh> = meshes.add(create_test_mesh());
    //
    // // Render the mesh with the custom texture, and add the marker.
    // commands.spawn((
    //     Mesh3d(cube_mesh_handle),
    //     MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
    //     CustomUV,
    // ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(128.0, 128.0, 128.0).looking_at(Vec3::ZERO, Vec3::Y);

    let cc = CameraController { ..default() };

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform, cc));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));
}

#[derive(Default, Debug, Resource)]
struct OccupiedScreenSpace {
    right: f32,
}

fn ui_example(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut curve_select: ResMut<CurveSelect>,
) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.right = egui::SidePanel::right("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            if ui.button("Here is a button").clicked() {
                info!("clicked");
                let next_curve = match *curve_select {
                    CurveSelect::Rayto(_) => CurveSelect::Bank(BankArgs::default()),
                    CurveSelect::Bank(_) => CurveSelect::Rayto(RaytoArgs::default()),
                };
                *curve_select = next_curve;
            };
            ui.label(format!("Selected curve is {:?}", *curve_select));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
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
