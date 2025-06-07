use bevy::input::ButtonState;
use crate::GameState::{GameOver, InGame};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
use rand_core::RngCore;

//region Constants
const GAME_SPEED: f32 = 400.0;
const JUMP_FORCE: f32 = 2000.0;
const GRAVITY: f32 = -4000.0;
const PLAYER_X: f32 = -300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const SPAWN_INTERVAL: f32 = 1.5;
const GROUND_LEVEL: f32 = -200.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const HEALTH_PICKUP_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3; // 30% chance to spawn instead of obstacle

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
        .insert_resource(ObstacleSpawningTimer(
            Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_state(InGame)
        .add_systems(Update, (jump, apply_gravity, player_movement, crouch)
            .run_if(in_state(InGame)))
        .add_systems(Update, toggle_pause)
        .add_systems(OnEnter(GameState::Paused), show_pause_text)
        .add_systems(OnExit(GameState::Paused), hide_pause_text)
        .add_systems(Update, (spawn_obstacles, move_obstacles, detect_collision, render_health_info, check_health)
            .run_if(in_state(InGame)))
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver))) // New system to restart the game
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let initial_health = 99;
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
            Health(initial_health),
            OriginalSize(PLAYER_SIZE),
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
            if e.state.is_pressed() && (e.key_code == KeyCode::Space || e.key_code == KeyCode::ArrowUp) && transform.translation.y <= GROUND_LEVEL {
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
    mut images: ResMut<Assets<Image>>,
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
            let cactus_image = generate_cactus(128, 100, &mut rng);
            let handle = images.add(cactus_image);
            // Spawn obstacle
            commands.spawn((
                Obstacle,
                Sprite {
                    image: handle,
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            ));
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
            commands.entity(entity).despawn();
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
    mut player_query: Query<(&Transform, &mut Health, &Sprite), With<Player>>,
    obstacle_query: Query<(Entity, &Transform, &Sprite), With<Obstacle>>,
    health_pickup_query: Query<(Entity, &Transform, &Sprite), With<HealthPickup>>,
) {
    if let Ok((player_transform, mut health, player_sprite)) = player_query.get_single_mut() {
        let player_size = player_sprite.custom_size.unwrap_or(PLAYER_SIZE);
        let player_half_width = player_size.x / 2.0;
        let player_half_height = player_size.y / 2.0;

        // Check collision with obstacles
        for (entity, obstacle_transform, obstacle_sprite) in obstacle_query.iter() {
            let obstacle_size = obstacle_sprite.custom_size.unwrap_or(OBSTACLE_SIZE);
            let obstacle_half_width = obstacle_size.x / 2.0;
            let obstacle_half_height = obstacle_size.y / 2.0;

            if is_colliding(
                player_transform.translation,
                player_half_width,
                player_half_height,
                obstacle_transform.translation,
                obstacle_half_width,
                obstacle_half_height,
            ) {
                health.0 = health.0.saturating_sub(1);
                commands.entity(entity).despawn();
            }
        }

        // Check collision with health pickups
        for (entity, pickup_transform, pickup_sprite) in health_pickup_query.iter() {
            let pickup_size = pickup_sprite.custom_size.unwrap_or(HEALTH_PICKUP_SIZE);
            let pickup_half_width = pickup_size.x / 2.0;
            let pickup_half_height = pickup_size.y / 2.0;

            if is_colliding(
                player_transform.translation,
                player_half_width,
                player_half_height,
                pickup_transform.translation,
                pickup_half_width,
                pickup_half_height,
            ) {
                health.0 = health.0.saturating_add(1);
                commands.entity(entity).despawn();
            }
        }
    }
}

// Helper function for collision detection
fn is_colliding(
    pos1: Vec3,
    half_width1: f32,
    half_height1: f32,
    pos2: Vec3,
    half_width2: f32,
    half_height2: f32,
) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() <= (half_width1 + half_width2);
    let collision_y = (pos1.y - pos2.y).abs() <= (half_height1 + half_height2);
    collision_x && collision_y
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
                commands.entity(player_entity).insert(Health(99));
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

fn generate_cactus(width: u32, height: u32, rng: &mut GlobalEntropy<WyRand>) -> Image {
    // Create empty image buffer
    let mut image = Image::new_fill(
        Extent3d { width, height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 0], // Transparent
        TextureFormat::Rgba8Unorm,
        default(),
    );

    let green = [0, 20, 0, 255];   // Cactus green

    // Calculate rectangle dimensions (smaller than full image)
    let trunk_width = width  / 6;
    let trunk_height = height * 2 / 3;
    let trunk_x = (width - trunk_width) / 2;
    let trunk_y = height - trunk_height; // Align to bottom

    let top_width = trunk_width * 3 / 4;
    let top_height = trunk_y / 7;
    let top_x_offset = (width - top_width) / 2;
    let top_y_offset = trunk_y - top_height; // Align to top

    // Fill the rectangle with green pixels
    for y in trunk_y..trunk_y +trunk_height {
        for x in trunk_x..trunk_x + trunk_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Draw the top segment (smaller rectangle)
    for y in top_y_offset..top_y_offset+top_height {
        for x in top_x_offset..top_x_offset + top_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Side arms parameters
    let arm_width = trunk_width * 3 / 4;
    let arm_height = trunk_height / 4;
    let square_size = arm_height / 2;

    let arm_y_min = trunk_y + trunk_height / 5;
    let arm_y_max = trunk_y + trunk_height * 9 / 10 - arm_height;
    let arm_y_left = arm_y_min + (rng.next_u32()  % (arm_y_max - arm_y_min));

    // Left arm
    for y in arm_y_left..arm_y_left + arm_height {
        for x in (trunk_x - arm_width)..trunk_x {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Left small square
    let left_square_x = trunk_x - arm_width - square_size;
    for y in arm_y_left +square_size/2..arm_y_left + square_size +square_size/2{
        for x in left_square_x..left_square_x + square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Tall narrow rectangle on left side
    let tall_rect_width = trunk_width *2/3;
    let tall_rect_height = arm_height * 3 / 2;
    let tall_rect_x = left_square_x + square_size - tall_rect_width; // Touches left arm
    let tall_rect_y = arm_y_left + square_size/2 - tall_rect_height; // Starts at top of small square

    for y in tall_rect_y..arm_y_left + square_size/2 {
        for x in tall_rect_x..tall_rect_x + tall_rect_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Small square on top of tall rectangle
    let top_square_size = 3*square_size/4;  // Slightly smaller than arm square
    let top_square_x = tall_rect_x + (tall_rect_width - top_square_size)/2;  // Centered horizontally
    let top_square_y = tall_rect_y - top_square_size;  // Positioned above tall rectangle

    for y in top_square_y..tall_rect_y {
        for x in top_square_x..top_square_x + top_square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    let arm_y_right = arm_y_min + (rng.next_u32() % (arm_y_max - arm_y_min));

    // Right arm
    for y in arm_y_right..arm_y_right + arm_height {
        for x in (trunk_x + trunk_width)..(trunk_x + trunk_width + arm_width) {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    let right_square_x = trunk_x + trunk_width + arm_width;
    for y in arm_y_right +square_size/2..arm_y_right + square_size+square_size/2 {
        for x in right_square_x..right_square_x + square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Tall narrow rectangle on right side
    let right_tall_rect_width = trunk_width * 2/3;
    let right_tall_rect_height = arm_height * 3 / 2;
    let right_tall_rect_x = right_square_x; // Touches right arm
    let right_tall_rect_y = arm_y_right + square_size/2 - right_tall_rect_height; // Starts at top of small square

    for y in right_tall_rect_y..arm_y_right + square_size/2 {
        for x in right_tall_rect_x..right_tall_rect_x + right_tall_rect_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Small square on top of right tall rectangle
    let right_top_square_size = square_size * 3/4;
    let right_top_square_x = right_tall_rect_x + (right_tall_rect_width - right_top_square_size)/2;
    let right_top_square_y = right_tall_rect_y - right_top_square_size;

    for y in right_top_square_y..right_tall_rect_y {
        for x in right_top_square_x..right_top_square_x + right_top_square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }


    // Add random yellow squares (flowers)
    let orange = [255, 100, 0, 255];
    let square_size = 3; // Size of yellow squares (2x2 pixels)


    // Place 5 squares in trunk (distributed vertically)
    for i in 0..5 {
        let square_x = trunk_x + (rng.next_u32() as u32 % (trunk_width - square_size));
        let square_y = trunk_y + (trunk_height * (i ) / 5) - (square_size / 2);

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Place 2 squares in left arm
    for _ in 0..2 {
        let square_x = (trunk_x - arm_width) + (rng.next_u32() as u32 % (arm_width - square_size));
        let square_y = arm_y_left + (rng.next_u32() as u32 % (arm_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Place 2 squares in right arm
    for _ in 0..2 {
        let square_x = (trunk_x + trunk_width) + (rng.next_u32() as u32 % (arm_width - square_size));
        let square_y = arm_y_right + (rng.next_u32() as u32 % (arm_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Left tall rectangle
    for _ in 0..1 {
        let square_x = tall_rect_x + (rng.next_u32() as u32 % (tall_rect_width - square_size));
        let square_y = tall_rect_y + (rng.next_u32() as u32 % (tall_rect_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Right tall rectangle
    for _ in 0..1 {
        let square_x = right_tall_rect_x + (rng.next_u32() as u32 % (right_tall_rect_width - square_size));
        let square_y = right_tall_rect_y + (rng.next_u32() as u32 % (right_tall_rect_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }
    image
}
