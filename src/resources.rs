use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct ObstacleSpawningTimer(pub(crate) Timer);

#[derive(Resource, Clone)]
pub struct Cheeseburger{
  pub image: Option<Handle<Image>>
}

#[derive(Resource, Clone)]
pub struct CactusTexture{
    pub image: Option<Handle<Image>>
}
