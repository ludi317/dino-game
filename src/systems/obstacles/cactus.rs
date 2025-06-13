use crate::components::{Collider, Obstacle};
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
use crate::resources::CactusTexture;

const CACTUS_FLOWER_CHANCE: f32 = 0.3; // 30% chance to spawn a flower on top of cactus

pub fn spawn_cactus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cactus_texture: ResMut<CactusTexture>,
    position: Vec2,
    rng: &mut GlobalEntropy<WyRand>,
    asset_server: Res<AssetServer>,
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

    if cactus_texture.image.is_none() {
        cactus_texture.image = Some(asset_server.load("cactus texture.png"));
    }
    commands.spawn((
            Obstacle,
            Transform::from_xyz(position.x, position.y, 0.0),
            Visibility::Visible,
        )).with_children(|parent| {

        // Main trunk
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(trunk_width, trunk_height)).into()),
            MeshMaterial2d(materials.add(cactus_texture.clone().image.unwrap())),
            Transform::from_xyz(0.0, trunk_height / 2.0, 0.6),
            Collider {
                size: Vec2::new(trunk_width, trunk_height),
            },
        ));

        // Circle top
        let circle_radius = trunk_width / 2.0;
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(circle_radius)).into()),
            MeshMaterial2d(materials.add(cactus_texture.clone().image.unwrap())),
            Transform::from_xyz(0.0, trunk_height, 0.1),
        ));

        // Generate the arms
        let x_multi = [1.0, -1.0];
        let curve_radius = arm_length;

        for i in 0..2 {
            let arm_highness = rng.gen_range(min_arm_highness..=max_arm_highness);
            let arm_offset = Transform::from_xyz(10.0 * x_multi[i], arm_highness, 0.0);

            let rect_width = arm_width - curve_radius;
            // Horizontal side arm
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(rect_width, arm_length)).into()),
                MeshMaterial2d(materials.add(cactus_texture.clone().image.unwrap())),
                TransformRelative(&arm_offset,
                                  x_multi[i] * (rect_width / 2.0),
                                  0.,
                                  0.2),
            ));


            // Curved segment to add texture noise between the horizontal and vertical segments
            parent.spawn((
                Mesh2d(meshes.add(CircularSector::from_radians(curve_radius, PI / 4.0)).into()),
                MeshMaterial2d(materials.add(cactus_texture.clone().image.unwrap())),
                TransformRelative(&arm_offset,
                    x_multi[i] * (arm_width - curve_radius),
                    arm_length / 2.0,
                    0.3,
                ).with_rotation(Quat::from_rotation_z(x_multi[i] * PI)),
            ));


            // Vertical capsule
            let caps_length =
                (curve_radius * ((rng.next_u32() % 3 + 1) as f32)).min(trunk_height - arm_highness);

            parent.spawn((
                Mesh2d(meshes.add(Capsule2d::new(curve_radius / 2.0, caps_length)).into()),
                MeshMaterial2d(materials.add(cactus_texture.clone().image.unwrap())),
                TransformRelative(
                    &arm_offset,
                    x_multi[i] * (arm_width - curve_radius / 2.0),
                    caps_length / 2.0,
                    0.4,
                ),

            ));
            // Side arm flowers
            if flower {
                for j in 0..top_spike_count {
                    let angle = PI + (j as f32 * std::f32::consts::TAU / top_spike_count as f32);

                    parent.spawn((
                        Mesh2d(meshes.add(Rectangle::new(spike_width, spike_length)).into()),
                        MeshMaterial2d(materials.add(Color::WHITE)),
                        TransformRelative(
                            &arm_offset,
                            x_multi[i] * (arm_width - curve_radius / 2.0),
                            caps_length + curve_radius / 2.0,
                            0.5,
                        ).with_rotation(Quat::from_rotation_z(angle)),
                    ));
                }
            }
        }

        // Top flower
        if flower {
            for i in 0..top_spike_count {
                let angle = PI + (i as f32 * std::f32::consts::TAU / top_spike_count as f32);

                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(spike_width, spike_length)).into()),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0.0, trunk_height + circle_radius, 0.5)
                        .with_rotation(Quat::from_rotation_z(angle)),
                ));
            }
        }
    });
}

#[allow(non_snake_case)]
fn TransformRelative(parent: &Transform, x: f32, y: f32, z: f32) -> Transform {
    let mut transform = Transform::from_xyz(x, y, z);
    transform.translation = parent.translation + transform.translation;
    transform
}