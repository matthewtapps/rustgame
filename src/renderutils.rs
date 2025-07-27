use crate::prelude::*;

pub fn position_translation(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = pos.x as f32 * 8.0; // 8.0 is tile size
        transform.translation.y = pos.y as f32 * 8.0;
        transform.translation.z = pos.z as f32;
    }
}

pub fn size_scaling(mut query: Query<(&TileSize, &mut Transform)>) {
    for (tile_size, mut transform) in query.iter_mut() {
        transform.scale.x = tile_size.width * 8.0; // 8.0 is base tile size
        transform.scale.y = tile_size.height * 8.0;
    }
}

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}
