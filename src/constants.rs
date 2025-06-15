use bevy::math::Vec2;

const PLAYER_SCALE: f32 = 0.15;
const HEALTH_SCALE: f32 = 0.5;

pub const PLAYER_SIZE: Vec2 = Vec2::new(1328.0 * PLAYER_SCALE, 768.0 * PLAYER_SCALE);
pub const GROUND_LEVEL: f32 = -250.0;
pub const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(77.0 * HEALTH_SCALE, 70.0 * HEALTH_SCALE);
pub const GAME_SPEED: f32 = 500.0;

#[cfg(debug_assertions)] // Development mode
pub const INITIAL_HEALTH: usize = 999;

#[cfg(not(debug_assertions))] // Release mode
pub const INITIAL_HEALTH: usize = 99;