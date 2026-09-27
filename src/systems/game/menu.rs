use crate::components::{
    DinoColorSetting, HealthInfo, MenuRoot, ScoreInfo, SettingsRoot, SpeedUpSetting,
};
use crate::resources::{GameSettings, ObstacleSpawningTimer, RealTimer, RunInProgress, ScoreOffset};
use crate::states::GameState;
use bevy::prelude::*;

const OVERLAY_COLOR: Color = Color::srgba(0.04, 0.04, 0.09, 0.93);
const TITLE_COLOR: Color = Color::srgb(1.0, 0.84, 0.35);
const ITEM_COLOR: Color = Color::srgb(0.92, 0.92, 0.96);
const HINT_COLOR: Color = Color::srgb(0.55, 0.6, 0.68);
const ON_COLOR: Color = Color::srgb(0.4, 0.88, 0.5);
const OFF_COLOR: Color = Color::srgb(0.9, 0.46, 0.46);

/// Full screen panel that covers the game world while a menu is open.
fn overlay() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        row_gap: Val::Px(18.),
        ..default()
    }
}

pub fn spawn_menu(
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    run: Res<RunInProgress>,
) {
    time.pause();

    commands
        .spawn((MenuRoot, overlay(), BackgroundColor(OVERLAY_COLOR)))
        .with_children(|menu| {
            menu.spawn((
                Text::new("DINO RUNNER"),
                TextFont::from_font_size(72.0),
                TextColor(TITLE_COLOR),
            ));
            menu.spawn((
                Text::new("Jump with Space or Up, duck with Down\nPress O during a run to come back here"),
                TextFont::from_font_size(20.0),
                TextColor(HINT_COLOR),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect::bottom(Val::Px(30.)),
                    ..default()
                },
            ));
            menu.spawn((
                Text::new(if run.0 { "(P)  Resume" } else { "(P)  Play" }),
                TextFont::from_font_size(36.0),
                TextColor(ITEM_COLOR),
            ));
            menu.spawn((
                Text::new("(S)  Settings"),
                TextFont::from_font_size(36.0),
                TextColor(ITEM_COLOR),
            ));
        });
}

pub fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::InGame);
    } else if keys.just_pressed(KeyCode::KeyS) {
        next_state.set(GameState::Settings);
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_settings(mut commands: Commands, settings: Res<GameSettings>) {
    commands
        .spawn((SettingsRoot, overlay(), BackgroundColor(OVERLAY_COLOR)))
        .with_children(|menu| {
            menu.spawn((
                Text::new("SETTINGS"),
                TextFont::from_font_size(56.0),
                TextColor(TITLE_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(30.)),
                    ..default()
                },
            ));
            menu.spawn((
                SpeedUpSetting,
                Text::new(speed_up_label(&settings)),
                TextFont::from_font_size(32.0),
                TextColor(speed_up_color(&settings)),
            ));
            menu.spawn((
                DinoColorSetting,
                Text::new(dino_color_label(&settings)),
                TextFont::from_font_size(32.0),
                TextColor(settings.dino_color.swatch()),
            ));
            menu.spawn((
                Text::new("(Esc)  Back"),
                TextFont::from_font_size(24.0),
                TextColor(HINT_COLOR),
                Node {
                    margin: UiRect::top(Val::Px(30.)),
                    ..default()
                },
            ));
        });
}

pub fn settings_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut speed_up_query: Query<(&mut Text, &mut TextColor), With<SpeedUpSetting>>,
    mut dino_color_query: Query<
        (&mut Text, &mut TextColor),
        (With<DinoColorSetting>, Without<SpeedUpSetting>),
    >,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        settings.speed_up_over_time = !settings.speed_up_over_time;

        if let Ok((mut label, mut color)) = speed_up_query.single_mut() {
            label.0 = speed_up_label(&settings);
            color.0 = speed_up_color(&settings);
        }
    }

    if keys.just_pressed(KeyCode::KeyC) {
        settings.dino_color = settings.dino_color.next();

        if let Ok((mut label, mut color)) = dino_color_query.single_mut() {
            label.0 = dino_color_label(&settings);
            color.0 = settings.dino_color.swatch();
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

pub fn despawn_settings(mut commands: Commands, query: Query<Entity, With<SettingsRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn speed_up_label(settings: &GameSettings) -> String {
    format!(
        "(T)  Speed up over time:  {}",
        if settings.speed_up_over_time {
            "ON"
        } else {
            "OFF"
        }
    )
}

fn dino_color_label(settings: &GameSettings) -> String {
    format!("(C)  Dino color:  {}", settings.dino_color.label())
}

fn speed_up_color(settings: &GameSettings) -> Color {
    if settings.speed_up_over_time {
        ON_COLOR
    } else {
        OFF_COLOR
    }
}

/// Sends the player back to the landing page, leaving the run frozen where it is.
pub fn open_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}

/// Resumes a suspended run, or starts a fresh one: the clock, the score and the
/// spawn timers only restart when no run is in progress.
pub fn start_run(
    mut time: ResMut<Time<Virtual>>,
    mut score_offset: ResMut<ScoreOffset>,
    mut spawning_timer: ResMut<ObstacleSpawningTimer>,
    mut real_timer: ResMut<RealTimer>,
    mut run: ResMut<RunInProgress>,
) {
    if !run.0 {
        time.set_relative_speed(1.0);
        score_offset.0 = time.elapsed_secs();
        spawning_timer.0.reset();
        real_timer.0.reset();
        run.0 = true;
    }

    time.unpause();
}

/// The run is over, so the next visit to the landing page offers a fresh start.
pub fn end_run(mut run: ResMut<RunInProgress>) {
    run.0 = false;
}

pub fn show_hud(mut query: Query<&mut Visibility, Or<(With<HealthInfo>, With<ScoreInfo>)>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Inherited;
    }
}

pub fn hide_hud(mut query: Query<&mut Visibility, Or<(With<HealthInfo>, With<ScoreInfo>)>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
