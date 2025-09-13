//! Grid calculations and number placement logic
use bevy::prelude::*;
use rand::Rng;

use crate::math_mono::common::{Position, get_primary_window_size};

use crate::screens::Screen;

use crate::math_mono::components::{NumberBlock, GridPosition};

/// Grid configuration constants
pub const GRID_ROWS: i32 = 7;
pub const GRID_COLS: i32 = 7;
pub const BLOCK_HEIGHT: f32 = 100.;
pub const BLOCK_WIDTH: f32 = 100.;
pub const BLOCK_SIZE: Vec2 = Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT);

pub const GAP_BETWEEN_BLOCKS: f32 = 3.0;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridConfig>().add_systems(
            Update,
            animate_colors_to_music.run_if(in_state(Screen::Gameplay)),
        );
    }
}

#[derive(Resource)]
pub struct GridConfig {
    pub rows: i32,
    pub cols: i32,
    pub block_size: Vec2,
    pub gap_between_blocks: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            rows: GRID_ROWS,
            cols: GRID_COLS,
            block_size: BLOCK_SIZE,
            gap_between_blocks: GAP_BETWEEN_BLOCKS,
        }
    }
}

/// Calculate the total size of the grid and bottom-left coordinate to center it
pub fn calculate_grid_layout(config: &GridConfig) -> (f32, f32, f32, f32) {
    let total_width = config.cols as f32 * (config.block_size.x + config.gap_between_blocks)
        - config.gap_between_blocks;
    let total_height = config.rows as f32 * (config.block_size.y + config.gap_between_blocks)
        - config.gap_between_blocks;
    let bottom_left_x = -total_width / 2.0;
    let bottom_left_y = -total_height / 2.0;
    (total_width, total_height, bottom_left_x, bottom_left_y)
}

/// Calculates the center position of a single block based on its row and column.
pub fn calculate_block_center(
    config: &GridConfig,
    grid_bottom_left: Vec2,
    row: i32,
    col: i32,
) -> Vec2 {
    let bottom_left_of_block = grid_bottom_left
        + Vec2::new(
            col as f32 * (config.block_size.x + config.gap_between_blocks),
            row as f32 * (config.block_size.y + config.gap_between_blocks),
        );
    bottom_left_of_block + config.block_size / 2.0
}

// Renamed the function from spawn_ball to spawn_blocks
pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    config: Res<GridConfig>,
    asset_server: Res<AssetServer>,
) {
    println!("Spawning blocks...");

    let window_size = get_primary_window_size(windows.single().unwrap());


    let (total_width, total_height, bottom_left_x, bottom_left_y) = calculate_grid_layout(&config);
    let total_size = Vec2::new(total_width, total_height);
    let grid_bottom_left = Vec2::new(bottom_left_x, bottom_left_y);
    println!("Total grid size: {:?}", total_size);
    println!("Grid bottom-left corner: {:?}", grid_bottom_left);

    spawn_background(&mut commands, &mut meshes, &mut materials, &window_size);

    let font: Handle<Font> = asset_server.load("fonts/MonofurNerdFont-Bold.ttf");
    let mut rng = rand::rng();

    // --- Loop to spawn multiple blocks in a grid ---
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let value = rng.random_range(1..=100);
            let block_center_position = calculate_block_center(&config, grid_bottom_left, row, col);
            println!("Block center position: {:?}", block_center_position);

            spawn_block(&mut commands, &font, block_center_position, value, row, col);
        }
    }
}

/// Helper function to spawn just the background.
fn spawn_background(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Vec2,
) {
    let bg_mesh = meshes.add(Rectangle::new(window.x, window.y));
    let bg_material = materials.add(Color::BLACK);

    commands.spawn((
        Name::new("Grid Background"),
        Mesh2d(bg_mesh),
        MeshMaterial2d(bg_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        StateScoped(Screen::Gameplay),
    ));
}

/// Helper function to spawn a single block.
fn spawn_block(
    commands: &mut Commands,
    font: &Handle<Font>,
    position: Vec2,
    value: i32,
    row: i32,
    col: i32,
) {
    // Generate a unique color for this block
    let color = Color::srgb(
        0.2 + 0.1 * row as f32,
        0.6 + 0.05 * col as f32,
        0.8 - 0.1 * row as f32,
    );

    let p_vec3 = position.extend(0.0);

    commands
        .spawn((
            NumberBlock { value, is_eaten: false },
            Sprite {
                color,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            Transform::from_translation(p_vec3),
            Position(position),
            GridPosition { row, col },
            StateScoped(Screen::Gameplay),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(value.to_string()),
                TextFont {
                    font: font.clone(),
                    ..default()
                },
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                Transform::from_translation(Vec3::Z * 0.1),
            ));
        });
}

fn animate_colors_to_music(mut query: Query<(&NumberBlock, &mut Sprite)>, time: Res<Time>) {
    for (block, mut sprite) in &mut query {
        let time_value = time.elapsed_secs();
        let red = (time_value + block.value as f32 * 0.1).sin() * 0.5 + 0.5;
        let green = (time_value + block.value as f32 * 0.2).sin() * 0.5 + 0.5;

        sprite.color = Color::srgb(red, green, 0.8);
    }
}

// /// Check if a grid position is valid (within bounds)
// pub fn is_valid_grid_position(pos: &GridPosition, config: &GridConfig) -> bool {
//     pos.row >= 0 && pos.row < config.rows && pos.col >= 0 && pos.col < config.cols
// }

/// Clamp a grid position to valid bounds
pub fn clamp_grid_position(pos: &mut GridPosition, config: &GridConfig) {
    pos.row = pos.row.clamp(0, config.rows - 1);
    pos.col = pos.col.clamp(0, config.cols - 1);
}

// /// Generate a random number for a grid cell
// pub fn generate_random_number() -> i32 {
//     use rand::Rng;
//     rand::rng().random_range(1..=100)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_grid_layout() {
        let config = GridConfig {
            rows: 2,
            cols: 2,
            block_size: Vec2::new(100.0, 100.0),
            gap_between_blocks: 10.0,
        };
        // Total size should be (2 * 100 + 1 * 10) = 210
        let (total_width, total_height, bottom_left_x, bottom_left_y) =
            calculate_grid_layout(&config);
        let total_size = Vec2::new(total_width, total_height);
        let bottom_left = Vec2::new(bottom_left_x, bottom_left_y);

        assert_eq!(total_size, Vec2::new(210.0, 210.0));
        // bottom_left should be (-105, -105) to center it
        assert_eq!(bottom_left, Vec2::new(-105.0, -105.0));
    }

    #[test]
    fn test_calculate_block_center_for_origin_block() {
        // Use the actual default config (7x7 grid)
        let config = GridConfig::default();
        let (_, _, bottom_left_x, bottom_left_y) = calculate_grid_layout(&config);
        let bottom_left = Vec2::new(bottom_left_x, bottom_left_y);

        // Position of the block at (row: 0, col: 0)
        let pos_0_0 = calculate_block_center(&config, bottom_left, 0, 0);

        // Calculate expected position for 7x7 grid with 100px blocks and 3px gaps
        // Total width: 7 * 100 + 6 * 3 = 700 + 18 = 718
        // Bottom left: -718/2 = -359
        // Block center: -359 + 100/2 = -359 + 50 = -309
        let expected_pos = Vec2::new(-309.0, -309.0);
        assert!((pos_0_0 - expected_pos).length() < 1e-6);
    }

    // #[test]
    // fn test_is_valid_grid_position() {
    //     let config = GridConfig::default();

    //     let valid_pos = GridPosition { row: 3, col: 3 };
    //     assert!(is_valid_grid_position(&valid_pos, &config));

    //     let invalid_pos = GridPosition { row: -1, col: 3 };
    //     assert!(!is_valid_grid_position(&invalid_pos, &config));

    //     let invalid_pos2 = GridPosition { row: 3, col: 10 };
    //     assert!(!is_valid_grid_position(&invalid_pos2, &config));
    // }
}
