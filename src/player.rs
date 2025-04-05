use core::f32::consts::FRAC_PI_2;

use crate::input::{CursorLockState, Input};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, camera_pitch_control)
            .add_systems(Update, player_yaw_control)
            .add_systems(Update, player_movement);
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

impl Player {
    const SPEED: f32 = 10.0;
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_pitch_control(
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
    input: Res<Input>,
) {
    (*input.cursor_lock_state() != CursorLockState::Free).then(|| {
        camera_transforms.iter_mut().for_each(|mut transform| {
            transform.rotation =
                calculate_pitch_rotation(&transform.rotation, *input.camera_pitch_delta());
        });
    });
}

#[expect(clippy::float_arithmetic, reason = "Transform rotation")]
fn calculate_pitch_rotation(current_rotation: &Quat, pitch_delta: f32) -> Quat {
    const PITCH_LIMIT: f32 = FRAC_PI_2 - f32::EPSILON;

    let (current_yaw, current_pitch, current_roll) = current_rotation.to_euler(EulerRot::YXZ);

    let new_pitch = (pitch_delta + current_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    Quat::from_euler(EulerRot::YXZ, current_yaw, new_pitch, current_roll)
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn player_yaw_control(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    input: Res<Input>,
) {
    (*input.cursor_lock_state() != CursorLockState::Free).then(|| {
        player_transforms.iter_mut().for_each(|mut transform| {
            transform.rotate_y(*input.camera_yaw_delta());
        });
    });
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn player_movement(
    mut motion_query: Query<(&mut KinematicCharacterController, &Transform)>,
    input: Res<Input>,
    time: Res<Time>,
) {
    (*input.cursor_lock_state() != CursorLockState::Free).then(|| {
        motion_query
            .iter_mut()
            .for_each(|(mut controller, transform)| {
                controller.translation =
                    calculate_player_translation(transform, &input, time.delta_secs());
            });
    });
}

#[expect(clippy::arithmetic_side_effects, reason = "Transform calculation")]
fn calculate_player_translation(
    transform: &Transform,
    input: &Input,
    delta_secs: f32,
) -> Option<Vec3> {
    input.translation_angle().map(|angle| {
        transform.rotation
            * Quat::from_rotation_y(*angle)
            * Vec3::NEG_Z
            * Player::SPEED
            * delta_secs
    })
}
