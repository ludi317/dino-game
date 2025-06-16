use bevy::prelude::*;
use bevy::color::palettes::basic::RED;

use crate::components::{CactusArm, CactusCollider, Collider, Health, HealthPickup, IsHit, Player, Velocity};

pub fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Children, &mut Health), With<Player>>,
    player_collider_query: Query<(&GlobalTransform, &Collider)>,

    collider_query: Query<(&GlobalTransform, &Collider, Entity), Or<(With<CactusCollider>, With<HealthPickup>)>>,
    mut cactus_collider: Query<&Parent, With<CactusCollider>>,
    mut children_query: Query<&Children>,
    mut cactus_arm_query: Query<(&mut IsHit, &mut Velocity), With<CactusArm>>,
) {
    // get player's health
    if let Ok((children, mut health)) = player_query.get_single_mut() {
        // get player's collider
        if let Ok((player_transform, player_collider)) = player_collider_query.get(*children.first().unwrap()) {
            let player_half = player_collider.size / 2.0;
            // query for colliders
            for (transform, collider, entity) in collider_query.iter() {
                // if player collided...
                if is_colliding(player_transform.translation(), player_half, transform.translation(), collider.size / 2.0) {
                    // ...with a cactus
                    if let Ok(parent) = cactus_collider.get_mut(entity) {
                        // get collider parent's children, aka siblings, which includes the cactus arm
                        if let Ok(children) = children_query.get_mut(**parent) {
                            // reset cactus arm velocity to 0
                            for &child in children {
                                if let Ok((mut is_hit, mut velocity)) = cactus_arm_query.get_mut(child) {
                                    is_hit.0 = true;
                                    velocity.0.y = 0.;
                                }
                            }
                        }
                        health.0 = health.0.saturating_sub(1);

                    // ...with a cheeseburger
                    } else {
                        health.0 = health.0.saturating_add(1);
                    }
                    commands.entity(entity).despawn();
                    // return;
                }
            }
        }
    }
}

pub fn is_colliding(pos1: Vec3, half_size1: Vec2, pos2: Vec3, half_size2: Vec2) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() <= (half_size1.x + half_size2.x);
    let collision_y = (pos1.y - pos2.y).abs() <= (half_size1.y + half_size2.y);
    collision_x && collision_y
}

#[allow(dead_code)]
pub fn debug_collider_outlines(
    query: Query<(&GlobalTransform, &Collider), Or<(With<CactusCollider>, With<HealthPickup>)>>,
    player_query: Query<&Children, With<Player>>,
    collider_query: Query<(&GlobalTransform, &Collider)>,
    mut gizmos: Gizmos,
) {
    for (transform, collider) in &query {
        // Calculate half sizes
        let half_width = collider.size.x / 2.0;
        let half_height = collider.size.y / 2.0;

        // Define the four corners of the collider in local space
        let corners = [
            Vec2::new(-half_width, -half_height), // Bottom-left
            Vec2::new(half_width, -half_height),  // Bottom-right
            Vec2::new(half_width, half_height),   // Top-right
            Vec2::new(-half_width, half_height),  // Top-left
        ];

        // Convert corners to world space and draw lines between them
        for i in 0..4 {
            let start = transform.transform_point(corners[i].extend(0.0));
            let end = transform.transform_point(corners[(i + 1) % 4].extend(0.0));

            gizmos.line_2d(
                start.truncate(),
                end.truncate(),
                RED, // You can customize the color
            );
        }
    }

    let player_collider = player_query.single().first().unwrap();
    if let Ok((transform, collider)) = collider_query.get(*player_collider) {
        // Calculate half sizes
        let half_width = collider.size.x / 2.0;
        let half_height = collider.size.y / 2.0;

        // Define the four corners of the collider in local space
        let corners = [
            Vec2::new(-half_width, -half_height), // Bottom-left
            Vec2::new(half_width, -half_height),  // Bottom-right
            Vec2::new(half_width, half_height),   // Top-right
            Vec2::new(-half_width, half_height),  // Top-left
        ];

        // Convert corners to world space and draw lines between them
        for i in 0..4 {
            let start = transform.transform_point(corners[i].extend(0.0));
            let end = transform.transform_point(corners[(i + 1) % 4].extend(0.0));

            gizmos.line_2d(
                start.truncate(),
                end.truncate(),
                RED, // You can customize the color
            );
        }
    }
}
