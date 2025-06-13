use bevy::prelude::*;

use crate::components::{Collider, Health, HealthPickup, Obstacle, Player};
use crate::constants::{HEALTH_PICKUP_SIZE, PLAYER_SIZE};

pub fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Children), With<Obstacle>>,
    health_pickup_query: Query<(Entity, &Transform), With<HealthPickup>>,
    collider_query: Query<(&GlobalTransform, &Collider)>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        let x_size_scale = 0.75;
        let player_half = Vec2::new(PLAYER_SIZE.x / 2.0 * x_size_scale, PLAYER_SIZE.y / 2.0);
        let player_translation = Vec3::new(player_transform.translation.x + PLAYER_SIZE.x / 2.0 * x_size_scale / 2.0, player_transform.translation.y, player_transform.translation.z);

        // Check collisions with obstacles
        for (entity, children) in obstacle_query.iter() {
            for &child in children.iter() {
                if let Ok((global_transform, collider)) = collider_query.get(child) {

                    if is_colliding(
                        player_translation,
                        player_half,
                        global_transform.translation(),
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