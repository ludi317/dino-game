use crate::components::{
    AnimationIndices, AnimationTimer, Collider, Health, HealthInfo, Layer, Player, PlayerCollider,
     Velocity,
};
use crate::constants::{DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y, DINO_RUN_SIZE, GAME_SPEED, GROUND_LEVEL, HIT_BOX_SCALE_X, INITIAL_HEALTH, PTERO_TIMER_INTERVAL};
use crate::resources::{
    CactusTexture, DinoDash, DinoRun, HealthPickUpImg, PterodactylDie, PterodactylFly,
};
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite, TextureAtlas, TextureAtlasLayout};

const PLAYER_X: f32 = -300.0;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    commands.spawn(Camera2d::default());
    let dino_run = asset_server.load("purple_trex_run.png");
    commands.insert_resource(DinoRun(dino_run.clone()));
    commands.insert_resource(HealthPickUpImg(
        asset_server.load("chocolate_icing_chocolate_drizzle.png"),
    ));
    commands.insert_resource(CactusTexture(asset_server.load("cactus texture.png")));
    commands.insert_resource(PterodactylFly(
        asset_server.load("blue_pterodactyl_flying.png"),
    ));
    commands.insert_resource(PterodactylDie(
        asset_server.load("blue_pterodactyl_die.png"),
    ));
    commands.insert_resource(DinoDash(asset_server.load("purple_trex_dash.png")));

    let texture = dino_run;
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y),
        4,
        4,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Player
    commands
        .spawn((
            Player,
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                }),
                flip_x: true,
                custom_size: Some(DINO_RUN_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 1.0),
            Velocity(Vec3::ZERO),
            AnimationIndices { first: 0, last: 15 },
            AnimationTimer(Timer::from_seconds(
                PTERO_TIMER_INTERVAL,
                TimerMode::Repeating,
            )),
        ))
        .with_children(|player| {
            player.spawn((
                PlayerCollider,
                Collider {
                    size: Vec2::new(DINO_RUN_SIZE.x * HIT_BOX_SCALE_X, DINO_RUN_SIZE.y),
                },
                Transform::from_xyz(
                    DINO_RUN_SIZE.x * (1. - HIT_BOX_SCALE_X) / 2.,
                    DINO_RUN_SIZE.y / 2.,
                    0.0,
                ),
                Health(INITIAL_HEALTH),
            ));
        });



    commands.spawn((HealthInfo, Text::new(format!("Health: {}", INITIAL_HEALTH))));
}
