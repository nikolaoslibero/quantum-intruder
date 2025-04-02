use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use core::num::NonZero;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct MouseSensitivity(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .init_resource::<MouseSensitivity>()
        .add_systems(Startup, (spawn_level, spawn_player))
        .add_systems(Update, (camera_control, toggle_mouse_grab_mode))
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

impl Default for MouseSensitivity {
    fn default() -> Self {
        Self(0.0005)
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_control(
    mut camera_transforms: Query<&mut Transform, With<Camera3d>>,
    mouse_motion: ResMut<AccumulatedMouseMotion>,
    sensitivity: Res<MouseSensitivity>,
    window: Single<&mut Window>,
) {
    if window.cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let mouse_delta = mouse_motion.delta;
    if mouse_delta.length_squared() <= 0.0 {
        return;
    }

    for mut transform in &mut camera_transforms {
        apply_yaw_rotation(&mut transform, mouse_delta.x, sensitivity.0);

        apply_pitch_rotation(&mut transform, mouse_delta.y, sensitivity.0);
    }
}

fn apply_yaw_rotation(transform: &mut Transform, mouse_x: f32, sensitivity: f32) {
    #[expect(clippy::float_arithmetic, reason = "Transform rotation")]
    transform.rotate_y(-mouse_x * sensitivity);
}

fn apply_pitch_rotation(transform: &mut Transform, mouse_y: f32, sensitivity: f32) {
    const PITCH_MIN: f32 = -1.5;
    const PITCH_MAX: f32 = 1.5;
    let (current_yaw, current_pitch, current_roll) = transform.rotation.to_euler(EulerRot::YXZ);

    #[expect(clippy::float_arithmetic, reason = "Camera pitch calculation")]
    let desired_pitch = mouse_y.mul_add(-sensitivity, current_pitch);
    let clamped_pitch = desired_pitch.clamp(PITCH_MIN, PITCH_MAX);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, current_yaw, clamped_pitch, current_roll);
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn toggle_mouse_grab_mode(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut window: Single<&mut Window>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        if window.cursor_options.grab_mode == CursorGrabMode::None {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Rust/Bevy hate me.")]
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
    const PLAYER_BOTTOM: Vec3 = Vec3::new(0.0, 0.5, 0.0);
    const PLAYER_TOP: Vec3 = Vec3::new(0.0, 1.5, 0.0);
    const PLAYER_RADIUS: f32 = 0.5;

    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
            KinematicCharacterController::default(),
            Collider::capsule(PLAYER_BOTTOM, PLAYER_TOP, PLAYER_RADIUS),
        ))
        .with_children(|parent| {
            parent.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.6, 0.0)));
        });
}
