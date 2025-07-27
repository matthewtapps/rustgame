use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct TileSize {
    pub width: f32,
    pub height: f32,
}

impl TileSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
pub struct MovementStats {
    pub max_move_distance: i32,
}

impl Default for MovementStats {
    fn default() -> Self {
        Self {
            max_move_distance: 6,
        }
    }
}

#[derive(Component)]
pub struct PendingMovement {
    pub start_position: Position,
    pub tiles_moved: i32,
}
