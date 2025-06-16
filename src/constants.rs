pub const GROUND_LEVEL: f32 = -250.0;
pub const GAME_SPEED: f32 = 500.0;
pub const CAMERA_SPEED: f32 = 3.0;

#[cfg(debug_assertions)] // Development mode
pub const INITIAL_HEALTH: usize = 999;

#[cfg(not(debug_assertions))] // Release mode
pub const INITIAL_HEALTH: usize = 99;