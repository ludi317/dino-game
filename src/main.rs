use crate::GameState::{GameOver, InGame};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
use rand_core::RngCore;

//region Constants
const GAME_SPEED: f32 = 400.0;
const JUMP_FORCE: f32 = 1000.0;
const GRAVITY: f32 = -4000.0;
const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const SPAWN_INTERVAL: f32 = 1.0;
const GROUND_LEVEL: f32 = -100.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
//endregion

//region Components, resources, and states
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct HealthInfo;

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    GameOver,
}
//endregion

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .insert_resource(ObstacleSpawningTimer(
            Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_state(InGame)
        .add_systems(Update, (jump, apply_gravity, player_movement)
            .run_if(in_state(InGame)))
        .add_systems(Update, (spawn_obstacles, move_obstacles, detect_collision, render_health_info, check_health)
            .run_if(in_state(InGame)))
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver))) // New system to restart the game
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let initial_health = 3;
    // Player
    commands
        .spawn((
            Player,
            Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
            Velocity(Vec3::ZERO),
            Health(initial_health)
        ));

    commands.spawn((
        HealthInfo,
        Text::new(format!("Health: {}", initial_health))
    )
    );

    // Ground
    commands.spawn((
        Sprite {
            color: GROUND_COLOR,
            custom_size: Some(GROUND_SIZE),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-GROUND_EDGE, GROUND_LEVEL, 0.0)
    ));
}

fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
            if e.state.is_pressed() && e.key_code == KeyCode::Space && transform.translation.y <= GROUND_LEVEL {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut rng: GlobalEntropy<WyRand>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL + (rng.next_u32() % 50) as f32;
        commands.spawn((
            Obstacle,
            Sprite {
                color: OBSTACLE_COLOR,
                custom_size: Some(OBSTACLE_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}

fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        // Remove obstacles once they're off-screen
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform.translation.distance(obstacle_transform.translation) < 50.0;
            if collision {
                health.0 -= 1;
                commands.entity(entity).despawn(); // Remove obstacle
            }
        }
    }
}

fn check_health(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}

fn game_over(mut commands: Commands) {
    commands.spawn((Node {
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
                Text("GAME OVER".to_string()),
                TextFont::from_font_size(160.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                GameOverText,
            ));
        });
}

fn render_health_info(
    player_query: Query<&mut Health, With<Player>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
) {
    if let Ok(mut health_info) = health_info_query.get_single_mut() {
        if let Ok(health) = player_query.get_single() {
            health_info.0 = format!("Health: {}", health.0);
        }
    }
}

// New system to restart the game
fn restart_game(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
    game_over_text_query: Query<Entity, With<GameOverText>>,
) {
    for e in events.read() {
        if e.state.is_pressed() && e.key_code == KeyCode::Space {
            // Reset game state
            game_state.set(InGame);

            // Reset player health
            if let Ok(player_entity) = player_query.get_single() {
                commands.entity(player_entity).insert(Health(3));
            }

            // Update health info text
            if let Ok(mut health_info) = health_info_query.get_single_mut() {
                health_info.0 = "Health: 3".to_string();
            }

            // Despawn all obstacles
            for obstacle_entity in obstacle_query.iter() {
                commands.entity(obstacle_entity).despawn();
            }

            // Despawn the "GAME OVER" text
            for text_entity in game_over_text_query.iter() {
                commands.entity(text_entity).despawn();
            }

        }
    }
}