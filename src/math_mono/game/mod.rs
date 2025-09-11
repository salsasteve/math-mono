pub mod enemies;
pub mod grid;
pub mod health;
pub mod math;
pub mod player;

use bevy::app::{App, Plugin};
// pub use enemies::*;
pub use grid::*;
// pub use health::*;
// pub use math::*;
pub use player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GridPlugin, PlayerPlugin));
    }
}
