use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand_core::RngCore;
use crate::components::{Collider, Health, HealthPickup, Obstacle, Player};
use crate::constants::{GROUND_LEVEL, PLAYER_SIZE};
use crate::resources::{CactusTexture, Cheeseburger, ObstacleSpawningTimer};
use crate::systems::obstacles::cactus::spawn_cactus;

const GAME_SPEED: f32 = 400.0;
const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const SCALE: f32 = 0.5;
const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(77.0* SCALE, 70.0* SCALE);
const HEALTH_PICKUP_SPAWN_CHANCE: f32 = 0.3;

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

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut cheeseburger: ResMut<Cheeseburger>,
    cactus_texture: ResMut<CactusTexture>,
    mut rng: GlobalEntropy<WyRand>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>, // Get camera position
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let camera_transform = camera_query.single();
        let obstacle_x = camera_transform.translation.x + GROUND_EDGE + 200.0 + rng.next_u32() as f32 % 300.0 - 150.0;
        // add some randomness to the obstacle's y position
        let obstacle_y = GROUND_LEVEL + rng.next_u32() as f32 % 50.0 - 25.0;

        // Randomly decide whether to spawn obstacle or health pickup
        if rng.next_u32() % 100 < (HEALTH_PICKUP_SPAWN_CHANCE * 100.0) as u32 {
            if cheeseburger.image.is_none() {
                cheeseburger.image = Some(asset_server.load("cheeseburger.png"));
            }
            // Spawn health pickup
            commands.spawn((
                HealthPickup,
                Sprite {
                    image: cheeseburger.clone().image.unwrap(),
                    custom_size: Some(HEALTH_PICKUP_SIZE),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
            ));
        } else {
            spawn_cactus(commands, meshes, materials,cactus_texture, Vec2::new(obstacle_x, obstacle_y), &mut rng, asset_server);
        }
    }
}

pub fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Transform, &Children), With<Obstacle>>,
    health_pickup_query: Query<(Entity, &Transform), With<HealthPickup>>,
    collider_query: Query<(&Transform, &Collider)>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        let x_size_scale = 0.75;
        let player_half = Vec2::new(PLAYER_SIZE.x / 2.0 * x_size_scale, PLAYER_SIZE.y / 2.0);
        let player_translation = Vec3::new(player_transform.translation.x + PLAYER_SIZE.x / 2.0 * x_size_scale / 2.0, player_transform.translation.y, player_transform.translation.z);

        // Check collisions with obstacles
        for (entity, obstacle_transform, children) in obstacle_query.iter() {
            for &child in children.iter() {
                if let Ok((child_transform, collider)) = collider_query.get(child) {
                    let global_transform = obstacle_transform.mul_transform(*child_transform);

                    if is_colliding(
                        player_translation,
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

pub fn is_colliding(pos1: Vec3, half_size1: Vec2, pos2: Vec3, half_size2: Vec2) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() <= (half_size1.x + half_size2.x);
    let collision_y = (pos1.y - pos2.y).abs() <= (half_size1.y + half_size2.y);
    collision_x && collision_y
}