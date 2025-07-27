use crate::prelude::*;

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    mb: Res<MapBuilder>,
) {
    for mut pos in player_positions.iter_mut() {
        let mut new_position = pos.clone();
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_position.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            new_position.x += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            new_position.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            new_position.y += 1;
        }

        if mb.map.can_enter_tile(new_position) {
            pos.x = new_position.x;
            pos.y = new_position.y;
        }
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // listening to user input on inventory screen
            .add_systems(Update, player_input);
    }
}
