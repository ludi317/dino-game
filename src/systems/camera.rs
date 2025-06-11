use crate::components::{HealthPickup, Obstacle, Player};
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{default, Camera, Camera2d, Commands, Entity, EventWriter, Or, Query, Transform, With};
use bevy_parallax::{CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent};

pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut transforms: Query<&mut Transform, Or<(With<Player>, With<Obstacle>, With<HealthPickup>)>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.single();

    move_event_writer.send(ParallaxMoveEvent {
        translation: Vec2::new(3.0, 0.0),
        rotation: 0.0,
        camera,
    });

    for mut transform in &mut transforms {
        transform.translation.x += 3.0;
    }
}

pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>
) {
    let camera = commands
        .spawn(Camera2d::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    let event = CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "9 Background.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -9.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "8 Stars.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -8.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "7 Clouds.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -7.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(1.0),
                path: "6 Sun.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -6.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.7),
                path: "5 Mountains.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -5.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.5),
                path: "4 Layer4.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -4.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.4),
                path: "3 Layer3.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -3.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.3),
                path: "2 Layer2.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -2.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.2),
                path: "sand3.png".to_string(),
                tile_size: UVec2::new(1920, 1080),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(1.0),
                z: -1.0,
                ..default()
            },
        ],
        camera: camera,
    };
    create_parallax.send(event);
}