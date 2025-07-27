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
