use core::time::Duration;
use std::collections::HashMap;

use bevy::prelude::*;

pub struct QuantumDevicePlugin;

impl Plugin for QuantumDevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_quantum_device);
    }
}

fn spawn_quantum_device(mut commands: Commands) {
    commands.spawn(
        QuantumDevice::new(Duration::from_secs(99))
    );
}

#[derive(Component)]
pub struct QuantumDevice {
    timer: Timer,
    histories: Vec<History>,
    current_history: Option<History>,
}

impl QuantumDevice {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            histories: vec![],
            current_history: Some(History::new()),
        }
    }
}

pub struct History {
    actions: HashMap<Action, Duration>,
}

impl History {
    pub fn new() -> Self {
        Self { actions: HashMap::new() }
    }
}

enum Action{}
