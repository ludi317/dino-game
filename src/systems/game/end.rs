use crate::components::{CactusRoot, GameOverText, Health, HealthPickup, PlayerCollider, Pterodactyl};
use crate::constants::INITIAL_HEALTH;
use crate::states::GameState;
use crate::states::GameState::InGame;
use bevy::color::Color;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::resources::ScoreOffset;

pub fn game_over(mut commands: Commands, mut score_offset: ResMut<ScoreOffset>,
                 mut time: ResMut<Time<Virtual>>) {

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.),
            right: Val::Percent(10.),
            top: Val::Percent(15.),
            bottom: Val::Percent(15.),
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Text(format!("Game Over\n Score: {}", (time.elapsed_secs() - score_offset.0).floor())),
                TextFont::from_font_size(160.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(0.0, 0.0, 1.0)),
                GameOverText,
            ));
        });
    score_offset.0 = time.elapsed_secs();
    time.pause();
}


// New system to restart the game
pub fn restart_game(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    player_query: Query<Entity, With<PlayerCollider>>,
    obstacle_query: Query<Entity, Or<(With<CactusRoot>, With<Pterodactyl>, With<HealthPickup>)>>,
    game_over_text_query: Query<Entity, With<GameOverText>>,
) {
    for e in events.read() {
        if e.state.is_pressed() && e.key_code == KeyCode::Space {
            // Reset game state
            game_state.set(InGame);

            // Reset player health
            if let Ok(player_entity) = player_query.single() {
                commands.entity(player_entity).insert(Health(INITIAL_HEALTH));
            }

            // reset time
            time.set_relative_speed(1.0);
            time.unpause();

            // Despawn all obstacles
            for obstacle_entity in obstacle_query.iter() {
                commands.entity(obstacle_entity).try_despawn();
            }

            // Despawn the "GAME OVER" text
            for text_entity in game_over_text_query.iter() {
                commands.entity(text_entity).try_despawn();
            }
        }
    }
}
