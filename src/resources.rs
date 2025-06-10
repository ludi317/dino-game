use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct ObstacleSpawningTimer(pub(crate) Timer);
