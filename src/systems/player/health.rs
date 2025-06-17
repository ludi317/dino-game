use crate::components::{Health, HealthInfo, PlayerCollider};
use crate::states::GameState;
use crate::states::GameState::GameOver;
use bevy::prelude::*;

pub fn check_health(
    player_query: Query<&Health, With<PlayerCollider>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}

pub fn render_health_info(
    player_query: Query<&mut Health, With<PlayerCollider>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
) {
    if let Ok(mut health_info) = health_info_query.get_single_mut() {
        if let Ok(health) = player_query.get_single() {
            health_info.0 = format!("Health: {}", health.0);
        }
    }
}