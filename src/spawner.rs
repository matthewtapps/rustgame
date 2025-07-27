use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, mb: Res<MapBuilder>, atlas: Res<CharsetAsset>) {
    commands.spawn((
        Sprite {
            image: atlas.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.atlas.clone(),
                index: 64, // @ symbol
            }),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        Position {
            x: mb.player_start.x,
            y: mb.player_start.y,
            z: 1,
        },
        TileSize::square(1.0),
        Player,
    ));
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}
