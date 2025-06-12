use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::sprite::{Anchor, Sprite, TextureAtlas, TextureAtlasLayout};
use crate::components::{AnimationIndices, AnimationTimer, Health, HealthInfo, OriginalSize, Player, Velocity};
use crate::constants::{GROUND_LEVEL, INITIAL_HEALTH, PLAYER_SIZE};
const PLAYER_X: f32 = -300.0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    let texture = asset_server.load("dino run 1-10.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1328, 768), 10, 1, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 9 };

    // Player
    commands.spawn((
        Player,
        Sprite{
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }),
            custom_size: Some(PLAYER_SIZE),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
        Health(INITIAL_HEALTH),
        OriginalSize(PLAYER_SIZE),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.0875, TimerMode::Repeating)),

    ));

    commands.spawn((HealthInfo, Text::new(format!("Health: {}", INITIAL_HEALTH))));

}
