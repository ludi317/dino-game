use bevy::asset::AssetServer;
use crate::components::Layer;
use crate::constants::{GAME_SPEED, GROUND_LEVEL, WINDOW_WIDTH};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const IMG_SIZE_X: f32 = 1920.0;
const IMG_SIZE_Y: f32 = 1080.0;

// https://bevy.org/examples/2d-rendering/sprite-tile/
pub fn scroll_background(
    mut layers: Query<(&mut Sprite, &mut Layer)>,
    time: Res<Time>,
) {
    for ((mut sprite, mut layer)) in layers.iter_mut() {
        layer.current_size_x += layer.speed_scale * 2.0 * GAME_SPEED * time.delta_secs();
        if layer.current_size_x >= 2.0 * (IMG_SIZE_X + WINDOW_WIDTH) {
            layer.current_size_x = (layer.current_size_x % IMG_SIZE_X) + IMG_SIZE_X;
        }
        sprite.custom_size = Some(Vec2::new(layer.current_size_x, IMG_SIZE_Y));
    }
}

pub fn initialize_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Ground
    commands.spawn((
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
        Layer {
            current_size_x: IMG_SIZE_X,
            speed_scale: 1.0,
        },
        Transform::from_xyz(0.0, GROUND_LEVEL - 150.0, -1.0),
    ));
}