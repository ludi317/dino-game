use bevy::math::Vec2;

const PLAYER_SCALE: f32 = 0.15;
const HEALTH_SCALE: f32 = 0.5;

pub const PLAYER_SIZE: Vec2 = Vec2::new(1328.0 * PLAYER_SCALE, 768.0 * PLAYER_SCALE);
pub const GROUND_LEVEL: f32 = -320.0;
pub const INITIAL_HEALTH: usize = 99;
pub const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(77.0 * HEALTH_SCALE, 70.0 * HEALTH_SCALE);
pub const GAME_SPEED: f32 = 400.0;