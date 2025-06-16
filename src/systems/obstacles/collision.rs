use bevy::color::palettes::basic::RED;
use bevy::prelude::*;

use crate::components::{AnimationIndices, CactusArm, CactusCollider, Collider, Health, HealthPickup, IsHit, Player, PterodactylCollider, Velocity};
use crate::constants::{PTERO_SIZE_X, PTERO_SIZE_Y};
use crate::resources::PterodactylDie;

pub fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(&Children, &mut Health), With<Player>>,
    player_collider_query: Query<(&GlobalTransform, &Collider)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    pterodactyl_die: ResMut<PterodactylDie>,
    collider_query: Query<(&GlobalTransform, &Collider, Entity), Or<(With<CactusCollider>, With<HealthPickup>, With<PterodactylCollider>)>>,
    mut cactus_collider: Query<&Parent, With<CactusCollider>>,
    mut pterodactyl_parent_query: Query<&Parent, With<PterodactylCollider>>,
    mut pterodactyl_query: Query<(&mut Sprite, &mut AnimationIndices, &mut IsHit), Without<CactusArm>>,
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

                   // ...with a pterodactyl
                    } else if let Ok(parent) = pterodactyl_parent_query.get_mut(entity) {
                        if let Ok((mut ptero_sprite, mut anim_indices, mut is_hit)) = pterodactyl_query.get_mut(**parent) {
                            let layout = TextureAtlasLayout::from_grid(UVec2::new(PTERO_SIZE_X, PTERO_SIZE_Y), 4, 1, None, None);
                            let texture_atlas_layout = texture_atlas_layouts.add(layout);

                            ptero_sprite.image = pterodactyl_die.0.clone();
                            ptero_sprite.texture_atlas = Some(TextureAtlas{
                                layout: texture_atlas_layout,
                                index: 0,
                            });
                            anim_indices.last = 3;
                            is_hit.0 = true;
                            health.0 = health.0.saturating_sub(1);
                        }
                    // ...with a health pickup
                    } else {
                        health.0 = health.0.saturating_add(1);
                    }
                    commands.entity(entity).despawn();
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
    collider_query: Query<(&GlobalTransform, &Collider)>,
    mut gizmos: Gizmos)
{
    for (transform, collider) in collider_query.iter() {
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
