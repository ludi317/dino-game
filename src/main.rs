mod cactus;

use crate::cactus::generate_cactus;
use crate::GameState::{GameOver, InGame};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
use rand_core::RngCore;
use rand::Rng;

//region Constants
const GAME_SPEED: f32 = 400.0;
const JUMP_FORCE: f32 = 2000.0;
const GRAVITY: f32 = -4000.0;
const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const SPAWN_INTERVAL: f32 = 0.5;
const GROUND_LEVEL: f32 = -200.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(80.0, 100.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const HEALTH_PICKUP_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.0; // 30% chance to spawn instead of obstacle
const INITIAL_HEALTH: usize = 99;
#[derive(Component)]
struct HealthPickup;
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
struct PauseText;

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct HealthInfo;

#[derive(Component)]
struct OriginalSize(Vec2);

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    Paused,
    GameOver,
}
//endregion

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    main();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_state(InGame)
        .add_systems(
            Update,
            (jump, apply_gravity, player_movement, crouch).run_if(in_state(InGame)),
        )
        .add_systems(Update, toggle_pause)
        .add_systems(OnEnter(GameState::Paused), show_pause_text)
        .add_systems(OnExit(GameState::Paused), hide_pause_text)
        .add_systems(
            Update,
            (
                spawn_obstacles,
                move_obstacles,
                detect_collision,
                render_health_info,
                check_health,
            )
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver))) // New system to restart the game
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(PLAYER_SIZE),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
        Health(INITIAL_HEALTH),
        OriginalSize(PLAYER_SIZE),
    ));

    commands.spawn((HealthInfo, Text::new(format!("Health: {}", INITIAL_HEALTH))));

    // Ground
    commands.spawn((
        Sprite {
            color: GROUND_COLOR,
            custom_size: Some(GROUND_SIZE),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-GROUND_EDGE, GROUND_LEVEL, 0.0),
    ));
}

fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
            if e.state.is_pressed()
                && (e.key_code == KeyCode::Space || e.key_code == KeyCode::ArrowUp)
                && transform.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
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
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL;

        // Randomly decide whether to spawn obstacle or health pickup
        if rng.next_u32() % 100 < (HEALTH_PICKUP_SPAWN_CHANCE * 100.0) as u32 {
            // Spawn health pickup
            commands.spawn((
                HealthPickup,
                Sprite {
                    color: HEALTH_PICKUP_COLOR,
                    custom_size: Some(HEALTH_PICKUP_SIZE),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            ));
        } else {
            // let cactus_image = generate_cactus(128, 100, &mut rng);
            // let handle = images.add(cactus_image);
            // Spawn obstacle
            // commands.spawn((
            //     Obstacle,
            //     Sprite {
            //         image: handle,
            //         anchor: Anchor::BottomCenter,
            //         ..default()
            //     },
            //     Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            // ));
            spawn_cactus(
                commands,
                meshes,
                materials,
                Vec2::new(obstacle_x, obstacle_y),
                &mut rng,
            );
            // commands.spawn((
            //     Obstacle,
            //     Mesh2d(meshes.add(Rectangle::default())),
            //     MeshMaterial2d(materials.add(Color::from(PURPLE))),
            //     Transform::default().with_scale(Vec3::splat(128.)),
            // ));
        }
    }
}

fn toggle_pause(
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

fn show_pause_text(mut commands: Commands) {
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

fn hide_pause_text(mut commands: Commands, query: Query<Entity, With<PauseText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_cactus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    rng: &mut GlobalEntropy<WyRand>,
) {
    let green = Color::srgb(0.0, 0.5, 0.1);
    let black = Color::BLACK;
    let blue = Color::srgb(0.0, 0.0, 1.0);
    let orange = Color::srgb(1.0, 0.5, 0.0);

    // Create the main cactus parent first
    let cactus_entity = commands.spawn((
        Obstacle,
        SpatialBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
    )).id();

    let max_trunk_width = 20.0;
    let min_trunk_width = 13.0;
    let trunk_width = rng.gen_range(min_trunk_width..=max_trunk_width);
    let trunk_height = rng.gen_range(38..58) as f32;
    // Add components to main cactus
    commands.entity(cactus_entity).with_children(|parent| {
        // Main trunk
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(trunk_width, trunk_height)).into()),
            MeshMaterial2d(materials.add(green)),
            Transform::from_xyz(0.0, trunk_height/2.0, 0.0),
            Collider { size: Vec2::new(trunk_width, trunk_height) },
        ));

        // Circle top
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(trunk_width/2.0)).into()),
            MeshMaterial2d(materials.add(green)),
            Transform::from_xyz(0.0, trunk_height, 0.0),
        ));
    });

    let arm_length = trunk_width / 2.0;
    let min_arm_width = 9.0;
    let max_arm_width = 20.0;

    let scale = (trunk_width - min_trunk_width) / (max_trunk_width - min_trunk_width);
    let arm_width = min_arm_width + scale * (max_arm_width - min_arm_width);
    let curve_radius = arm_length;
    let arm_height = rng.gen_range(10.0..=30.0);
    let rect_length = (curve_radius * ((rng.next_u32() % 3 + 1) as f32)).min(trunk_height - arm_height);

    // Create and attach the right arm
    commands.entity(cactus_entity).with_children(|parent| {
        parent.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(10.0, arm_height, 0.0),

                ..default()

            },
        )).with_children(|arm| {
            // Horizontal part
            arm.spawn((
                Mesh2d(meshes.add(Rectangle::new(arm_width, arm_length)).into()),
                MeshMaterial2d(materials.add(green)),
                // Transform::IDENTITY,
                // Collider { size: Vec2::new(arm_width, arm_length) },
            ));

            // Curved segment
            arm.spawn((
                Mesh2d(meshes.add(CircularSector::new(curve_radius, std::f32::consts::PI / 2.0)).into()),
                MeshMaterial2d(materials.add(green)),
                Transform::IDENTITY.with_translation(Vec3::new(arm_width -curve_radius, arm_length/2.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(5.0*std::f32::consts::PI/4.0 )),
            ));

            // Vertical capsule
            arm.spawn((
                Mesh2d(meshes.add(Capsule2d::new(curve_radius/2.0, rect_length)).into()),
                MeshMaterial2d(materials.add(green)),
                Transform::IDENTITY.with_translation(Vec3::new(arm_width -curve_radius/2.0, arm_length +curve_radius/2.0, 0.0)),
            ));
        });
    });

    let scale = (trunk_width - min_trunk_width) / (max_trunk_width - min_trunk_width);
    let arm_width = min_arm_width + scale * (max_arm_width - min_arm_width);
    let arm_length = trunk_width / 2.0;
    let curve_radius = arm_length;
    // randomly generate the length of the arm
    let arm_height = rng.gen_range(10.0..=30.0);
    let rect_length = (curve_radius * (rng.next_u32() % 3 + 1) as f32).min(trunk_height - arm_height);


    // Create and attach the left arm (mirror of right arm)
    commands.entity(cactus_entity).with_children(|parent| {
        parent.spawn((
            Transform::from_xyz(-10.0, arm_height, 0.0), // Negative X for left side
            Visibility::Visible,

        )).with_children(|arm| {
            // Horizontal part (mirrored)
            arm.spawn((
                Mesh2d(meshes.add(Rectangle::new(arm_width, arm_length)).into()),
                MeshMaterial2d(materials.add(green)),
                Transform::IDENTITY,
            ));

            // Curved segment (mirrored)
            arm.spawn((
                Mesh2d(meshes.add(CircularSector::new(curve_radius, std::f32::consts::PI / 2.0)).into()),
                MeshMaterial2d(materials.add(green)),
                Transform::IDENTITY
                    .with_translation(Vec3::new(-(arm_width - curve_radius), arm_length/2.0, 0.0)) // Negative X
                    .with_rotation(Quat::from_rotation_z(3.0*std::f32::consts::PI/4.0)), // Different rotation for mirror
            ));

            // Vertical capsule
            arm.spawn((
                Mesh2d(meshes.add(Capsule2d::new(curve_radius/2.0, rect_length)).into()),
                MeshMaterial2d(materials.add(green)),
                Transform::IDENTITY
                    .with_translation(Vec3::new(-(arm_width - curve_radius/2.0), arm_length + curve_radius/2.0, 0.0)),
            ));
        });
    });
}

fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut transforms: ParamSet<(
        Query<(Entity, &mut Transform), With<Obstacle>>,
        Query<(Entity, &mut Transform), With<HealthPickup>>,
    )>,
) {
    // Move obstacles
    for (entity, mut transform) in transforms.p0().iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Move health pickups
    for (entity, mut transform) in transforms.p1().iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Transform, &Children), With<Obstacle>>,
    collider_query: Query<(&Transform, &Collider)>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        let player_size = PLAYER_SIZE;
        let player_half = player_size / 2.0;

        for (entity, obstacle_transform, children) in obstacle_query.iter() {
            for &child in children.iter() {
                if let Ok((child_transform, collider)) = collider_query.get(child) {
                    // Combine parent and child transforms
                    let global_transform = obstacle_transform.mul_transform(*child_transform);

                    if is_colliding(
                        player_transform.translation,
                        player_half,
                        global_transform.translation,
                        collider.size / 2.0,
                    ) {
                        health.0 = health.0.saturating_sub(1);
                        commands.entity(entity).despawn_recursive();
                        break; // No need to check other parts
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

// Helper function for collision detection
fn is_colliding(pos1: Vec3, half_size1: Vec2, pos2: Vec3, half_size2: Vec2) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() <= (half_size1.x + half_size2.x);
    let collision_y = (pos1.y - pos2.y).abs() <= (half_size1.y + half_size2.y);
    collision_x && collision_y
}
fn check_health(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}

fn crouch(
    mut events: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut Sprite, &OriginalSize), With<Player>>,
) {
    for e in events.read() {
        if let Ok((mut sprite, original_size)) = player_query.get_single_mut() {
            if e.state.is_pressed() && e.key_code == KeyCode::ArrowDown {
                // Reduce the player's height to half its original size
                let new_height = original_size.0.y / 2.0;
                if let Some(size) = sprite.custom_size {
                    if size.y > new_height {
                        sprite.custom_size = Some(Vec2::new(size.x, new_height));
                    }
                }
            } else if e.state == ButtonState::Released && e.key_code == KeyCode::ArrowDown {
                sprite.custom_size = Some(original_size.0);
            }
        }
    }
}

fn game_over(mut commands: Commands) {
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
                commands.entity(player_entity).insert(Health(INITIAL_HEALTH));
            }

            // Despawn all obstacles
            for obstacle_entity in obstacle_query.iter() {
                commands.entity(obstacle_entity).despawn_recursive();
            }

            // Despawn the "GAME OVER" text
            for text_entity in game_over_text_query.iter() {
                commands.entity(text_entity).despawn();
            }
        }
    }
}
