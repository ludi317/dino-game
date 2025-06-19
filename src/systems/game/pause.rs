use crate::components::PauseText;
use bevy::color::Color;
use bevy::prelude::*;

pub fn toggle_pause(mut time: ResMut<Time<Virtual>>, commands: Commands, query: Query<Entity, With<PauseText>>) {
    if time.is_paused() {
        time.unpause();
        hide_pause_text(commands, query);
    } else {
        time.pause();
        show_pause_text(commands);
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
    if let Ok(entity) = query.single() {
       commands.entity(entity).despawn();
    }
}


