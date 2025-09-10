//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    asset_tracking::LoadResource,
    demo::{
        animation::PlayerAnimation, components::GridPosition, game::grid::GridConfig, systems::movement::{move_player_on_grid, sync_player_to_grid_position},
    },
    screens::Screen,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerAssets>()
            .load_resource::<PlayerAssets>()
            .register_type::<GridPosition>()
            .add_systems(OnEnter(Screen::Gameplay), spawn_player_on_grid)
            .add_systems(
                Update,
                (
                    move_player_on_grid,
                    sync_player_to_grid_position.after(move_player_on_grid),
                )
                .run_if(in_state(Screen::Gameplay)),
            );
    }
}


pub fn spawn_player_on_grid(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<GridConfig>,
) {
    // Start the player in the middle of the grid
    let start_row = config.rows / 2;
    let start_col = config.cols / 2;

    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player,
        GridPosition {
            row: start_row,
            col: start_col,
        },
        Sprite::from_atlas_image(
            player_assets.ducky.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        player_animation,
        Transform::from_scale(Vec2::splat(3.0).extend(0.2)),
        StateScoped(Screen::Gameplay),
    ));
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;


#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
