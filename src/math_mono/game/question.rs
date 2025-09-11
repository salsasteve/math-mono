//! Question-specific behavior.

use bevy::{prelude::*, sprite::Anchor, text::TextBounds};

use crate::{math_mono::{common::get_primary_window_size, components::{Question, QuestionDisplay}, game::{calculate_grid_layout, GridConfig}}, screens::Screen};

pub struct QuestionPlugin;

impl Plugin for QuestionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_question)
            .add_systems(OnEnter(Screen::Gameplay), spawn_question);
    }
}

pub fn update_question(mut question_query: Query<&mut Question>) {
    for mut question in question_query.iter_mut() {
        question.text = "Hello, world!".to_string();
    }
}

#[derive(Component)]
struct AnimateScale;

pub fn spawn_question(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GridConfig>,
    windows: Query<&Window>,
) {
    let font = asset_server.load("fonts/MonofurNerdFont-Bold.ttf");
    let window_size = get_primary_window_size(windows.single().unwrap());

    // Calculate grid layout to determine available space
    let (total_width, total_height, bottom_left_x, bottom_left_y) = calculate_grid_layout(&config);
    println!("total_width: {}", total_width);
    println!("total_height: {}", total_height);
    println!("bottom_left_x: {}", bottom_left_x);
    println!("bottom_left_y: {}", bottom_left_y);


    let unplayable_margin = calculate_unplayable_margin(window_size.x, total_width);
    println!("unplayable_margin: {}", unplayable_margin);
    let usable_percentage = 0.01;
    let box_size = calculate_text_box_size(unplayable_margin, total_height, usable_percentage);
    let box_position = calculate_text_box_position(bottom_left_x, unplayable_margin);




    let question_text = "What is 10 + 10 ?";

    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn((
            Sprite::from_color(Color::BLACK, box_size),
            Transform::from_translation(box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(question_text),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // Ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
                // anchor
                Anchor::Center,
            ));
        });
}

fn calculate_unplayable_margin(totol_window_width: f32, total_grid_width: f32) -> f32 {
    // These margins area assumed to be the same on both sides
    // This is used to calculate the unplayable margin on the left and right sides
    (totol_window_width - total_grid_width) / 2.0
}

fn calculate_text_box_size(unplayable_margin: f32, total_height: f32, usable_percentage: f32) -> Vec2 {
    // margin_percentage is the percentage of the unplayable margin to use for the text box
    // 1.0 means use the entire unplayable margin
    // 0.5 means use half of the unplayable margin
    // 0.0 means use no unplayable margin
    let margin = unplayable_margin * usable_percentage;
    let width = unplayable_margin - margin;
    let height = total_height / 2.0;

    Vec2::new(width, height)
}

fn calculate_text_box_position(bottom_left_x: f32, unplayable_margin: f32) -> Vec2 {
    // This is used to calculate the position of the text box
    // y is at the center of the grid
    // in bevy 2d game 0,0 is the center of the screen
    // https://bevy-cheatbook.github.io/fundamentals/coords.html
    let left_edge_x = bottom_left_x - unplayable_margin;
    let offset = unplayable_margin / 2.0; // half of the unplayable margin

    Vec2::new(left_edge_x + offset, 0.0)
}




