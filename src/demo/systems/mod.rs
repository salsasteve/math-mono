pub mod movement;

use bevy::app::{App, Plugin, Update};
pub use movement::*;


pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, _app: &mut App) {
    }
}
