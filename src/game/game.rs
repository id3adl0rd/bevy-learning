use bevy::prelude::*;

use super::window::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(window::WindowSettingsPlugin);
    }
}