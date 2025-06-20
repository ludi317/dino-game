use crate::components::{
    AnimationIndices, AnimationTimer, Health, HealthInfo, Player, PlayerCollider, ScoreInfo,
};
use crate::constants::DINO_DIE_SIZE;
use crate::resources::{DinoDie, ScoreOffset};
use crate::systems::player::animation::animate_die;
use bevy::prelude::*;

pub fn check_health(
    health_query: Query<&Health, With<PlayerCollider>>,
    mut player_query: Query<(&mut Sprite, &mut AnimationIndices, &mut AnimationTimer), With<Player>>,
    mut time: ResMut<Time<Virtual>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut dino_die: Res<DinoDie>,
) {
    let Health(health) = health_query.single().unwrap();
    if *health == 0 {
        let (mut sprite, mut anim_indices, mut anim_timer) = player_query.single_mut().unwrap();
        if sprite.custom_size != Some(DINO_DIE_SIZE) {
            // a slow death
            time.set_relative_speed(1.0);

            // switch to dino die animation
            animate_die(&mut dino_die, &mut sprite, &mut anim_indices, &mut anim_timer, &mut texture_atlas_layouts);
        }
    }
}

pub fn render_health_info(
    player_query: Query<&mut Health, With<PlayerCollider>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
) {
    if let Ok(mut health_info) = health_info_query.single_mut() {
        if let Ok(health) = player_query.single() {
            health_info.0 = format!("Health: {}", health.0);
        }
    }
}

pub fn render_score_info(
    time: Res<Time<Virtual>>,
    mut score_info_query: Query<&mut Text, With<ScoreInfo>>,
    offset: Res<ScoreOffset>,
) {
    score_info_query.single_mut().unwrap().0 =
        format!("\nScore: {}", (time.elapsed_secs() - offset.0).floor());
}
