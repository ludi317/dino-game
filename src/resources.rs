use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct ObstacleSpawningTimer(pub Timer);

#[derive(Resource, Clone)]
pub struct HealthPickUpImg(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct PterodactylFly(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct PterodactylDie(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct CactusTexture(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoRun(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoDash(pub Handle<Image>);

#[derive(Resource)]
pub struct AnimationState {
    pub current: f32,
    pub speed: f32,
}
