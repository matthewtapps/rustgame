use crate::prelude::*;

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &mut Position,
            &MovementStats,
            Option<&mut PendingMovement>,
        ),
        With<Player>,
    >,
    mb: Res<MapBuilder>,
    mut commands: Commands,
    mut game_tick: ResMut<GameTick>,
    mut movement_state: ResMut<MovementState>,
) {
    for (entity, mut pos, movement_stats, pending_movement) in player_query.iter_mut() {
        // If not currently in movement mode, check for movement initiation
        if !movement_state.is_moving {
            let mut movement_attempted = false;
            let mut new_position = pos.clone();

            if keyboard_input.just_pressed(KeyCode::KeyA) {
                new_position.x -= 1;
                movement_attempted = true;
            }
            if keyboard_input.just_pressed(KeyCode::KeyD) {
                new_position.x += 1;
                movement_attempted = true;
            }
            if keyboard_input.just_pressed(KeyCode::KeyS) {
                new_position.y -= 1;
                movement_attempted = true;
            }
            if keyboard_input.just_pressed(KeyCode::KeyW) {
                new_position.y += 1;
                movement_attempted = true;
            }

            if movement_attempted && mb.map.can_enter_tile(new_position) {
                // Start movement action
                let start_pos = *pos; // Store original position
                pos.x = new_position.x;
                pos.y = new_position.y;
                movement_state.is_moving = true;

                // Add pending movement component to track this action
                commands.entity(entity).insert(PendingMovement {
                    start_position: start_pos,
                    tiles_moved: 1,
                });

                println!(
                    "Started movement action. Moved 1/{} tiles",
                    movement_stats.max_move_distance
                );
            }
        } else {
            // Currently in movement mode - can continue moving or confirm
            if let Some(mut pending) = pending_movement {
                if keyboard_input.just_pressed(KeyCode::Enter)
                    || keyboard_input.just_pressed(KeyCode::Space)
                {
                    // Confirm movement action - advance tick
                    game_tick.advance();
                    movement_state.is_moving = false;

                    // Remove pending movement component
                    // Note: You'd need to store the entity ID to do this properly
                    println!(
                        "Movement action completed! Moved {} tiles. Tick: {}",
                        pending.tiles_moved, game_tick.current
                    );
                    return;
                }

                // Continue movement if within limits
                if pending.tiles_moved < movement_stats.max_move_distance {
                    let mut new_position = pos.clone();
                    let mut moved = false;

                    if keyboard_input.just_pressed(KeyCode::KeyA) {
                        new_position.x -= 1;
                        moved = true;
                    }
                    if keyboard_input.just_pressed(KeyCode::KeyD) {
                        new_position.x += 1;
                        moved = true;
                    }
                    if keyboard_input.just_pressed(KeyCode::KeyS) {
                        new_position.y -= 1;
                        moved = true;
                    }
                    if keyboard_input.just_pressed(KeyCode::KeyW) {
                        new_position.y += 1;
                        moved = true;
                    }

                    if moved && mb.map.can_enter_tile(new_position) {
                        pos.x = new_position.x;
                        pos.y = new_position.y;
                        pending.tiles_moved += 1;

                        println!(
                            "Continued movement. Moved {}/{} tiles",
                            pending.tiles_moved, movement_stats.max_move_distance
                        );

                        // Auto-complete if reached max distance
                        if pending.tiles_moved >= movement_stats.max_move_distance {
                            game_tick.advance();
                            movement_state.is_moving = false;
                            println!(
                                "Movement action auto-completed! Tick: {}",
                                game_tick.current
                            );
                        }
                    }
                }
            }
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
