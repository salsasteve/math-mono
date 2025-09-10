pub mod grid;
pub mod player;
pub mod math;
pub mod health;
pub mod enemies;

use bevy::app::{App, Plugin};
pub use grid::*;
pub use player::*;
pub use math::*;
pub use health::*;
pub use enemies::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GridPlugin, PlayerPlugin));
    }
}
