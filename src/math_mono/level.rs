//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource, audio::music, math_mono::game::spawn_grid,
};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelAssets>();
        app.load_resource::<LevelAssets>();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}


pub fn spawn_level(
        mut commands: Commands,
        level_assets: Res<LevelAssets>,
        config: Res<crate::math_mono::game::grid::GridConfig>,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
        window_size: Query<&Window>,
        asset_server: Res<AssetServer>,
) {
    let level_entity = commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(crate::screens::Screen::Gameplay),
    )).id();

    // Spawn music
    commands.spawn((
        Name::new("Gameplay Music"),
        music(level_assets.music.clone()),
    )).insert(ChildOf(level_entity));

    // Spawn grid
    spawn_grid( commands, meshes, materials, window_size, config, asset_server);

}
