use crate::input::InputPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::num::NonZero;

mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(InputPlugin)
        .add_systems(Startup, (spawn_level, spawn_player))
        .run();
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Quantum Intruder".to_owned(),
            desired_maximum_frame_latency: NonZero::new(1u32),
            ..default()
        }),
        ..default()
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = 100.0;

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("elevator-room.gltf"))),
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));
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
struct Player;
