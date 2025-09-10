use bevy::{prelude::*};

use crate::{
    demo::{
        grid::{calculate_block_center, calculate_grid_layout, GridConfig},
        player::{GridPosition, Player},
    }, AppSystems, PausableSystems
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();

    app.add_systems(
        Update,
        (apply_movement)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
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

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        grid_pos.row += 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        grid_pos.row -= 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        grid_pos.col -= 1;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        grid_pos.col += 1;
        moved = true;
    }

    if moved {
        // Clamp the position to stay within the grid bounds
        grid_pos.row = grid_pos.row.clamp(0, config.rows - 1);
        grid_pos.col = grid_pos.col.clamp(0, config.cols - 1);
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
    let (_total_size, bottom_left) = calculate_grid_layout(&config);
    let new_world_pos = calculate_block_center(&config, bottom_left, grid_pos.row, grid_pos.col);

    // Update the player's actual world position.
    // We give it a higher Z value to make sure it renders on top of the grid.
    transform.translation = new_world_pos.extend(1.0);
}
