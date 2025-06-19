use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct CactusRoot;

#[derive(Component)]
pub struct HealthPickup;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct HealthInfo;

#[derive(Component)]
pub struct ScoreInfo;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct CactusArm;

#[derive(Component)]
pub struct CactusCollider;

#[derive(Component)]
pub struct IsHit(pub bool);

#[derive(Component)]
pub struct Pterodactyl;

#[derive(Component)]
pub struct PterodactylCollider;

#[derive(Component)]
pub struct PlayerCollider;

#[derive(Component)]
pub struct Layer {
    pub current_size_x: f32,
    pub speed_scale: f32,
}