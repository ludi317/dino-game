mod cactus;

use crate::cactus::spawn_cactus;
use crate::GameState::{GameOver, InGame};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
use rand_core::RngCore;
use bevy_parallax::{LayerSpeed, LayerData, ParallaxCameraComponent, ParallaxPlugin, CreateParallaxEvent, ParallaxSystems, ParallaxMoveEvent};

//region Constants
const GAME_SPEED: f32 = 400.0;
const JUMP_FORCE: f32 = 1800.0;
const GRAVITY: f32 = -4000.0;
const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(87.0, 94.0);
const SPAWN_INTERVAL: f32 = 1.5;
const GROUND_LEVEL: f32 = -300.0;
const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(80.0, 100.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const HEALTH_PICKUP_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3;
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
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


//endregion


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    main();
}

fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..default()
    };
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParallaxPlugin)
        .add_systems(Startup, initialize_camera_system)
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
        .add_systems(Update,
            (spawn_obstacles, move_obstacles, detect_collision, render_health_info, check_health,
             animate_sprite, move_camera_system.before(ParallaxSystems))
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver))) // New system to restart the game
        .run();
}

pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut transforms: Query<&mut Transform, Or<(With<Player>, With<Obstacle>, With<HealthPickup>)>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.single();

    move_event_writer.send(ParallaxMoveEvent {
        translation: Vec2::new(3.0, 0.0),
        rotation: 0.0,
        camera,
    });

    for mut transform in &mut transforms {
        transform.translation.x += 3.0;
    }
}

pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>
) {
    let camera = commands
        .spawn(Camera2d::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    let event = CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "9 Background.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -9.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "8 Stars.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -8.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "7 Clouds.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -7.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.8),
                path: "6 Sun.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -6.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.7),
                path: "5 Mountains.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -5.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.5),
                path: "4 Layer4.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -4.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.4),
                path: "3 Layer3.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -3.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.3),
                path: "2 Layer2.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -2.0,
                ..default()
            },
            // LayerData {
            //     speed: LayerSpeed::Horizontal(0.2),
            //     path: "1 Layer1.png".to_string(),
            //     tile_size: UVec2::new(1920, 1080),
            //     cols: 1,
            //     rows: 1,
            //     scale: Vec2::splat(1.0),
            //     z: -1.0,
            //     ..default()
            // },
        ],
        camera: camera,
    };
    create_parallax.send(event);
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    // commands.spawn(Camera2d::default());

    let texture = asset_server.load("DinoRun1-0.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(87, 94), 2, 1, Some(UVec2::new(1,0)), None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 1 };

    // Player
    commands.spawn((
        Player,
        Sprite{
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
        Health(INITIAL_HEALTH),
        OriginalSize(PLAYER_SIZE),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),

    ));

    commands.spawn((HealthInfo, Text::new(format!("Health: {}", INITIAL_HEALTH))));

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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>, // Get camera position
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let camera_transform = camera_query.single();
        let obstacle_x = camera_transform.translation.x + GROUND_EDGE;
        // add some randomness to the obstacle's y position
        let obstacle_y = GROUND_LEVEL + rng.next_u32() as f32 % 50.0 - 25.0;

        // Randomly decide whether to spawn obstacle or health pickup
        if rng.next_u32() % 100 < (HEALTH_PICKUP_SPAWN_CHANCE * 100.0) as u32 {
            // Spawn health pickup
            commands.spawn((
                HealthPickup,
                Sprite {
                    image: asset_server.load("cheeseburger.png"),
                    custom_size: Some(HEALTH_PICKUP_SIZE),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Collider {
                    size: HEALTH_PICKUP_SIZE,
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            ));
        } else {
            spawn_cactus(commands, meshes, materials, Vec2::new(obstacle_x, obstacle_y), &mut rng);
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
    health_pickup_query: Query<(Entity, &Transform), With<HealthPickup>>,
    collider_query: Query<(&Transform, &Collider)>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        let player_size = PLAYER_SIZE;
        let player_half = player_size / 2.0;

        // Check collisions with obstacles
        for (entity, obstacle_transform, children) in obstacle_query.iter() {
            for &child in children.iter() {
                if let Ok((child_transform, collider)) = collider_query.get(child) {
                    let global_transform = obstacle_transform.mul_transform(*child_transform);

                    if is_colliding(
                        player_transform.translation,
                        player_half,
                        global_transform.translation,
                        collider.size / 2.0,
                    ) {
                        health.0 = health.0.saturating_sub(1);
                        commands.entity(entity).despawn_recursive();
                        break;
                    }
                }
            }
        }

        // Check collisions with health pickups
        for (entity, pickup_transform) in health_pickup_query.iter() {
            if is_colliding(
                player_transform.translation,
                player_half,
                pickup_transform.translation,
                HEALTH_PICKUP_SIZE / 2.0,
            ) {
                health.0 = health.0.saturating_add(1);
                commands.entity(entity).despawn();
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
