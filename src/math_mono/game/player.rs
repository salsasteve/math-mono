//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    math_mono::{
        animation::PlayerAnimation,
        components::{GridPosition, NumberBlock, Question},
        game::{calculate_block_center, calculate_grid_layout, clamp_grid_position, grid::GridConfig},
    },
    screens::Screen,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .add_systems(OnEnter(Screen::Gameplay), spawn_player)
            .add_systems(
                Update,
                (
                    move_player_on_grid,
                    sync_player_to_grid_position.after(move_player_on_grid),
                    eat_number_on_spacebar,
                    update_block_visuals,
                )
                    .run_if(in_state(Screen::Gameplay)),
            );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<GridConfig>,
) {
    // Start the player in the middle of the grid
    let start_row = config.rows / 2;
    let start_col = config.cols / 2;

    let layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
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

pub fn move_player_on_grid(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut GridPosition, With<Player>>,
    config: Res<GridConfig>,
) {
    let Ok(mut grid_pos) = player_query.single_mut() else {
        return;
    };

    let mut moved = false;

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        grid_pos.row += 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
        grid_pos.row -= 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        grid_pos.col -= 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight) {
        grid_pos.col += 1;
        moved = true;
    }

    if moved {
        // Clamp the position to stay within the grid bounds
        clamp_grid_position(&mut grid_pos, &config);
    }
}

// Updates the player's Transform to match its GridPosition
pub fn sync_player_to_grid_position(
    config: Res<GridConfig>,
    mut player_query: Query<(&GridPosition, &mut Transform), (With<Player>, Changed<GridPosition>)>,
) {
    let Ok((grid_pos, mut transform)) = player_query.single_mut() else {
        return;
    };

    // We can reuse the grid calculation logic we already wrote!
    let (_, _, bottom_left_x, bottom_left_y) = calculate_grid_layout(&config);
    let bottom_left = Vec2::new(bottom_left_x, bottom_left_y);
    let new_world_pos = calculate_block_center(&config, bottom_left, grid_pos.row, grid_pos.col);

    // Update the player's actual world position.
    // We give it a higher Z value to make sure it renders on top of the grid.
    transform.translation = new_world_pos.extend(1.0);
}

pub fn eat_number_on_spacebar(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&GridPosition, With<Player>>,
    mut block_query: Query<(&mut NumberBlock, &GridPosition), Without<Player>>,
) {
    // Only eat when spacebar is pressed
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        return;
    };


    // Find the block at the player's position
    for (mut block, block_pos) in block_query.iter_mut() {
        if player_pos == block_pos && !block.is_eaten {
            block.is_eaten = true;
            println!("Player position: {:?}", player_pos);
            break; // Only eat one block per spacebar press
        }
    }
}

pub fn update_block_visuals(
    block_query: Query<(&NumberBlock, &Children), Changed<NumberBlock>>,
    mut text_query: Query<&mut Visibility>,
) {

    for (block, children) in block_query.iter() {
        for child in children.iter() {
            if let Ok(mut visibility) = text_query.get_mut(child) {
                // Hide text if block is eaten
                *visibility = if block.is_eaten {
                    println!("Updating block visuals");
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                };
            }
        }
    }
}
