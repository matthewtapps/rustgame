mod components;
mod map;
mod map_builder;
mod renderutils;
mod resources;
mod spawner;
mod systems;

mod prelude {
    pub use bevy::prelude::*;
    pub use rand::Rng;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::renderutils::*;
    pub use crate::resources::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup the sprite sheet
    let texture_handle: Handle<Image> = asset_server.load("terminal8x8.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(8, 8), 16, 16, None, None);
    let layout_handle = atlases.add(layout); // add sprite atlas as resource

    commands.insert_resource(CharsetAsset {
        atlas: layout_handle.clone(),
        texture: texture_handle.clone(),
    });

    // Add a 2D Camera
    commands.spawn((
        MainCamera,
        Camera2d,
        Transform {
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roguelike Game".to_string(),
                        resolution: (80.0 * 10.0, 50.0 * 10.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_plugins(MapPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(SystemsPlugin)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}
