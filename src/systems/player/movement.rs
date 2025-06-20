use crate::components::{
    AnimationIndices, AnimationTimer, Collider, Player, PlayerCollider, Velocity,
};
use crate::constants::{DINO_DIE_SIZE, DINO_DUCK_SIZE, DINO_JUMP_SIZE, DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y, DINO_RUN_SIZE, GROUND_LEVEL, HIT_BOX_SCALE_X};
use crate::resources::{DinoDuck, DinoJump, DinoRun, RealTimer};
use crate::states::GameState;
use crate::states::GameState::GameOver;
use crate::systems::player::animation::{animate_duck, animate_jump, animate_run};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::*;

const JUMP_FORCE: f32 = 1900.0;
const GRAVITY: f32 = -4000.0;
const MAX_REL_TIME: f32 = 3.0;

#[cfg(debug_assertions)] // Development mode
const REL_TIME_INCR: f32 = 0.02;

#[cfg(not(debug_assertions))] // Release mode
const REL_TIME_INCR: f32 = 0.02;

pub fn drop_player(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut Sprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
    mut player_collider: Query<&mut Collider, With<PlayerCollider>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut dino_run: Res<DinoRun>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (mut transform, mut velocity, mut sprite, mut anim_indices, mut anim_timer) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
            // back to running if jumping
            if sprite.custom_size == Some(DINO_JUMP_SIZE) {

                let mut collider = player_collider.single_mut().unwrap();
                collider.size = Vec2::new(DINO_RUN_SIZE.x * HIT_BOX_SCALE_X, DINO_RUN_SIZE.y);

                animate_run(&mut dino_run, &mut sprite, &mut anim_indices, &mut anim_timer, &mut texture_atlas_layouts, 4);
                // not touching the transforms
            } else if sprite.custom_size == Some(DINO_DIE_SIZE) {
                if let Some(atlas) = &sprite.texture_atlas {
                    if atlas.index >= 4 {
                        // dino dead on the ground, set game state to game over
                        game_state.set(GameOver);
                    }
                }
            }
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

pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

pub fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<
        (
            &mut Velocity,
            &Transform,
            &mut Sprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
    touches: Res<Touches>,
    mut dino_jump: Res<DinoJump>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_collider: Query<&mut Collider, With<PlayerCollider>>,
    time: Res<Time<Virtual>>,
) {
    for e in events.read() {
        if let Ok((mut velocity, transform, mut sprite, mut anim_indices, mut anim_timer)) =
            query.single_mut()
        {
            if e.state.is_pressed()
                && (e.key_code == KeyCode::Space || e.key_code == KeyCode::ArrowUp)
                && transform.translation.y <= GROUND_LEVEL
                && !time.is_paused()
            {
                velocity.0.y = JUMP_FORCE;
                let mut collider = player_collider.single_mut().unwrap();
                animate_jump(&mut dino_jump, &mut sprite, &mut anim_indices, &mut anim_timer, &mut texture_atlas_layouts, &mut collider);
            }
        }
    }
    for _touch in touches.iter_just_pressed() {
        if let Ok((mut velocity, transform, mut sprite, mut anim_indices, mut anim_timer)) =
            query.single_mut()
        {
            if transform.translation.y <= GROUND_LEVEL {
                velocity.0.y = JUMP_FORCE;
                let mut collider = player_collider.single_mut().unwrap();
                animate_jump(&mut dino_jump, &mut sprite, &mut anim_indices, &mut anim_timer, &mut texture_atlas_layouts, &mut collider);

            }
        }
    }
}

pub fn duck(
    mut events: EventReader<KeyboardInput>,
    mut player_query: Query<&mut Sprite, With<Player>>,
    mut player_collider: Query<(&mut Collider, &mut Transform), With<PlayerCollider>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    dino_run: Res<DinoRun>,
    mut dino_duck: Res<DinoDuck>,
) {
    for e in events.read() {
        if e.key_code == KeyCode::ArrowDown && e.state == ButtonState::Pressed {
            let mut sprite = player_query.single_mut().unwrap();
            // switch to ducking if not already
            if sprite.custom_size != Some(DINO_DUCK_SIZE) {
                let (mut collider, mut transform) = player_collider.single_mut().unwrap();
                animate_duck(&mut dino_duck, &mut sprite, &mut texture_atlas_layouts, &mut collider, &mut transform);

            }
        } else if e.key_code == KeyCode::ArrowDown && e.state == ButtonState::Released {
            let mut sprite = player_query.single_mut().unwrap();
            let (mut collider, mut transform) = player_collider.single_mut().unwrap();

            // back to running
            let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y), 4, 4, None, None);
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

pub fn change_time_speed(
    mut time_virtual: ResMut<Time<Virtual>>,
    time_fixed: ResMut<Time<Fixed>>,
    mut timer: ResMut<RealTimer>,
) {
    if !time_virtual.is_paused() {
        timer.0.tick(time_fixed.delta());

        if timer.0.finished() {
            let rel_speed = (time_virtual.relative_speed() + REL_TIME_INCR).min(MAX_REL_TIME);
            time_virtual.set_relative_speed(rel_speed);
        }
    }
}
