//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
pub mod common;
pub mod grid;
pub mod level;
mod movement;
pub mod player;
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level::plugin,
            animation::plugin,
            movement::plugin,
            player::plugin,
            grid::GridPlugin,
        ));
    }
}
