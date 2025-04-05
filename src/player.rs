use core::f32::consts::FRAC_PI_2;

use crate::input::{CursorLockState, Input};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, camera_pitch_control)
            .add_systems(Update, player_yaw_control);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(PlayerBundle::default())
        .with_children(|parent| {
            parent.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.6, 0.0)));
        });
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: Transform,
    visibility: Visibility,
    kinematic_character_controller: KinematicCharacterController,
    collider: Collider,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        const PLAYER_BOTTOM: Vec3 = Vec3::new(0.0, 0.5, 0.0);
        const PLAYER_TOP: Vec3 = Vec3::new(0.0, 1.5, 0.0);
        const PLAYER_RADIUS: f32 = 0.5;

        Self {
            player: Player,
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            visibility: Visibility::default(),
            kinematic_character_controller: KinematicCharacterController::default(),
            collider: Collider::capsule(PLAYER_BOTTOM, PLAYER_TOP, PLAYER_RADIUS),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_pitch_control(
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
    input: Res<Input>,
) {
    if *input.cursor_lock_state() == CursorLockState::Free || !input.camera_moved() {
        return;
    }

    for mut transform in &mut camera_transforms {
        apply_mouse_pitch(&mut transform, *input.camera_pitch_delta());
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
fn player_yaw_control(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    input: Res<Input>,
) {
    if *input.cursor_lock_state() == CursorLockState::Free || !input.camera_moved() {
        return;
    }

    for mut transform in &mut player_transforms {
        apply_mouse_yaw(&mut transform, *input.camera_yaw_delta());
    }
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn apply_mouse_yaw(camera_transform: &mut Transform, yaw_delta: f32) {
    const MOUSE_SENSITIVITY: f32 = -0.0005;

    camera_transform.rotate_y(yaw_delta * MOUSE_SENSITIVITY);
}
