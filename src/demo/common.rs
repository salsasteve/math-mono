use bevy::prelude::*;

// Movable entities will have a position
#[derive(Component, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

// Entities with a shape have a size
#[derive(Component, Default, Deref, DerefMut)]
pub struct Shape(pub Vec2);

#[derive(Component, Default, Deref, DerefMut)]
pub struct CurrentColor(pub Color);
