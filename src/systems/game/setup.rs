use crate::components::{AnimationIndices, AnimationTimer, Collider, Health, HealthInfo, OriginalSize, Player, Sand, Velocity};
use crate::constants::{GROUND_LEVEL, INITIAL_HEALTH, PTERO_TIMER_INTERVAL};
use crate::resources::{CactusTexture, HealthPickUpImg, PterodactylDie, PterodactylFly};
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite, TextureAtlas, TextureAtlasLayout};

const PLAYER_SIZE_X: u32 = 939;
const PLAYER_SIZE_Y: u32 = 668;
const PLAYER_SCALE: f32 = 200./ PLAYER_SIZE_X as f32;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_SIZE_X as f32 * PLAYER_SCALE, PLAYER_SIZE_Y as f32 * PLAYER_SCALE);
const HIT_BOX_SCALE_X: f32 = 0.67;
const PLAYER_X: f32 = -300.0;

pub fn setup(mut commands: Commands,
             asset_server: Res<AssetServer>,
             mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    let texture = asset_server.load("purple_trex_run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(PLAYER_SIZE_X, PLAYER_SIZE_Y), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(HealthPickUpImg(asset_server.load("chocolate_icing_chocolate_drizzle.png")));
    commands.insert_resource(CactusTexture(asset_server.load("cactus texture.png")));
    commands.insert_resource(PterodactylFly(asset_server.load("blue_pterodactyl_flying.png")));
    commands.insert_resource(PterodactylDie(asset_server.load("blue_pterodactyl_die.png")));

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            flip_x: true,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL+50., 1.0),
        Velocity(Vec3::ZERO),
        Health(INITIAL_HEALTH),
        OriginalSize(PLAYER_SIZE),
        AnimationIndices { first: 0, last: 15 },
        AnimationTimer(Timer::from_seconds(PTERO_TIMER_INTERVAL, TimerMode::Repeating)),
    )).with_children(|player| {
        player.spawn((
            Collider {
                size: Vec2::new( PLAYER_SIZE.x * HIT_BOX_SCALE_X, PLAYER_SIZE.y),
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
