use crate::components::{HealthPickup, Obstacle, Sand};
use crate::constants::{GAME_SPEED, GROUND_LEVEL, HEALTH_PICKUP_SIZE};
use crate::resources::{AnimationState, CactusTexture, Cheeseburger, ObstacleSpawningTimer};
use crate::systems::obstacles::cactus::spawn_cactus;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand_core::RngCore;

const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3;

pub fn move_ground(
    // https://bevy.org/examples/2d-rendering/sprite-tile/
    mut sprites: Query<(&mut Transform, &mut Sprite), With<Sand>>,
    mut state: ResMut<AnimationState>,
    time: Res<Time>,
) {
    state.current += state.speed * time.delta_secs();
    let (mut transform, mut sprite) = sprites.single_mut();
    sprite.custom_size = Some(Vec2::new(state.current, 1080.0));
    transform.translation.x += 3.0;
}

pub fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut transforms: Query<(Entity, &mut Transform), Or<(With<Obstacle>, With<HealthPickup>)>>,
) {
    // Move obstacles
    for (entity, mut transform) in transforms.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    cheeseburger: ResMut<Cheeseburger>,
    cactus_texture: ResMut<CactusTexture>,
    mut rng: GlobalEntropy<WyRand>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let camera_transform = camera_query.single();
        let obstacle_x = camera_transform.translation.x + GROUND_EDGE + 200.0 + rng.next_u32() as f32 % 300.0 - 150.0;
        // add some randomness to the obstacle's y position
        let obstacle_y = GROUND_LEVEL + rng.next_u32() as f32 % 50.0 - 25.0;

        // Randomly decide whether to spawn obstacle or health pickup
        if rng.next_u32() % 100 < (HEALTH_PICKUP_SPAWN_CHANCE * 100.0) as u32 {
            // Spawn health pickup
            commands.spawn((
                HealthPickup,
                Sprite {
                    image: cheeseburger.0.clone(),
                    custom_size: Some(HEALTH_PICKUP_SIZE),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            ));
        } else {
            spawn_cactus(commands, meshes, materials,cactus_texture, Vec2::new(obstacle_x, obstacle_y), &mut rng);
        }
    }
}

