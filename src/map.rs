use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![TileType::Floor; NUM_TILES];
        tiles[2030] = TileType::Wall;
        Self { tiles }
    }

    pub fn in_bounds(&self, point: Position) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Position) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Position) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn spawn_map_tiles(mut commands: Commands, mb: Res<MapBuilder>, atlas: Res<CharsetAsset>) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);

            match mb.map.tiles[idx] {
                TileType::Floor => {
                    commands.spawn((
                        Sprite {
                            image: atlas.texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: atlas.atlas.clone(),
                                index: 46,
                            }),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..Default::default()
                        },
                        Position { x, y, z: 0 },
                        TileSize::square(1.0),
                    ));
                }
                TileType::Wall => {
                    commands.spawn((
                        Sprite {
                            image: atlas.texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: atlas.atlas.clone(),
                                index: 35,
                            }),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..Default::default()
                        },
                        Position { x, y, z: 0 },
                        TileSize::square(1.0),
                    ));
                }
                TileType::Void => (),
            }
        }
    }
}
