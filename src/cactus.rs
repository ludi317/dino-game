use std::f32::consts::PI;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::image::Image;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{default, Capsule2d, Circle, CircularSector, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Rectangle, ResMut, Transform, Visibility};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;
use rand_core::RngCore;
use crate::{Collider, Obstacle};

pub(crate) fn spawn_cactus(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    rng: &mut GlobalEntropy<WyRand>,
) {
    let green = Color::srgb(0.0, 0.5, 0.1);
    let black = Color::BLACK;
    let blue = Color::srgb(0.0, 0.0, 1.0);
    let orange = Color::srgb(1.0, 0.5, 0.0);

    // Create the main cactus parent first
    let cactus_entity = commands.spawn((
        Obstacle,
        Transform::from_xyz(position.x, position.y, 0.0),
        Visibility::Visible,
    )).id();

    let max_trunk_width = 20.0;
    let min_trunk_width = 13.0;
    let trunk_width = rng.gen_range(min_trunk_width..=max_trunk_width);
    let trunk_height = rng.gen_range(38..58) as f32;
    // Add components to main cactus
    commands.entity(cactus_entity).with_children(|parent| {
        // Main trunk
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(trunk_width, trunk_height)).into()),
            MeshMaterial2d(materials.add(green)),
            Transform::from_xyz(0.0, trunk_height/2.0, 0.0),
            Collider { size: Vec2::new(trunk_width, trunk_height) },
        ));

        // Circle top
        parent.spawn((
            Mesh2d(meshes.add(Circle::new(trunk_width/2.0)).into()),
            MeshMaterial2d(materials.add(green)),
            Transform::from_xyz(0.0, trunk_height, 0.0),
        ));
    });

    let min_arm_width = 9.0;
    let max_arm_width = 20.0;
    let x_multi = [1.0, -1.0];

    let arm_length = trunk_width / 2.0;
    let scale = (trunk_width - min_trunk_width) / (max_trunk_width - min_trunk_width);
    let arm_width = min_arm_width + scale * (max_arm_width - min_arm_width);
    let curve_radius = arm_length;

    // Generate the arms
    for i in 0..2 {
        let arm_highness = rng.gen_range(10.0..=30.0);
        let caps_length = (curve_radius * ((rng.next_u32() % 3 + 1) as f32)).min(trunk_height - arm_highness);

        commands.entity(cactus_entity).with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(10.0 * x_multi[i], arm_highness, 0.0),
                Visibility::Visible,
            )).with_children(|arm| {
                // Horizontal side arm
                arm.spawn((
                    Mesh2d(meshes.add(Rectangle::new(arm_width, arm_length)).into()),
                    MeshMaterial2d(materials.add(green)),
                ));
                // Curved segment
                arm.spawn((
                    Mesh2d(meshes.add(CircularSector::new(curve_radius, PI / 2.0)).into()),
                    MeshMaterial2d(materials.add(green)),
                    Transform::IDENTITY.with_translation(Vec3::new(x_multi[i] * (arm_width -curve_radius), arm_length/2.0, 0.0))
                        .with_rotation(Quat::from_rotation_z(5.0* x_multi[i]*PI/4.0 )),
                ));
                // Vertical capsule
                arm.spawn((
                    Mesh2d(meshes.add(Capsule2d::new(curve_radius/2.0, caps_length)).into()),
                    MeshMaterial2d(materials.add(green)),
                    Transform::IDENTITY.with_translation(Vec3::new(x_multi[i] * (arm_width -curve_radius/2.0), arm_length + curve_radius/2.1, 0.0)),
                ));
            });
        });
    }
}


pub(crate) fn generate_cactus(width: u32, height: u32, rng: &mut GlobalEntropy<WyRand>) -> Image {
    // Create empty image buffer
    let mut image = Image::new_fill(
        Extent3d { width, height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 0], // Transparent
        TextureFormat::Rgba8Unorm,
        default(),
    );

    let green = [0, 20, 0, 255];   // Cactus green

    // Calculate rectangle dimensions (smaller than full image)
    let trunk_width = width  / 6;
    let trunk_height = height * 2 / 3;
    let trunk_x = (width - trunk_width) / 2;
    let trunk_y = height - trunk_height; // Align to bottom

    let top_width = trunk_width * 3 / 4;
    let top_height = trunk_y / 7;
    let top_x_offset = (width - top_width) / 2;
    let top_y_offset = trunk_y - top_height; // Align to top

    // Fill the rectangle with green pixels
    for y in trunk_y..trunk_y +trunk_height {
        for x in trunk_x..trunk_x + trunk_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Draw the top segment (smaller rectangle)
    for y in top_y_offset..top_y_offset+top_height {
        for x in top_x_offset..top_x_offset + top_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Side arms parameters
    let arm_width = trunk_width * 3 / 4;
    let arm_height = trunk_height / 4;
    let square_size = arm_height / 2;

    let arm_y_min = trunk_y + trunk_height / 5;
    let arm_y_max = trunk_y + trunk_height * 9 / 10 - arm_height;
    let arm_y_left = arm_y_min + (rng.next_u32()  % (arm_y_max - arm_y_min));

    // Left arm
    for y in arm_y_left..arm_y_left + arm_height {
        for x in (trunk_x - arm_width)..trunk_x {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Left small square
    let left_square_x = trunk_x - arm_width - square_size;
    for y in arm_y_left +square_size/2..arm_y_left + square_size +square_size/2{
        for x in left_square_x..left_square_x + square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Tall narrow rectangle on left side
    let tall_rect_width = trunk_width *2/3;
    let tall_rect_height = arm_height * 3 / 2;
    let tall_rect_x = left_square_x + square_size - tall_rect_width; // Touches left arm
    let tall_rect_y = arm_y_left + square_size/2 - tall_rect_height; // Starts at top of small square

    for y in tall_rect_y..arm_y_left + square_size/2 {
        for x in tall_rect_x..tall_rect_x + tall_rect_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Small square on top of tall rectangle
    let top_square_size = 3*square_size/4;  // Slightly smaller than arm square
    let top_square_x = tall_rect_x + (tall_rect_width - top_square_size)/2;  // Centered horizontally
    let top_square_y = tall_rect_y - top_square_size;  // Positioned above tall rectangle

    for y in top_square_y..tall_rect_y {
        for x in top_square_x..top_square_x + top_square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    let arm_y_right = arm_y_min + (rng.next_u32() % (arm_y_max - arm_y_min));

    // Right arm
    for y in arm_y_right..arm_y_right + arm_height {
        for x in (trunk_x + trunk_width)..(trunk_x + trunk_width + arm_width) {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    let right_square_x = trunk_x + trunk_width + arm_width;
    for y in arm_y_right +square_size/2..arm_y_right + square_size+square_size/2 {
        for x in right_square_x..right_square_x + square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Tall narrow rectangle on right side
    let right_tall_rect_width = trunk_width * 2/3;
    let right_tall_rect_height = arm_height * 3 / 2;
    let right_tall_rect_x = right_square_x; // Touches right arm
    let right_tall_rect_y = arm_y_right + square_size/2 - right_tall_rect_height; // Starts at top of small square

    for y in right_tall_rect_y..arm_y_right + square_size/2 {
        for x in right_tall_rect_x..right_tall_rect_x + right_tall_rect_width {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }

    // Small square on top of right tall rectangle
    let right_top_square_size = square_size * 3/4;
    let right_top_square_x = right_tall_rect_x + (right_tall_rect_width - right_top_square_size)/2;
    let right_top_square_y = right_tall_rect_y - right_top_square_size;

    for y in right_top_square_y..right_tall_rect_y {
        for x in right_top_square_x..right_top_square_x + right_top_square_size {
            let index = (y * width + x) as usize * 4;
            image.data[index..index + 4].copy_from_slice(&green);
        }
    }


    // Add random yellow squares (flowers)
    let orange = [255, 100, 0, 255];
    let square_size = 3; // Size of yellow squares (2x2 pixels)


    // Place 5 squares in trunk (distributed vertically)
    for i in 0..5 {
        let square_x = trunk_x + (rng.next_u32() as u32 % (trunk_width - square_size));
        let square_y = trunk_y + (trunk_height * (i ) / 5) - (square_size / 2);

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Place 2 squares in left arm
    for _ in 0..2 {
        let square_x = (trunk_x - arm_width) + (rng.next_u32() as u32 % (arm_width - square_size));
        let square_y = arm_y_left + (rng.next_u32() as u32 % (arm_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Place 2 squares in right arm
    for _ in 0..2 {
        let square_x = (trunk_x + trunk_width) + (rng.next_u32() as u32 % (arm_width - square_size));
        let square_y = arm_y_right + (rng.next_u32() as u32 % (arm_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Left tall rectangle
    for _ in 0..1 {
        let square_x = tall_rect_x + (rng.next_u32() as u32 % (tall_rect_width - square_size));
        let square_y = tall_rect_y + (rng.next_u32() as u32 % (tall_rect_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }

    // Right tall rectangle
    for _ in 0..1 {
        let square_x = right_tall_rect_x + (rng.next_u32() as u32 % (right_tall_rect_width - square_size));
        let square_y = right_tall_rect_y + (rng.next_u32() as u32 % (right_tall_rect_height - square_size));

        for y in square_y..square_y + square_size {
            for x in square_x..square_x + square_size {
                let index = (y * width + x) as usize * 4;
                image.data[index..index + 4].copy_from_slice(&orange);
            }
        }
    }
    image
}
