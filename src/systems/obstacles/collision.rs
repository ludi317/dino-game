use bevy::color::palettes::basic::{BLUE, RED};
use bevy::prelude::*;

use crate::components::{AnimationIndices, CactusArm, CactusCollider, Collider, Health, HealthPickup, IsHit, PlayerCollider, PterodactylCollider, Velocity};
use crate::constants::{GROUND_LEVEL, PTERO_SIZE_X, PTERO_SIZE_Y};
use crate::resources::PterodactylDie;

pub fn detect_collision(
    mut commands: Commands,
    mut player_collider_query: Query<(&GlobalTransform, &Collider, &mut Health), With<PlayerCollider>>,
    collider_query: Query<(&GlobalTransform, &Collider, Entity), Or<(With<CactusCollider>, With<HealthPickup>, With<PterodactylCollider>)>>,

    mut cactus_collider: Query<&Parent, With<CactusCollider>>,
    mut children_query: Query<&Children>,
    mut cactus_arm_query: Query<(&mut IsHit, &mut Velocity), With<CactusArm>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

    mut pterodactyl_parent_query: Query<&Parent, With<PterodactylCollider>>,
    mut pterodactyl_query: Query<(&mut Sprite, &mut AnimationIndices, &mut IsHit, &mut Velocity), Without<CactusArm>>,
    pterodactyl_die: ResMut<PterodactylDie>,
) {
    // get player's health and collider
    let (player_transform, player_collider, mut health) =  player_collider_query.get_single_mut().unwrap();
    let player_half = player_collider.size / 2.0;

    // query for colliders
    for (transform, collider, entity) in collider_query.iter() {
        if is_colliding(player_transform.translation(), player_half, transform.translation(), collider.size / 2.0) {
            // cactus collision
            if let Ok(parent) = cactus_collider.get_mut(entity) {
                // get collider parent's children, aka siblings, which includes the cactus arm
                let children = children_query.get_mut(**parent).unwrap();
                // reset cactus arm velocity to 0
                for &child in children {
                    if let Ok((mut is_hit, mut velocity)) = cactus_arm_query.get_mut(child) {
                        is_hit.0 = true;
                        velocity.0.y = 0.;
                    }
                }
                health.0 = health.0.saturating_sub(1);

            // pterodactyl collision
            } else if let Ok(parent) = pterodactyl_parent_query.get_mut(entity) {
                let (mut ptero_sprite, mut anim_indices, mut is_hit, mut velocity) = pterodactyl_query.get_mut(**parent).unwrap();
                // change animation to die
                let layout = TextureAtlasLayout::from_grid(UVec2::new(PTERO_SIZE_X, PTERO_SIZE_Y), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                ptero_sprite.image = pterodactyl_die.0.clone();
                ptero_sprite.texture_atlas = Some(TextureAtlas{
                    layout: texture_atlas_layout,
                    index: 0,
                });
                anim_indices.last = 3;
                is_hit.0 = true;
                velocity.0.y = 0.;
                health.0 = health.0.saturating_sub(1);

            //  health pickup collision
            } else {
                health.0 = health.0.saturating_add(1);
            }
            // despawn collider after collision
            commands.entity(entity).despawn();
        }
    }
}

pub fn is_colliding(pos1: Vec3, half_size1: Vec2, pos2: Vec3, half_size2: Vec2) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() <= (half_size1.x + half_size2.x);
    let collision_y = (pos1.y - pos2.y).abs() <= (half_size1.y + half_size2.y);
    collision_x && collision_y
}

#[allow(dead_code)]
pub fn draw_ground_level(mut gizmos: Gizmos) {
    // Draw a horizontal line at GROUND_LEVEL
    const LENGTH: f32 = 999999.0;
    gizmos.line(
        Vec3::new(-1. * LENGTH / 2.0, GROUND_LEVEL, 10.0), // Start point
        Vec3::new(LENGTH / 2.0, GROUND_LEVEL, 10.0), // End point
        BLUE,
    );
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
                RED,
            );
        }
    }
}
