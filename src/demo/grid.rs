use bevy::prelude::*;

use crate::demo::common::{Position, Shape};

use crate::screens::Screen;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, _app: &mut App) {}
}

// --- Constants for our blocks ---
// Define the size of a single block
const BLOCK_SIZE: Vec2 = Vec2::new(140., 140.);
// Define the gap between each block
const GAP_BETWEEN_BLOCKS: f32 = 2.;
// Define how many rows and columns of blocks to spawn
const N_ROWS: i32 = 5;
const N_COLS: i32 = 5;

#[derive(Component)]
#[require(
    Position,
    // Shape is now based on our constant
    Shape = Shape(BLOCK_SIZE),
)]
// Renamed the component from Ball to Block for clarity
pub struct Block;

// Renamed the function from spawn_ball to spawn_blocks
pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    println!("Spawning blocks...");

    // Get the primary window size
    let window = match windows.single() {
        Ok(window) => window,
        Err(e) => {
            println!("Could not get primary window: {:?}", e);
            return;
        }
    };
    let window_width = window.width();
    let window_height = window.height();

    // Calculate the total size of the grid of blocks
    let total_width = N_COLS as f32 * (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS) - GAP_BETWEEN_BLOCKS;
    let total_height = N_ROWS as f32 * (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS) - GAP_BETWEEN_BLOCKS;
    // Calculate the bottom-left corner of the grid to center it
    let bottom_left = Vec2::new(-total_width / 2.0, -total_height / 2.0);

    // --- Create a mesh and material for all blocks to share ---
    let shape = Rectangle::new(BLOCK_SIZE.x, BLOCK_SIZE.y);
    let bg_shape = Rectangle::new(window_width, window_height);
    let bg_mesh: Handle<Mesh> = meshes.add(bg_shape);
    let bg_material: Handle<ColorMaterial> = materials.add(Color::BLACK);


     commands.spawn((
        Name::new("Grid Background"),
        Mesh2d(bg_mesh.clone()),
        MeshMaterial2d(bg_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // Z = -1, behind blocks
        StateScoped(Screen::Gameplay),
    ));

    let block_mesh: Handle<Mesh> = meshes.add(shape);

    // --- Loop to spawn multiple blocks in a grid ---
    for row in 0..N_ROWS {
        for col in 0..N_COLS {
            // Calculate the position for each block
            let bottom_left_of_block = bottom_left
                + Vec2::new(
                    col as f32 * (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS),
                    row as f32 * (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS),
                );
            println!("Block position: {:?}", bottom_left_of_block);
            let color = Color::srgb(
                0.2 + 0.1 * row as f32,
                0.6 + 0.05 * col as f32,
                0.8 - 0.1 * row as f32,
            );
            let block_material = materials.add(color);
            let block_center_position = bottom_left_of_block + BLOCK_SIZE / 2.0;
            println!("Block center position: {:?}", block_center_position);

            // Spawn a block entity
            commands.spawn((
                Block,
                Mesh2d(block_mesh.clone()),
                MeshMaterial2d(block_material),
                Position(block_center_position),
                Transform::from_translation(block_center_position.extend(0.0)),
                StateScoped(Screen::Gameplay),
            ));
        }
    }
}
