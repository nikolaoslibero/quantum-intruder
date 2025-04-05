use bevy::prelude::*;

#[cfg(feature = "default")]
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);

        #[cfg(feature = "default")]
        app.add_plugins(FpsOverlayPlugin::default());
    }
}

const fn spawn_ui(mut commands: Commands) {}
