use crate::prelude::*;

mod camera;
mod player_input;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player_input::PlayerInputPlugin)
            .add_systems(Update, camera::camera_move);
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera::camera_move);
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AwaitingInputPlugin)
            .add_plugins(PlayerPlugin);
    }
}
