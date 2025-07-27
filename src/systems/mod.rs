use crate::prelude::*;

mod camera;
mod player_input;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player_input::PlayerInputPlugin)
            .add_systems(Update, camera::camera_move);
    }
}
