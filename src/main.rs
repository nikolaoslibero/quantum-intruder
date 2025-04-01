use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::num::NonZero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
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
