use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
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
