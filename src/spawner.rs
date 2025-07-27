use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, mb: Res<MapBuilder>, atlas: Res<CharsetAsset>) {
    commands.spawn((
        Sprite::from_atlas_image(
            atlas.texture.clone(),
            TextureAtlas {
                layout: atlas.atlas.clone(),
                index: 64,
            },
        ),
        Transform::from_xyz(mb.player_start.x as f32, mb.player_start.y as f32, 1.0),
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
