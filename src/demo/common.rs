use bevy::prelude::*;

// Movable entities will have a position
#[derive(Component, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

// Entities with a shape have a size
#[derive(Component, Default, Deref, DerefMut)]
pub struct Shape(pub Vec2);

#[derive(Component, Default, Deref, DerefMut)]
pub struct CurrentColor(pub Color);

pub fn get_primary_window_size(windows: Query<&Window>) -> Option<Vec2> {
    let window = windows.single().ok()?;
    Some(Vec2::new(window.width(), window.height()))
}
