use bevy::prelude::*;
use bevy::color::palettes::basic::RED;

use crate::components::{CactusArm, CactusChild, CactusTrunk, Collider, Health, HealthPickup, Player, Velocity};

pub fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Children, &mut Health), With<Player>>,
    player_collider_query: Query<(&GlobalTransform, &Collider)>,

    collider_query: Query<(&GlobalTransform, &Collider, Entity), Or<(With<CactusChild>, With<HealthPickup>)>>,
    mut cactus_trunk_collider: Query<(Entity, &Parent), With<CactusChild>>,
    mut cactus_trunk_query: Query<(&mut CactusTrunk, &Children)>,
    mut cactus_arm_query: Query<&mut Velocity, With<CactusArm>>,
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
                    if let Ok((mut trunk_collider, parent)) = cactus_trunk_collider.get_mut(entity) {
                        // get the cactus trunk
                        if let Ok((mut trunk, children)) = cactus_trunk_query.get_mut(**parent) {
                            trunk.is_hit = true;
                            // reset cactus arm velocity to 0
                            for &child in children {
                                if let Ok((mut velocity)) = cactus_arm_query.get_mut(child) {
                                    velocity.0.y = 0.;
                                }
                            }
                        }
                        health.0 = health.0.saturating_sub(1);
                        commands.entity(trunk_collider).despawn();
                    // ...with a cheeseburger
                    } else {
                        commands.entity(entity).despawn();
                        health.0 = health.0.saturating_add(1);
                    }
                    return;
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

pub fn debug_collider_outlines(
    query: Query<(&GlobalTransform, &Collider), Or<(With<CactusChild>, With<HealthPickup>)>>,
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
