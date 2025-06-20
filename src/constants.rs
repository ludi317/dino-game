use bevy::math::Vec2;

pub const GROUND_LEVEL: f32 = -330.0;
pub const GAME_SPEED: f32 = 500.0;
pub const WINDOW_WIDTH: f32 = 1280.0;

pub const PTERO_SIZE_X: u32 = 862;
pub const PTERO_SIZE_Y: u32 = 970;
const PTERO_SCALE: f32 = 100. / PTERO_SIZE_X as f32;
pub const PTERO_SIZE: Vec2 = Vec2::new(PTERO_SIZE_X as f32 * PTERO_SCALE, PTERO_SIZE_Y as f32 * PTERO_SCALE);
pub const RUN_ANIMATION_TIMER_INTERVAL: f32 = 0.07;

pub const DINO_RUN_IMG_SIZE_X: u32 = 939;
pub const DINO_RUN_IMG_SIZE_Y: u32 = 668;
const DINO_RUN_SCALE: f32 = 200./ DINO_RUN_IMG_SIZE_X as f32;
pub const DINO_RUN_SIZE: Vec2 = Vec2::new(DINO_RUN_IMG_SIZE_X as f32 * DINO_RUN_SCALE, DINO_RUN_IMG_SIZE_Y as f32 * DINO_RUN_SCALE);
pub const HIT_BOX_SCALE_X: f32 = 0.67;

pub const DINO_DASH_IMG_SIZE_X: u32 = 902;
pub const DINO_DASH_IMG_SIZE_Y: u32 = 571;
const DINO_DASH_SCALE: f32 = 200. / DINO_DASH_IMG_SIZE_X as f32;
pub const DINO_DASH_SIZE: Vec2 = Vec2::new(DINO_DASH_IMG_SIZE_X as f32 * DINO_DASH_SCALE, DINO_DASH_IMG_SIZE_Y as f32 * DINO_DASH_SCALE);
pub const HIT_BOX_SCALE_Y: f32 = 0.75;

pub const DINO_JUMP_IMG_SIZE_X: u32 = 862;
pub const DINO_JUMP_IMG_SIZE_Y: u32 = 866;
const DINO_JUMP_SCALE: f32 = 200. / DINO_JUMP_IMG_SIZE_X as f32;
pub const DINO_JUMP_SIZE: Vec2 = Vec2::new(DINO_JUMP_IMG_SIZE_X as f32 * DINO_JUMP_SCALE, DINO_JUMP_IMG_SIZE_Y as f32 * DINO_JUMP_SCALE);
pub const JUMP_ANIMATION_TIMER_INTERVAL: f32 = 0.11;

pub const DINO_DIE_IMG_SIZE_X: u32 = 871;
pub const DINO_DIE_IMG_SIZE_Y: u32 = 689;
const DINO_DIE_SCALE: f32 = 200. / DINO_DIE_IMG_SIZE_X as f32;
pub const DINO_DIE_SIZE: Vec2 = Vec2::new(DINO_DIE_IMG_SIZE_X as f32 * DINO_DIE_SCALE, DINO_DIE_IMG_SIZE_Y as f32 * DINO_DIE_SCALE);
pub const DIE_ANIMATION_TIMER_INTERVAL: f32 = 0.13;

#[cfg(debug_assertions)] // Development mode
pub const INITIAL_HEALTH: usize = 1;

#[cfg(not(debug_assertions))] // Release mode
pub const INITIAL_HEALTH: usize = 1;

