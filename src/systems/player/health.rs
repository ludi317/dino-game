use crate::components::{Health, HealthInfo, PlayerCollider, ScoreInfo};
use crate::states::GameState;
use crate::states::GameState::GameOver;
use bevy::prelude::*;
use crate::resources::ScoreOffset;

pub fn check_health(
    player_query: Query<&Health, With<PlayerCollider>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Health(health)) = player_query.single() {
        if *health == 0 {
            game_state.set(GameOver);
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

pub fn render_score_info(time: Res<Time<Virtual>>,
                         mut score_info_query: Query<&mut Text, With<ScoreInfo>>, offset: Res<ScoreOffset>) {
    score_info_query.single_mut().unwrap().0 = format!("\nScore: {}", (time.elapsed_secs() - offset.0).floor());
}