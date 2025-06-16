use crate::components::{CactusArm, CactusCollider, CactusRoot, Collider, IsHit, Velocity};
use crate::resources::CactusTexture;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::math::{Quat, Vec2};
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;
use rand_core::RngCore;
use std::f32::consts::PI;

const CACTUS_FLOWER_CHANCE: f32 = 0.3; // 30% chance to spawn a flower on top of cactus

pub fn spawn_cactus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cactus_texture: ResMut<CactusTexture>,
    position: Vec2,
    rng: &mut GlobalEntropy<WyRand>,
) {
    // cactus parameters
    let max_trunk_width = 30.0;
    let min_trunk_width = 23.0;
    let trunk_height = rng.gen_range(58..158) as f32;
    let min_arm_highness = 20.0;
    let max_arm_highness = trunk_height - 10.0;
    let min_arm_width = 15.0;
    let max_arm_width = 30.0;
    let trunk_width = rng.gen_range(min_trunk_width..=max_trunk_width);
    let arm_length = trunk_width / 2.0;
    let scale = (trunk_width - min_trunk_width) / (max_trunk_width - min_trunk_width);
    let arm_width = min_arm_width + scale * (max_arm_width - min_arm_width);

    // flower parameters
    let top_spike_count = 3;
    let spike_length = 8.0;
    let spike_width = 1.5;
    let flower = rng.next_u32() % 100 < (CACTUS_FLOWER_CHANCE * 100.0) as u32;


    commands.spawn((CactusRoot, Transform::from_xyz(position.x, position.y, 0.0), Visibility::Visible)).with_children(|root| {
        // Circle top
        let circle_radius = trunk_width / 2.0;
        root.spawn((
            Mesh2d(meshes.add(Circle::new(circle_radius)).into()),
            MeshMaterial2d(materials.add(cactus_texture.0.clone())),
            Transform::from_xyz(0.0, trunk_height, 0.1),
        ));

        // Top flower
        if flower {
            for i in 0..top_spike_count {
                let angle = PI + (i as f32 * std::f32::consts::TAU / top_spike_count as f32);

                root.spawn((
                    Mesh2d(meshes.add(Rectangle::new(spike_width, spike_length)).into()),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0.0, trunk_height + circle_radius, 0.5)
                        .with_rotation(Quat::from_rotation_z(angle)),
                ));
            }
        }

        // Main trunk
        root.spawn((
            Mesh2d(meshes.add(Rectangle::new(trunk_width, trunk_height)).into()),
            MeshMaterial2d(materials.add(cactus_texture.0.clone())),
            Transform::from_xyz(0.0, trunk_height / 2.0, 0.6),

        )).with_children(|trunk| {
            // cactus collider needs to be leaf node (for despawning),
            // and only 1 per cactus (for efficiency and single health point deduction),
            // and close to cactus arm (for convenience)
            // so it's the cactus arm sibling. both are children of the trunk.
            trunk.spawn((
                CactusCollider,
                Transform::IDENTITY,
                Collider{size: Vec2::new(trunk_width + 2.0 * arm_width - arm_length / 2.0, trunk_height)},
            ));

            // Generate cactus arms
            let x_multi = [1.0, -1.0];
            let curve_radius = arm_length;
            for i in 0..2 {
                let arm_highness = rng.gen_range(min_arm_highness..=max_arm_highness);
                let caps_length = (curve_radius * ((rng.next_u32() % 3 + 1) as f32)).min(trunk_height - arm_highness);

                trunk.spawn((
                    CactusArm,
                    IsHit(false),
                    Transform::from_xyz(10.0 * x_multi[i], arm_highness-trunk_height / 2.0, -0.6),  // offset the transform of the trunk
                    Visibility::Visible,
                    Velocity(Vec3::ZERO),
                )).with_children(|arm| {

                    // Horizontal side arm
                    let rect_width = arm_width - curve_radius;
                    arm.spawn((
                        Mesh2d(meshes.add(Rectangle::new(rect_width, arm_length)).into()),
                        MeshMaterial2d(materials.add(cactus_texture.0.clone())),
                        Transform::from_xyz(x_multi[i] * (rect_width / 2.0), 0., 0.2),
                    ));

                    // Curved segment to add texture noise between the horizontal and vertical segments
                    arm.spawn((
                        Mesh2d(meshes.add(CircularSector::from_radians(curve_radius, PI / 4.0)).into()),
                        MeshMaterial2d(materials.add(cactus_texture.0.clone())),
                        Transform::from_xyz(x_multi[i] * (arm_width - curve_radius), arm_length / 2.0, 0.3)
                            .with_rotation(Quat::from_rotation_z(x_multi[i] * PI)),
                    ));

                    // Vertical capsule
                    arm.spawn((
                        Mesh2d(meshes.add(Capsule2d::new(curve_radius / 2.0, caps_length)).into()),
                        MeshMaterial2d(materials.add(cactus_texture.0.clone())),
                        Transform::from_xyz(x_multi[i] * (arm_width - curve_radius / 2.0), caps_length / 2.0, 0.4),
                    ));

                    // Side arm flowers
                    if flower {
                        for j in 0..top_spike_count {
                            let angle = PI + (j as f32 * std::f32::consts::TAU / top_spike_count as f32);

                            arm.spawn((
                                Mesh2d(meshes.add(Rectangle::new(spike_width, spike_length)).into()),
                                MeshMaterial2d(materials.add(Color::WHITE)),
                                Transform::from_xyz(x_multi[i] * (arm_width - curve_radius / 2.0), caps_length + curve_radius / 2.0, 0.5)
                                    .with_rotation(Quat::from_rotation_z(angle)),
                            ));
                        }
                    };
                });
            }
        });
    });
}
