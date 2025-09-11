use bevy::prelude::*;

mod animation;
pub mod common;
pub mod components;
pub mod game;
pub mod level;
pub struct MathMonoPlugin;

impl Plugin for MathMonoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((level::plugin, animation::AnimationPlugin, game::GamePlugin));
    }
}
