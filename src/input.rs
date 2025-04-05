use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::CursorGrabMode};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Input>()
            .add_systems(Update, (cursor_lock_state, camera_delta));
    }
}

#[derive(Resource, Default)]
pub struct Input {
    camera_delta: Vec2,
    cursor_lock_state: CursorLockState,
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
fn cursor_lock_state(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<Input>,
    mut window: Single<&mut Window>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        input.cursor_lock_state.toggle();
        input.cursor_lock_state.apply_to_bevy_window(&mut window);
    }
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn camera_delta(mouse_motion: ResMut<AccumulatedMouseMotion>, mut input: ResMut<Input>) {
    input.camera_delta = mouse_motion.delta;
}
