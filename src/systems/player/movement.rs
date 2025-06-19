use crate::components::{
    AnimationIndices, AnimationTimer, CactusArm, Collider, Player, PlayerCollider, Pterodactyl,
    Velocity,
};
use crate::constants::{
    DINO_DASH_IMG_SIZE_X, DINO_DASH_IMG_SIZE_Y, DINO_DASH_SIZE, DINO_RUN_IMG_SIZE_X,
    DINO_RUN_IMG_SIZE_Y, DINO_RUN_SIZE, GROUND_LEVEL, HIT_BOX_SCALE_X, HIT_BOX_SCALE_Y,
};
use crate::resources::{DinoDash, DinoRun, RealTimer};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::log::tracing_subscriber::fmt::time;
use bevy::math::{UVec2, Vec2, VectorSpace};
use bevy::math::ops::log2;
use bevy::prelude::*;
use crate::SPAWN_INTERVAL;

const JUMP_FORCE: f32 = 2000.0;
const GRAVITY: f32 = -4000.0;
const MAX_REL_TIME: f32 = 3.0;
const REL_TIME_INCR: f32 = 0.02;

pub fn drop_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<&mut Velocity, Or<(With<Player>, With<CactusArm>, With<Pterodactyl>)>>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

pub fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    touches: Res<Touches>,
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) = query.single_mut() {
            if e.state.is_pressed()
                && (e.key_code == KeyCode::Space || e.key_code == KeyCode::ArrowUp)
                && transform.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
    for _touch in touches.iter_just_pressed() {
        if let Ok((mut velocity, transform)) = query.single_mut() {
            if transform.translation.y <= GROUND_LEVEL {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

pub fn crouch(
    mut events: EventReader<KeyboardInput>,
    mut player_query: Query<&mut Sprite, With<Player>>,
    mut player_collider: Query<(&mut Collider, &mut Transform), With<PlayerCollider>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    dino_run: Res<DinoRun>,
    dino_dash: Res<DinoDash>,
) {
    for e in events.read() {
        if e.key_code == KeyCode::ArrowDown && e.state == ButtonState::Pressed {
            let mut sprite = player_query.single_mut().unwrap();
            let (mut collider, mut transform) = player_collider.single_mut().unwrap();
            // switch to crouching if not already
            if sprite.custom_size != Some(DINO_DASH_SIZE) {
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(DINO_DASH_IMG_SIZE_X, DINO_DASH_IMG_SIZE_Y),
                    4,
                    4,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                sprite.image = dino_dash.0.clone();
                sprite.custom_size = Some(DINO_DASH_SIZE);
                sprite.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                });
                collider.size = Vec2::new(
                    DINO_DASH_SIZE.x * HIT_BOX_SCALE_X,
                    DINO_DASH_SIZE.y * HIT_BOX_SCALE_Y,
                );
                transform.translation = Vec3::new(
                    DINO_DASH_SIZE.x * (1. - HIT_BOX_SCALE_X) / 2.,
                    DINO_DASH_SIZE.y * HIT_BOX_SCALE_Y / 2.,
                    0.0,
                );
            }
        } else if e.key_code == KeyCode::ArrowDown && e.state == ButtonState::Released {
            let mut sprite = player_query.single_mut().unwrap();
            let (mut collider, mut transform) = player_collider.single_mut().unwrap();

            // back to running
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y),
                4,
                4,
                None,
                None,
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            sprite.image = dino_run.0.clone();
            sprite.custom_size = Some(DINO_RUN_SIZE);
            sprite.texture_atlas = Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            });
            collider.size = Vec2::new(DINO_RUN_SIZE.x * HIT_BOX_SCALE_X, DINO_RUN_SIZE.y);
            transform.translation = Vec3::new(
                DINO_RUN_SIZE.x * (1. - HIT_BOX_SCALE_X) / 2.,
                DINO_RUN_SIZE.y / 2.,
                0.0,
            );
        }
    }
}

pub fn change_time_speed(mut time_virtual: ResMut<Time<Virtual>>, time_fixed: ResMut<Time<Fixed>>, mut timer: ResMut<RealTimer>) {
    timer.0.tick(time_fixed.delta());

    if timer.0.finished() {
        let rel_speed = (time_virtual.relative_speed() + REL_TIME_INCR).min(MAX_REL_TIME);
        time_virtual.set_relative_speed(rel_speed);
    }
}