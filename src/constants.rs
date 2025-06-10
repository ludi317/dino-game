use bevy::color::Color;
use bevy::math::Vec2;

pub const GAME_SPEED: f32 = 400.0;
pub const JUMP_FORCE: f32 = 1800.0;
pub const GRAVITY: f32 = -4000.0;
pub const PLAYER_X: f32 = -300.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(87.0, 94.0);
pub const SPAWN_INTERVAL: f32 = 1.5;
pub const GROUND_LEVEL: f32 = -300.0;
pub const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
pub const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
pub const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const OBSTACLE_SIZE: Vec2 = Vec2::new(80.0, 100.0);
pub const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
pub const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub const HEALTH_PICKUP_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
pub const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3;
pub const INITIAL_HEALTH: usize = 99;