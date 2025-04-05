use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::CursorGrabMode};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Input>().add_systems(
            Update,
            (
                update_cursor_lock_state,
                apply_cursor_lock_state,
                camera_delta,
                translation_angle,
            ),
        );
    }
}

#[derive(Resource, Default)]
pub struct Input {
    camera_delta: Vec2,
    cursor_lock_state: CursorLockState,
    translation_angle: Option<f32>,
}

impl Input {
    pub fn camera_moved(&self) -> bool {
        self.camera_delta.length_squared() > 0.0
    }

    pub const fn camera_pitch_delta(&self) -> &f32 {
        &self.camera_delta.y
    }

    pub const fn camera_yaw_delta(&self) -> &f32 {
        &self.camera_delta.x
    }

    pub const fn cursor_lock_state(&self) -> &CursorLockState {
        &self.cursor_lock_state
    }

    pub const fn translation_angle(&self) -> Option<&f32> {
        self.translation_angle.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorLockState {
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
fn update_cursor_lock_state(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<Input>,
) {
    mouse_buttons.just_pressed(MouseButton::Right).then(|| {
        input.cursor_lock_state.toggle();
    });
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn apply_cursor_lock_state(input: Res<Input>, mut window: Single<&mut Window>) {
    input.cursor_lock_state.apply_to_bevy_window(&mut window);
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_delta(mouse_motion: ResMut<AccumulatedMouseMotion>, mut input: ResMut<Input>) {
    input.camera_delta = mouse_motion.delta;
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn translation_angle(keyboard_buttons: Res<ButtonInput<KeyCode>>, mut input: ResMut<Input>) {
    let direction = get_keys_vector(&keyboard_buttons);

    input.translation_angle = (direction != Vec2::ZERO).then_some(direction.x.atan2(direction.y));
}

#[expect(clippy::arithmetic_side_effects, reason = "Direction calculation")]
fn get_keys_vector(keyboard_buttons: &Res<'_, ButtonInput<KeyCode>>) -> Vec2 {
    const DIRECTION_VALUES: [(KeyCode, Vec2); 4] = [
        (KeyCode::KeyW, Vec2::new(0.0, 1.0)),
        (KeyCode::KeyS, Vec2::new(0.0, -1.0)),
        (KeyCode::KeyA, Vec2::new(1.0, 0.0)),
        (KeyCode::KeyD, Vec2::new(-1.0, 0.0)),
    ];

    DIRECTION_VALUES
        .iter()
        .filter(|&&(key, _)| keyboard_buttons.pressed(key))
        .fold(Vec2::ZERO, |acc, &(_, direction)| acc + direction)
        .normalize_or_zero()
}
