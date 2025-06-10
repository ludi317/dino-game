use bevy::color::Color;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::components::PauseText;
use crate::states::GameState;
use crate::states::GameState::InGame;

pub fn toggle_pause(
    mut events: EventReader<KeyboardInput>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for e in events.read() {
        if e.state.is_pressed() && e.key_code == KeyCode::KeyP {
            match game_state.get() {
                InGame => next_state.set(GameState::Paused),
                GameState::Paused => next_state.set(InGame),
                _ => {} // Don't toggle pause from other states
            }
        }
    }
}

pub fn show_pause_text(mut commands: Commands) {
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
                Text("You have paused the game".to_string()),
                TextFont::from_font_size(16.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(0.0, 0.5, 0.5)),
                PauseText,
            ));
        });
}

pub fn hide_pause_text(mut commands: Commands, query: Query<Entity, With<PauseText>>) {
    if let Ok((entity)) = query.get_single() {
       commands.entity(entity).despawn();
    }
}


