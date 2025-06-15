use crate::components::{Collider, HealthPickup, Obstacle, Sand};
use crate::constants::{GAME_SPEED, GROUND_LEVEL, HEALTH_PICKUP_SIZE};
use crate::resources::{AnimationState, CactusTexture, Cheeseburger, ObstacleSpawningTimer};
use crate::systems::obstacles::cactus::spawn_cactus;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;
use rand_core::RngCore;

const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3;
const SKY_OFFSET: f32 = GROUND_LEVEL + 300.0;
const FLY_SPEED: f32 = 100.0;

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
    mut transforms: ParamSet<(
        Query<(Entity, &mut Transform), With<Obstacle>>,
        Query<(Entity, &mut Transform), With<HealthPickup>>,
    )>,
) {
    // Move obstacles
    for (entity, mut transform) in transforms.p0().iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).try_despawn_recursive();
        }
    }

    // Move health pickups
    for (entity, mut transform) in transforms.p1().iter_mut() {
        transform.translation.x -= (GAME_SPEED + FLY_SPEED) * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).try_despawn();
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

        // Randomly decide whether to spawn obstacle or health pickup
        if rng.next_u32() % 100 < (HEALTH_PICKUP_SPAWN_CHANCE * 100.0) as u32 {
            let obstacle_y = SKY_OFFSET + rng.next_u32() as f32 % 300.0 - 150.0;
            // Spawn health pickup
            commands.spawn((
                HealthPickup,
                Sprite {
                    image: cheeseburger.0.clone(),
                    custom_size: Some(HEALTH_PICKUP_SIZE),
                    ..default()
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
                Collider{
                    size : HEALTH_PICKUP_SIZE,
                }
            ));
        } else {
            let obstacle_y = GROUND_LEVEL + rng.gen_range(-80.0..-20.);
            spawn_cactus(commands, meshes, materials,cactus_texture, Vec2::new(obstacle_x, obstacle_y), &mut rng);
        }
    }
}

