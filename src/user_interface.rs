use bevy::{dev_tools::fps_overlay::FpsOverlayConfig, prelude::*};

#[cfg(feature = "default")]
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;

use crate::input::Input;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_ui);

        #[cfg(feature = "default")]
        Self::default_build(app);
    }
}

impl UserInterfacePlugin {
    #[cfg(feature = "default")]
    fn default_build(app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ..default()
            },
        });
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(DebugTextBundle::default());
}

#[derive(Bundle)]
struct DebugTextBundle {
    debug_text: DebugText,
    text: Text,
    text_font: TextFont,
    node: Node,
}

impl Default for DebugTextBundle {
    fn default() -> Self {
        Self {
            debug_text: DebugText,
            text: Text::new(""),
            text_font: TextFont {
                font_size: 16.0,
                ..default()
            },
            node: Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
        }
    }
}

#[derive(Component, Default)]
struct DebugText;

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
#[expect(clippy::too_many_arguments, reason = "Excuse my debug mess")]
fn update_ui(
    mut text_query: Query<&mut Text, With<DebugText>>,
    input: Res<Input>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_camera: Query<&Transform, With<Camera3d>>,
) {
    text_query.iter_mut().for_each(|mut text| {
        let (yaw, pitch, roll) = player_camera.single().rotation.to_euler(EulerRot::YXZ);
        text.0 = format!(
            "cursor: {:?}\ntranslation angle: {:?}\nkeys: {:?}\npitch: {:.2?}",
            input.cursor_lock_state(),
            input.translation_angle(),
            keyboard.get_pressed().collect::<Vec<_>>(),
            pitch,
        );
    });
}
