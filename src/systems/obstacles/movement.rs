use crate::components::{CactusArm, Collider, HealthPickup, CactusRoot, Sand, Velocity, IsHit};
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

pub fn move_obstacles_y(time: Res<Time>,
                        mut arms_query: Query<(&IsHit, &mut Transform, &mut Velocity), With<CactusArm>>) {
    let mut ang_vel = 8.0;
    for (is_hit, mut transform, mut velocity) in arms_query.iter_mut() {
        if is_hit.0 {
            transform.translation.z = -0.1; // found by trial and error
            transform.translation.y += velocity.0.y * time.delta_secs();
            ang_vel *= -1.;
            transform.rotate_z(ang_vel*time.delta_secs());
            if transform.translation.y <= GROUND_LEVEL + 200.{
                transform.translation.y = GROUND_LEVEL + 200.;
                velocity.0.y = 0.0;
                transform.rotate_z(-1.0 * ang_vel * time.delta_secs());
            }
        }
    }
}


pub fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut transforms: ParamSet<(
        Query<(Entity, &mut Transform), With<CactusRoot>>,
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
            let obstacle_y = GROUND_LEVEL + rng.gen_range(-80.0..-20.) ;
            spawn_cactus(commands, meshes, materials,cactus_texture, Vec2::new(obstacle_x, obstacle_y), &mut rng);
        }
    }
}

