use bevy::prelude::*;
use bevy::{ecs::component::Component, reflect::Reflect};

// Shared components
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub invulnerable_timer: f32, // Prevent rapid damage
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Damage {
    pub amount: i32,
}

// Player components
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub score: i32,
}

#[derive(Component, Reflect, Clone, Copy, PartialEq)]
pub struct GridPosition {
    pub row: i32,
    pub col: i32,
}

// Enemy components
// #[derive(Component)]
// pub struct Enemy {
//     pub speed: f32,
//     pub ai_state: EnemyState,
// }

// #[derive(Component)]
// pub struct EnemySpawner {
//     pub timer: f32,
//     pub spawn_interval: f32,
// }

// Game state components
#[derive(Component)]
pub struct Question {
    pub text: String,
    pub answer: i32,
}

#[derive(Component, Reflect, Clone, Copy, PartialEq)]
pub struct NumberBlock {
    pub value: i32,
    pub is_eaten: bool,
}

// UI components
// #[derive(Component)]
// pub struct HealthBar;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct QuestionDisplay;

// Collision components
// #[derive(Component)]
// pub struct CollisionBox {
//     pub size: Vec2,
// }

// Enums
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum EnemyState {
//     Idle,
//     Chasing,
//     Patrolling,
// }

// impl Default for EnemyState {
//     fn default() -> Self {
//         EnemyState::Idle
//     }
// }

// Default implementations
impl Default for Health {
    fn default() -> Self {
        Self {
            current: 5,
            max: 5,
            invulnerable_timer: 0.0,
        }
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self { amount: 1 }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self { score: 0 }
    }
}

impl Default for GridPosition {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

// impl Default for Enemy {
//     fn default() -> Self {
//         Self {
//             speed: 50.0,
//             ai_state: EnemyState::default(),
//         }
//     }
// }

// impl Default for EnemySpawner {
//     fn default() -> Self {
//         Self {
//             timer: 0.0,
//             spawn_interval: 10.0, // Spawn every 10 seconds
//         }
//     }
// }
