use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct ObstacleSpawningTimer(pub Timer);

#[derive(Resource)]
pub struct RealTimer(pub Timer);

#[derive(Resource)]
pub struct ScoreOffset(pub f32);

/// Set while a run is suspended in the menu, so that (P) resumes instead of restarting.
#[derive(Resource, Default)]
pub struct RunInProgress(pub bool);

#[derive(Resource)]
pub struct GameSettings {
    /// Whether the game gradually gets faster the longer a run lasts.
    pub speed_up_over_time: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            speed_up_over_time: true,
        }
    }
}

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
pub struct DinoDuck(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoJump(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoDie(pub Handle<Image>);