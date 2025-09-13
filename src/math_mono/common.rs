use bevy::prelude::*;

// Movable entities will have a position
#[derive(Component, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Component, Default, Deref, DerefMut)]
pub struct CurrentColor(pub Color);

pub fn get_primary_window_size(window: &Window) -> Vec2 {
    Vec2::new(window.width(), window.height())
}

pub fn clamp_position_to_screen(position: Vec2, size: Vec2, window_size: Vec2) -> Vec2 {
    let half_window = window_size / 2.0;
    let half_size = size / 2.0;

    Vec2::new(
        position.x.clamp(-half_window.x + half_size.x, half_window.x - half_size.x),
        position.y.clamp(-half_window.y + half_size.y, half_window.y - half_size.y),
    )
}
