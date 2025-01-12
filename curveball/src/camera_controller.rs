// This example was copied-and-pasted from the Bevy example directory with minimal changes.

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
    window::CursorGrabMode,
};
use std::{f32::consts::*, fmt};

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
    pub run_speed: f32,
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
            run_speed: 250.0 * 3.0,
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
    pub key_orbit: KeyCode,
    pub mouse_key_cursor_grab: MouseButton,
    pub keyboard_key_toggle_cursor_grab: KeyCode,
    pub keyboard_key_escape_cursor_grab: KeyCode,
    pub scroll_factor: f32,
    pub friction: f32,
    pub min_walkspeed: f32,
    pub max_walkspeed: f32,
    pub run_factor: f32,
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
            key_orbit: KeyCode::AltLeft,
            mouse_key_cursor_grab: MouseButton::Right,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyC,
            keyboard_key_escape_cursor_grab: KeyCode::Escape,
            scroll_factor: 0.2,
            friction: 0.5,
            min_walkspeed: 12.5,
            max_walkspeed: 5000.0,
            run_factor: 3.0,
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
            self.settings.mouse_key_cursor_grab,
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

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
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

    // Handle key input
    let mut axis_input = Vec3::ZERO;
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

    let mut cursor_grab_change = false;
    if key_input.just_pressed(controller.settings.keyboard_key_toggle_cursor_grab) {
        *toggle_cursor_grab = !*toggle_cursor_grab;
        cursor_grab_change = true;
    }
    if key_input.just_pressed(controller.settings.keyboard_key_escape_cursor_grab) {
        *toggle_cursor_grab = false;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_pressed(controller.settings.mouse_key_cursor_grab) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(controller.settings.mouse_key_cursor_grab) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

    // Cursor update
    if cursor_grab {
        let scroll = match accumulated_mouse_scroll.unit {
            MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
            MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
        };
        controller.walk_speed += scroll * controller.settings.scroll_factor * controller.walk_speed;
        controller.walk_speed = controller.walk_speed.clamp(
            controller.settings.min_walkspeed,
            controller.settings.max_walkspeed,
        );
        controller.run_speed = controller.walk_speed * controller.settings.run_factor;
    }

    // Apply movement update
    if axis_input != Vec3::ZERO {
        let max_speed = if key_input.pressed(controller.settings.key_run) {
            controller.run_speed
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

    // Handle cursor grab
    if cursor_grab_change {
        if cursor_grab {
            for mut window in &mut windows {
                if !window.focused {
                    continue;
                }

                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            }
        } else {
            for mut window in &mut windows {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }

    // Handle mouse input
    if cursor_grab {
        if key_input.pressed(controller.settings.key_orbit) {
            // Orbit
            if accumulated_mouse_motion.delta != Vec2::ZERO {
                // Current position in spherical coordinates
                let [r, mut theta, mut phi]: [f32; 3] =
                    cartesian_to_spherical(transform.translation).into();
                phi = phi
                    + accumulated_mouse_motion.delta.x
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity;
                theta = (theta
                    - accumulated_mouse_motion.delta.y
                        * RADIANS_PER_DOT
                        * controller.settings.sensitivity)
                    .clamp(0.000001, PI - 0.000001);
                transform.translation = spherical_to_cartesian(Vec3::new(r, theta, phi));
            }
            *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
        } else if accumulated_mouse_motion.delta != Vec2::ZERO {
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
