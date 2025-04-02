use core::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::CursorGrabMode};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorLockState>()
            .add_systems(Update, (cursor_lock_state, camera_control));
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
    if *cursor_state == CursorLockState::Free || mouse_motion.delta.length_squared() <= 0.0 {
        return;
    }

    for mut transform in &mut camera_transforms {
        apply_mouse_rotation(&mut transform, mouse_motion.delta);
    }
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn apply_mouse_rotation(transform: &mut Transform, delta: Vec2) {
    const MOUSE_SENSITIVITY: f32 = -0.0005;
    const PITCH_LIMIT: f32 = FRAC_PI_2 - f32::EPSILON;

    let (yaw_delta, pitch_delta) = delta.into();
    let (current_yaw, current_pitch, current_roll) = transform.rotation.to_euler(EulerRot::YXZ);

    let new_yaw = yaw_delta.mul_add(MOUSE_SENSITIVITY, current_yaw);
    let new_pitch = pitch_delta
        .mul_add(MOUSE_SENSITIVITY, current_pitch)
        .clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, current_roll);
}
