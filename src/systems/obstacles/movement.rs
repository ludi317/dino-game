use crate::components::{AnimationIndices, AnimationTimer, CactusArm, CactusRoot, Collider, HealthPickup, IsHit, Pterodactyl, PterodactylCollider, Sand, Velocity};
use crate::constants::{GAME_SPEED, GROUND_LEVEL, PTERO_SIZE, PTERO_SIZE_X, PTERO_SIZE_Y, SAND_SIZE_X, SAND_SIZE_Y, WINDOW_WIDTH};
use crate::resources::{AnimationState, CactusTexture, HealthPickUpImg, ObstacleSpawningTimer, PterodactylFly};
use crate::systems::obstacles::cactus::spawn_cactus;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;
use rand_core::RngCore;

const GROUND_SIZE: Vec2 = Vec2::new(1400.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const SKY_SPAWN_CHANCE: f32 = 0.3;
const FLY_SPEED: f32 = 100.0;

const HEALTH_SIZE_X: u32 = 544;
const HEALTH_SIZE_Y: u32 = 457;
const HEALTH_SCALE: f32 = 40./HEALTH_SIZE_X as f32;
const HEALTH_PICKUP_SIZE: Vec2 = Vec2::new(HEALTH_SIZE_X as f32 * HEALTH_SCALE, HEALTH_SIZE_Y as f32 * HEALTH_SCALE);

const SKY_OBSTACLE_CHANCE: f32 = 0.5;


pub fn move_ground(
    // https://bevy.org/examples/2d-rendering/sprite-tile/
    mut sprites: Query<&mut Sprite, With<Sand>>,
    mut state: ResMut<AnimationState>,
    time: Res<Time>,
) {
    state.current += state.speed * time.delta_secs();
    if state.current >= 2.0 * (SAND_SIZE_X + WINDOW_WIDTH) {
        state.current = (state.current % SAND_SIZE_X) + SAND_SIZE_X;
    }
    let mut sprite = sprites.single_mut();
    sprite.custom_size = Some(Vec2::new(state.current, SAND_SIZE_Y));
}

pub fn drop_obstacles(time: Res<Time>,
                      mut transforms: Query<(&IsHit, &mut Transform, &mut Velocity, &GlobalTransform), Or<(With<Pterodactyl>, With<CactusArm>)>>,

) {
    let mut ang_vel = 8.0;
    for (is_hit, mut transform, mut velocity, gt) in transforms.iter_mut() {
        if is_hit.0 {
            transform.translation.z = -0.1; // found by trial and error
            let orig_transform_y = transform.translation.y;
            transform.translation.y += velocity.0.y * time.delta_secs();
            ang_vel *= -1.;
            transform.rotate_z(ang_vel*time.delta_secs());
            // if obstacle hit the ground
            if gt.translation().y <= GROUND_LEVEL {
                velocity.0.y = 0.0;
                // undo the incremental transforms on y and z
                transform.translation.y = orig_transform_y;
                transform.rotate_z(-1.0 * ang_vel * time.delta_secs());
            }
        }
    }
}

pub fn move_sky_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut transforms: Query<(Entity, &mut Transform), Or<(With<HealthPickup>, With<Pterodactyl>)>>,
) {
    // Move obstacles
    for (entity, mut transform) in transforms.iter_mut() {
        transform.translation.x -= (GAME_SPEED + FLY_SPEED) * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).try_despawn_recursive();
        }
    }
}

pub fn move_ground_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut transforms: Query<(Entity, &mut Transform), With<CactusRoot>>,
) {
    // Move obstacles
    for (entity, mut transform) in transforms.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).try_despawn_recursive();
        }
    }
}

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    health_pickup: Res<HealthPickUpImg>,
    cactus_texture: Res<CactusTexture>,
    pterodactyl_fly: Res<PterodactylFly>,
    mut rng: GlobalEntropy<WyRand>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    camera_query: Query<&Transform, With<Camera>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let camera_transform = camera_query.single();
        let obstacle_x = camera_transform.translation.x + GROUND_EDGE + 200.0 + rng.next_u32() as f32 % 300.0 - 150.0;
        let rand_n = rng.next_u32() % 100;
        // Randomly decide whether to spawn obstacle or health pickup
        if rand_n < (SKY_SPAWN_CHANCE * 100.0) as u32 {
            let obstacle_y = rng.gen_range(GROUND_LEVEL+100.0..-GROUND_LEVEL);

            // pterodactyl
            if rand_n < (SKY_OBSTACLE_CHANCE * SKY_SPAWN_CHANCE * 100.0) as u32 {
                let layout = TextureAtlasLayout::from_grid(UVec2::new(PTERO_SIZE_X, PTERO_SIZE_Y), 4, 3, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((
                    Pterodactyl,
                    Sprite {
                        image: pterodactyl_fly.0.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout,
                            index: 0,
                        }),
                        custom_size: Some(PTERO_SIZE),
                        ..default()
                    },
                    Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
                    Velocity(Vec3::ZERO),
                    IsHit(false),
                    AnimationIndices { first: 0, last: 11 },
                    AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
                )).with_children(|ptero| {
                    ptero.spawn((
                        PterodactylCollider,
                        Collider {
                            size: PTERO_SIZE,
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                });
            } else {
                // food
                commands.spawn((
                    HealthPickup,
                    Sprite {
                        image: health_pickup.0.clone(),
                        custom_size: Some(HEALTH_PICKUP_SIZE),
                        ..default()
                    },
                    Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
                    Collider{
                        size : HEALTH_PICKUP_SIZE,
                    }
                ));
            }

        } else {
            let obstacle_y = GROUND_LEVEL;
            spawn_cactus(commands, meshes, materials,cactus_texture, Vec2::new(obstacle_x, obstacle_y), &mut rng);
        }
    }
}
