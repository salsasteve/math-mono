use bevy::prelude::*;
use rand::{rng, Rng};

use crate::demo::common::{Position, Shape, get_primary_window_size};

use crate::screens::Screen;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridConfig>();
    }
}

// --- Constants for our blocks ---
// Define the size of a single block
const BLOCK_SIZE: Vec2 = Vec2::new(100., 100.);
// Define the gap between each block
const GAP_BETWEEN_BLOCKS: f32 = 3.;
// Define how many rows and columns of blocks to spawn
const N_ROWS: i32 = 7;
const N_COLS: i32 = 7;

// #[derive(Component)]
// #[require(
//     Position,
//     // Shape is now based on our constant
//     Shape = Shape(BLOCK_SIZE),
// )]
// Renamed the component from Ball to Block for clarity
#[derive(Component)]
pub struct Block {
    pub value: i32,
}

#[derive(Component)]
pub struct Player;

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
            rows: N_ROWS,
            cols: N_COLS,
            block_size: BLOCK_SIZE,
            gap_between_blocks: GAP_BETWEEN_BLOCKS,
        }
    }
}

/// Calculates the total size of the grid and the bottom-left coordinate to center it.
pub fn calculate_grid_layout(config: &GridConfig) -> (Vec2, Vec2) {
    let total_width = config.cols as f32 * (config.block_size.x + config.gap_between_blocks)
        - config.gap_between_blocks;
    let total_height = config.rows as f32 * (config.block_size.y + config.gap_between_blocks)
        - config.gap_between_blocks;
    let total_size = Vec2::new(total_width, total_height);
    let bottom_left = Vec2::new(-total_width / 2.0, -total_height / 2.0);
    (total_size, bottom_left)
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

    let Some(window_size) = get_primary_window_size(windows) else {
        println!("No primary window found.");
        return;
    };

    let (total_size, grid_bottom_left) = calculate_grid_layout(&config);
    println!("Total grid size: {:?}", total_size);
    println!("Grid bottom-left corner: {:?}", grid_bottom_left);

    spawn_background(&mut commands, &mut meshes, &mut materials, &window_size);

    // --- Create a mesh and material for all blocks to share ---
    let shape = Rectangle::new(config.block_size.x, config.block_size.y);
    let block_mesh: Handle<Mesh> = meshes.add(shape);
    let font: Handle<Font> = asset_server.load("fonts/MonofurNerdFont-Bold.ttf");
    let mut rng = rand::rng();

    // --- Loop to spawn multiple blocks in a grid ---
    for row in 0..N_ROWS {
        for col in 0..N_COLS {
            let value = rng.random_range(1..=100);
            let block_center_position =
                calculate_block_center(&config, grid_bottom_left, row, col);
            println!("Block center position: {:?}", block_center_position);

            spawn_block(
                &mut commands,
                &mut materials,
                &block_mesh,
                &font,
                block_center_position,
                value,
                row,
                col,
            );
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
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mesh: &Handle<Mesh>,
    font: &Handle<Font>,      // <-- ADD THIS
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
    let material = materials.add(color);

    commands.spawn((
        Block{ value: row * N_COLS + col + 1 },
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material),
        Position(position),
        Transform::from_translation(position.extend(0.0)),
        StateScoped(Screen::Gameplay),
    ))
    .with_children(|builder| {
            builder.spawn((
                Text2d::new(value.to_string()),
                TextFont { font: font.clone(), ..default() },
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                 Transform::from_translation(Vec3::Z * 0.1),
            ));
        });
}


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
        let (total_size, bottom_left) = calculate_grid_layout(&config);

        assert_eq!(total_size, Vec2::new(210.0, 210.0));
        // bottom_left should be (-105, -105) to center it
        assert_eq!(bottom_left, Vec2::new(-105.0, -105.0));
    }

    #[test]
    fn test_calculate_block_center_for_origin_block() {
        let config = GridConfig::default(); // 5x5 grid
        let (_, bottom_left) = calculate_grid_layout(&config);

        // Position of the block at (row: 0, col: 0)
        let pos_0_0 = calculate_block_center(&config, bottom_left, 0, 0);

        // Expected: bottom_left + BLOCK_SIZE / 2
        // (-354.5 + 70.0, -354.5 + 70.0)
        let expected_pos = Vec2::new(-284.5, -284.5);
        assert!((pos_0_0 - expected_pos).length() < 1e-6);
    }
}
