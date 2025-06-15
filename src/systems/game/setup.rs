use crate::components::{AnimationIndices, AnimationTimer, Collider, Health, HealthInfo, OriginalSize, Player, Sand, Velocity};
use crate::constants::{GROUND_LEVEL, INITIAL_HEALTH, PLAYER_SIZE};
use crate::resources::{CactusTexture, Cheeseburger};
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite, TextureAtlas, TextureAtlasLayout};

const PLAYER_X: f32 = -300.0;
const HIT_BOX_SCALE_X: f32 = 0.67;

pub fn setup(mut commands: Commands,
             asset_server: Res<AssetServer>,
             mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    let texture = asset_server.load("dino run 1-10.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1328, 768), 10, 1, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 9 };

    commands.insert_resource(Cheeseburger(asset_server.load("cheeseburger.png")));
    commands.insert_resource(CactusTexture(asset_server.load("cactus texture.png")));

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL+50., 1.0),
        Velocity(Vec3::ZERO),
        Health(INITIAL_HEALTH),
        OriginalSize(PLAYER_SIZE),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.0875, TimerMode::Repeating)),
    )).with_children(|player| {
        player.spawn((
            Collider {
                size: Vec2::new(PLAYER_SIZE.x * HIT_BOX_SCALE_X, PLAYER_SIZE.y),
            },
            Transform::from_xyz(PLAYER_SIZE.x * (1. - HIT_BOX_SCALE_X) * 0.5, 0.0, 0.0),
        ));
    });

    // Ground
    commands.spawn((
        Sand,
        Sprite {
            image: asset_server.load("sand3.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, GROUND_LEVEL - 150.0, -1.0),

    ));

    commands.spawn((HealthInfo, Text::new(format!("Health: {}", INITIAL_HEALTH))));
}
