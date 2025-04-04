use core::f32::consts::FRAC_PI_2;

use crate::player::Player;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::CursorGrabMode};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorLockState>().add_systems(
            Update,
            (cursor_lock_state, camera_pitch_control, camera_yaw_control),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Resource)]
enum CursorLockState {
    #[default]
    Free,
    Locked,
}

impl CursorLockState {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Free => Self::Locked,
            Self::Locked => Self::Free,
        };
    }

    fn apply_to_bevy_window(self, window: &mut Window) {
        match self {
            Self::Locked => {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            }
            Self::Free => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn cursor_lock_state(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_lock_state: ResMut<CursorLockState>,
    mut window: Single<&mut Window>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        cursor_lock_state.toggle();
        cursor_lock_state.apply_to_bevy_window(&mut window);
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_pitch_control(
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
    mouse_motion: ResMut<AccumulatedMouseMotion>,
    cursor_state: Res<CursorLockState>,
) {
    if *cursor_state == CursorLockState::Free || mouse_motion.delta.length_squared() <= 0.0 {
        return;
    }

    for mut transform in &mut camera_transforms {
        apply_mouse_pitch(&mut transform, mouse_motion.delta.y);
    }
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn apply_mouse_pitch(camera_transform: &mut Transform, pitch_delta: f32) {
    const MOUSE_SENSITIVITY: f32 = -0.0005;
    const PITCH_LIMIT: f32 = FRAC_PI_2 - f32::EPSILON;

    let (current_yaw, current_pitch, current_roll) =
        camera_transform.rotation.to_euler(EulerRot::YXZ);

    let new_pitch = pitch_delta
        .mul_add(MOUSE_SENSITIVITY, current_pitch)
        .clamp(-PITCH_LIMIT, PITCH_LIMIT);

    camera_transform.rotation =
        Quat::from_euler(EulerRot::YXZ, current_yaw, new_pitch, current_roll);
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_yaw_control(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    mouse_motion: ResMut<AccumulatedMouseMotion>,
    cursor_state: Res<CursorLockState>,
) {
    if *cursor_state == CursorLockState::Free || mouse_motion.delta.length_squared() <= 0.0 {
        return;
    }

    for mut transform in &mut player_transforms {
        apply_mouse_yaw(&mut transform, mouse_motion.delta.x);
    }
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn apply_mouse_yaw(camera_transform: &mut Transform, yaw_delta: f32) {
    const MOUSE_SENSITIVITY: f32 = -0.0005;

    camera_transform.rotate_y(yaw_delta * MOUSE_SENSITIVITY);
}
