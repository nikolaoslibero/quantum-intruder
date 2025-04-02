use core::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::CursorGrabMode};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorLockState>()
            .add_systems(Update, (camera_control, cursor_lock_state));
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
fn camera_control(
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
    mouse_motion: ResMut<AccumulatedMouseMotion>,
    cursor_state: Res<CursorLockState>,
) {
    const MOUSE_SENSITIVITY: f32 = 0.0005;

    let mouse_delta = mouse_motion.delta;
    if *cursor_state == CursorLockState::Free || mouse_delta.length_squared() <= 0.0 {
        return;
    }

    for mut transform in &mut camera_transforms {
        apply_yaw_rotation(&mut transform, mouse_delta.x, MOUSE_SENSITIVITY);
        apply_pitch_rotation(&mut transform, mouse_delta.y, MOUSE_SENSITIVITY);
    }
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn apply_yaw_rotation(transform: &mut Transform, mouse_x: f32, sensitivity: f32) {
    transform.rotate_y(-mouse_x * sensitivity);
}

#[expect(clippy::float_arithmetic, reason = "Camera pitch calculation")]
fn apply_pitch_rotation(transform: &mut Transform, mouse_y: f32, sensitivity: f32) {
    const PITCH_LIMIT: f32 = FRAC_PI_2 - f32::EPSILON;

    let (current_yaw, current_pitch, current_roll) = transform.rotation.to_euler(EulerRot::YXZ);

    let desired_pitch = mouse_y.mul_add(-sensitivity, current_pitch);
    let clamped_pitch = desired_pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, current_yaw, clamped_pitch, current_roll);
}
