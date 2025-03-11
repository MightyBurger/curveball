// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Includes example code from Bevy

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    picking::pointer::PointerInteraction,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::{f32::consts::*, fmt};

use crate::gui::egui_blocking_plugin::EguiBlockInputState;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
    }
}

pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub settings: CameraControllerSettings,
    pub walk_speed: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            settings: CameraControllerSettings::default(),
            walk_speed: 250.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub struct CameraControllerSettings {
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_navigate: MouseButton,
    pub mouse_key_pan: MouseButton,
    pub mouse_key_orbit: MouseButton,
    pub keyboard_key_toggle_cursor_grab: KeyCode,
    pub keyboard_key_escape_cursor_grab: KeyCode,
    pub keyboard_key_orbit_modifier: KeyCode,
    pub scroll_factor: f32,
    pub friction: f32,
    pub min_walkspeed: f32,
    pub max_walkspeed: f32,
    pub run_factor: f32,
    pub zoom_speed: f32,
}

impl Default for CameraControllerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyQ,
            key_down: KeyCode::KeyX,
            key_run: KeyCode::ShiftLeft,
            mouse_key_navigate: MouseButton::Right,
            mouse_key_pan: MouseButton::Middle,
            mouse_key_orbit: MouseButton::Left,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyC,
            keyboard_key_escape_cursor_grab: KeyCode::Escape,
            keyboard_key_orbit_modifier: KeyCode::AltLeft,
            scroll_factor: 0.2,
            friction: 0.5,
            min_walkspeed: 12.5,
            max_walkspeed: 5000.0,
            run_factor: 3.0,
            zoom_speed: 16.0,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Freecam Controls:
    Mouse\t- Move camera orientation
    Scroll\t- Adjust movement speed
    {:?}\t- Hold to grab cursor
    {:?}\t- Toggle cursor grab
    {:?} & {:?}\t- Fly forward & backwards
    {:?} & {:?}\t- Fly sideways left & right
    {:?} & {:?}\t- Fly up & down
    {:?}\t- Fly faster while held",
            self.settings.mouse_key_navigate,
            self.settings.keyboard_key_toggle_cursor_grab,
            self.settings.key_forward,
            self.settings.key_back,
            self.settings.key_left,
            self.settings.key_right,
            self.settings.key_up,
            self.settings.key_down,
            self.settings.key_run,
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum CursorGrabState {
    #[default]
    NoGrab,
    OrbitInFront(Vec3),
    OrbitPoint(Vec3),
    Pan,
    Navigate,
}

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut cursor_grab_state: Local<CursorGrabState>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    egui_block_input_state: Res<EguiBlockInputState>,
    pointers: Query<&PointerInteraction>,
    mut gizmos: Gizmos<DefaultGizmoConfigGroup>,
) {
    let dt = time.delta_secs();

    let Ok((mut transform, mut controller)) = query.get_single_mut() else {
        return;
    };

    if !controller.initialized {
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        controller.initialized = true;
        info!("{}", *controller);
    }
    if !controller.enabled {
        return;
    }

    let mut window = windows.single_mut();

    // Determine grab state
    let old_grab_state = *cursor_grab_state;
    if !egui_block_input_state.wants_keyboard_input {
        if key_input.just_pressed(controller.settings.keyboard_key_toggle_cursor_grab) {
            *cursor_grab_state = match *cursor_grab_state {
                CursorGrabState::NoGrab => CursorGrabState::Navigate,
                CursorGrabState::OrbitInFront(_)
                | CursorGrabState::OrbitPoint(_)
                | CursorGrabState::Pan
                | CursorGrabState::Navigate => CursorGrabState::NoGrab,
            }
        };
        if key_input.just_pressed(controller.settings.keyboard_key_escape_cursor_grab) {
            *cursor_grab_state = CursorGrabState::NoGrab;
        }
    }
    if !egui_block_input_state.wants_pointer_input {
        if mouse_button_input.just_pressed(controller.settings.mouse_key_orbit) {
            let point = transform.translation + transform.forward() * 512.0;
            *cursor_grab_state = CursorGrabState::OrbitInFront(point);
        }
        if mouse_button_input.just_pressed(controller.settings.mouse_key_pan) {
            *cursor_grab_state = CursorGrabState::Pan;
        }
        if mouse_button_input.just_pressed(controller.settings.mouse_key_navigate) {
            // If we alt-right-click and are hovering over a valid point, go into Orbit Point.
            // If not hovering over a valid point, don't do anything.
            // Otherwise go into Pan.
            if !egui_block_input_state.wants_keyboard_input
                && key_input.pressed(controller.settings.keyboard_key_orbit_modifier)
            {
                if let Some((point, _normal)) = pointers
                    .iter()
                    .filter_map(|interaction| interaction.get_nearest_hit())
                    .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
                    .next()
                {
                    *cursor_grab_state = CursorGrabState::OrbitPoint(point);
                } else {
                    // Hey, you missed!
                    *cursor_grab_state = CursorGrabState::NoGrab;
                }
            } else {
                *cursor_grab_state = CursorGrabState::Navigate;
            }
        }
    }
    if mouse_button_input.just_released(controller.settings.mouse_key_orbit) {
        *cursor_grab_state = CursorGrabState::NoGrab;
    }
    if mouse_button_input.just_released(controller.settings.mouse_key_pan) {
        *cursor_grab_state = CursorGrabState::NoGrab;
    }
    if mouse_button_input.just_released(controller.settings.mouse_key_navigate) {
        *cursor_grab_state = CursorGrabState::NoGrab;
    }
    let cursor_grab_change = if *cursor_grab_state == old_grab_state {
        false
    } else {
        true
    };

    if let CursorGrabState::OrbitPoint(point) = *cursor_grab_state {
        let dist = (point - transform.translation).length();
        gizmos
            .sphere(point, dist / 128.0, bevy::color::palettes::css::WHITE)
            .resolution(64);
        gizmos.cross(
            Isometry3d::from_translation(point),
            dist / 64.0,
            bevy::color::palettes::css::WHITE,
        )
    };

    // Keyboard navigation
    let mut axis_input = Vec3::ZERO;
    if !egui_block_input_state.wants_keyboard_input {
        if key_input.pressed(controller.settings.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(controller.settings.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(controller.settings.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(controller.settings.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(controller.settings.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(controller.settings.key_down) {
            axis_input.y -= 1.0;
        }
    }

    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.settings.key_run) {
            controller.walk_speed * controller.settings.run_factor
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
    } else {
        let friction = controller.settings.friction;
        controller.velocity *= 1.0 - friction;
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    let forward = *transform.forward();
    let right = *transform.right();
    transform.translation += controller.velocity.x * dt * right
        + controller.velocity.y * dt * Vec3::Y
        + controller.velocity.z * dt * forward;

    // Handle scroll
    if !egui_block_input_state.wants_pointer_input {
        let scroll = match accumulated_mouse_scroll.unit {
            MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
            MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
        };
        match *cursor_grab_state {
            CursorGrabState::NoGrab
            | CursorGrabState::OrbitInFront(_)
            | CursorGrabState::OrbitPoint(_)
            | CursorGrabState::Pan => {
                let forward = *transform.forward();
                transform.translation += scroll * forward * controller.settings.zoom_speed;
            }
            CursorGrabState::Navigate => {
                controller.walk_speed +=
                    scroll * controller.settings.scroll_factor * controller.walk_speed;
                controller.walk_speed = controller.walk_speed.clamp(
                    controller.settings.min_walkspeed,
                    controller.settings.max_walkspeed,
                );
            }
        }
    }

    // Handle cursor grab
    if cursor_grab_change {
        match *cursor_grab_state {
            CursorGrabState::OrbitInFront(_)
            | CursorGrabState::OrbitPoint(_)
            | CursorGrabState::Pan
            | CursorGrabState::Navigate => {
                if window.focused {
                    // window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    // window.cursor_options.visible = false;
                }
            }
            CursorGrabState::NoGrab => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }

    // Handle mouse input
    match *cursor_grab_state {
        CursorGrabState::Navigate => {
            if accumulated_mouse_motion.delta != Vec2::ZERO {
                // Regular mechanics
                // Apply look update
                controller.pitch = (controller.pitch
                    - accumulated_mouse_motion.delta.y
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity)
                    .clamp(-PI / 2., PI / 2.);
                controller.yaw -= accumulated_mouse_motion.delta.x
                    * RADIANS_PER_DOT
                    * controller.settings.sensitivity;
                transform.rotation =
                    Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
            }
        }
        CursorGrabState::Pan => {
            if accumulated_mouse_motion.delta != Vec2::ZERO {
                let up = *transform.up();
                let right = *transform.right();
                transform.translation += -accumulated_mouse_motion.delta.x
                    * right
                    * controller.settings.sensitivity
                    / 4.0
                    + accumulated_mouse_motion.delta.y * up * controller.settings.sensitivity / 4.0;
            }
        }
        CursorGrabState::OrbitInFront(point) => {
            // if accumulated_mouse_motion.delta != Vec2::ZERO {
            //     // Current position in spherical coordinates
            //     let [r, mut theta, mut phi]: [f32; 3] =
            //         cartesian_to_spherical(transform.translation).into();
            //     phi = phi
            //         + accumulated_mouse_motion.delta.x
            //             * RADIANS_PER_DOT
            //             * controller.settings.sensitivity;
            //     theta = (theta
            //         - accumulated_mouse_motion.delta.y
            //             * RADIANS_PER_DOT
            //             * controller.settings.sensitivity)
            //         .clamp(0.000001, PI - 0.000001);
            //     transform.translation = spherical_to_cartesian(Vec3::new(r, theta, phi));
            // }
            // *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
            // let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            // controller.yaw = yaw;
            // controller.pitch = pitch;
            if accumulated_mouse_motion.delta != Vec2::ZERO {
                // Current position in spherical coordinates
                let [r, mut theta, mut phi]: [f32; 3] =
                    cartesian_to_spherical(transform.translation - point).into();
                phi = phi
                    + accumulated_mouse_motion.delta.x
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity;
                theta = (theta
                    - accumulated_mouse_motion.delta.y
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity)
                    .clamp(0.000001, PI - 0.000001);
                transform.translation = spherical_to_cartesian(Vec3::new(r, theta, phi)) + point;
            }
            *transform = transform.looking_at(point, Vec3::Y);
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
        }
        CursorGrabState::OrbitPoint(point) => {
            if accumulated_mouse_motion.delta != Vec2::ZERO {
                // Current position in spherical coordinates
                let [r, mut theta, mut phi]: [f32; 3] =
                    cartesian_to_spherical(transform.translation - point).into();
                phi = phi
                    + accumulated_mouse_motion.delta.x
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity;
                theta = (theta
                    - accumulated_mouse_motion.delta.y
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity)
                    .clamp(0.000001, PI - 0.000001);
                transform.translation = spherical_to_cartesian(Vec3::new(r, theta, phi)) + point;
            }
            *transform = transform.looking_at(point, Vec3::Y);
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
        }
        CursorGrabState::NoGrab => {}
    }
}

fn cartesian_to_spherical(cart: Vec3) -> Vec3 {
    let r = cart.length();
    let r_flat = cart.xz();
    let theta = f32::atan2(r_flat.length(), cart.y); // radial
    let phi = f32::atan2(cart.z, cart.x); // azimuth
    Vec3::new(r, theta, phi)
}

fn spherical_to_cartesian(spherical: Vec3) -> Vec3 {
    let [r, theta, phi]: [f32; 3] = spherical.into();
    let x = r * theta.sin() * phi.cos();
    let z = r * theta.sin() * phi.sin();
    let y = r * theta.cos();
    Vec3::new(x, y, z)
}
