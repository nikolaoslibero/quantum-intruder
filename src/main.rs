use crate::input::InputPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::num::NonZero;
use player::PlayerPlugin;
use user_interface::UserInterfacePlugin;
mod input;
mod player;
mod user_interface;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(UserInterfacePlugin)
        .add_systems(Startup, spawn_level)
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
