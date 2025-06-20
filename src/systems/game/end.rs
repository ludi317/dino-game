use crate::components::{AnimationIndices, AnimationTimer, CactusRoot, GameOverText, Health, HealthPickup, Player, PlayerCollider, Pterodactyl};
use crate::constants::{DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y, DINO_RUN_SIZE, INITIAL_HEALTH, RUN_ANIMATION_TIMER_INTERVAL};
use crate::resources::{DinoRun, ScoreOffset};
use crate::states::GameState;
use crate::states::GameState::InGame;
use bevy::color::Color;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use std::time::Duration;

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
                Text(format!("Game Over. Score: {}", (time.elapsed_secs() - score_offset.0).floor())),
                TextFont::from_font_size(48.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(0.0, 0.5, 0.5)),
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
    mut player_query: Query<(&mut Sprite, &mut AnimationIndices, &mut AnimationTimer), With<Player>>,
    mut health_query: Query<&mut Health, With<PlayerCollider>>,
    obstacle_query: Query<Entity, Or<(With<CactusRoot>, With<Pterodactyl>, With<HealthPickup>)>>,
    game_over_text_query: Query<Entity, With<GameOverText>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    dino_run: Res<DinoRun>,
) {
    for e in events.read() {
        if e.state.is_pressed() && e.key_code == KeyCode::Space {
            // Reset player health
            let mut health = health_query.single_mut().unwrap();
            health.0 = INITIAL_HEALTH;

            // dino run animation
            let (mut sprite, mut anim_indices, mut anim_timer) = player_query.single_mut().unwrap();
            let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y), 4, 4, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            sprite.image = dino_run.0.clone();
            sprite.custom_size = Some(DINO_RUN_SIZE);
            sprite.texture_atlas = Some(TextureAtlas { layout: texture_atlas_layout, index: 0});
            anim_indices.last = 15;
            anim_timer.0.set_duration(Duration::from_secs_f32(RUN_ANIMATION_TIMER_INTERVAL));
        }

        // Despawn all obstacles
        for obstacle_entity in obstacle_query.iter() {
            commands.entity(obstacle_entity).try_despawn();
        }

        // Despawn the "GAME OVER" text
        for text_entity in game_over_text_query.iter() {
            commands.entity(text_entity).try_despawn();
        }

        // Reset game state
        game_state.set(InGame);

        // reset time
        time.unpause();
    }
}
