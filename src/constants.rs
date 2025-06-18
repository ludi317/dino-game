use bevy::math::Vec2;

pub const GROUND_LEVEL: f32 = -300.0;
pub const GAME_SPEED: f32 = 500.0;
pub const CAMERA_SPEED: f32 = 3.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const SAND_SIZE_X: f32 = 1920.0;
pub const SAND_SIZE_Y: f32 = 1080.0;

pub const PTERO_SIZE_X: u32 = 862;
pub const PTERO_SIZE_Y: u32 = 970;
const PTERO_SCALE: f32 = 100. / PTERO_SIZE_X as f32;
pub const PTERO_SIZE: Vec2 = Vec2::new(PTERO_SIZE_X as f32 * PTERO_SCALE, PTERO_SIZE_Y as f32 * PTERO_SCALE);
pub const PTERO_TIMER_INTERVAL: f32 = 0.07;
#[cfg(debug_assertions)] // Development mode
pub const INITIAL_HEALTH: usize = 999;

#[cfg(not(debug_assertions))] // Release mode
pub const INITIAL_HEALTH: usize = 99;
