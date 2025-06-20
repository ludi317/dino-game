use crate::components::{
    AnimationIndices, AnimationTimer, Health, HealthInfo, Player, PlayerCollider, ScoreInfo,
};
use crate::constants::{
    DIE_ANIMATION_TIMER_INTERVAL, DINO_DIE_IMG_SIZE_X, DINO_DIE_IMG_SIZE_Y, DINO_DIE_SIZE
};
use crate::resources::{DinoDie, ScoreOffset};
use bevy::prelude::*;
use std::time::Duration;

pub fn check_health(
    health_query: Query<&Health, With<PlayerCollider>>,
    mut player_query: Query<(&mut Sprite, &mut AnimationIndices, &mut AnimationTimer), With<Player>>,
    mut time: ResMut<Time<Virtual>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    dino_die: Res<DinoDie>,
) {
    let Health(health) = health_query.single().unwrap();
    if *health == 0 {
        let (mut sprite, mut anim_indices, mut anim_timer) = player_query.single_mut().unwrap();
        if sprite.custom_size != Some(DINO_DIE_SIZE) {
            // a slow death
            time.set_relative_speed(1.0);

            // switch to dino die animation
            let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_DIE_IMG_SIZE_X, DINO_DIE_IMG_SIZE_Y), 4, 2, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            sprite.image = dino_die.0.clone();
            sprite.custom_size = Some(DINO_DIE_SIZE);
            sprite.texture_atlas = Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            });
            anim_indices.first = 0;
            anim_indices.last = 7;
            anim_timer.0.set_duration(Duration::from_secs_f32(DIE_ANIMATION_TIMER_INTERVAL));
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
