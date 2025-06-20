use std::time::Duration;
use bevy::prelude::*;
use crate::components::{AnimationIndices, AnimationTimer, Collider};
use crate::constants::{DIE_ANIMATION_TIMER_INTERVAL, DINO_DIE_IMG_SIZE_X, DINO_DIE_IMG_SIZE_Y, DINO_DIE_SIZE, DINO_DUCK_IMG_SIZE_X, DINO_DUCK_IMG_SIZE_Y, DINO_DUCK_SIZE, DINO_JUMP_IMG_SIZE_X, DINO_JUMP_IMG_SIZE_Y, DINO_JUMP_SIZE, DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y, DINO_RUN_SIZE, HIT_BOX_SCALE_X, HIT_BOX_SCALE_Y, JUMP_ANIMATION_TIMER_INTERVAL, RUN_ANIMATION_TIMER_INTERVAL};
use crate::resources::{DinoDie, DinoDuck, DinoJump, DinoRun};

pub fn animate_die(dino_die: &mut Res<DinoDie>, sprite: &mut Sprite, anim_indices: &mut AnimationIndices, anim_timer: &mut AnimationTimer,
                   texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_DIE_IMG_SIZE_X, DINO_DIE_IMG_SIZE_Y), 4, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    sprite.image = dino_die.0.clone();
    sprite.custom_size = Some(DINO_DIE_SIZE);
    sprite.texture_atlas = Some(TextureAtlas {
        layout: texture_atlas_layout,
        index: 0,
    });
    anim_indices.first = 0;
    anim_indices.last = 7;
    anim_timer.0.set_duration(Duration::from_secs_f32(DIE_ANIMATION_TIMER_INTERVAL));
}

pub fn animate_run(dino_run: &mut Res<DinoRun>, sprite: &mut Sprite, anim_indices: &mut AnimationIndices, anim_timer: &mut AnimationTimer,
                   texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
                   start_idx: usize) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_RUN_IMG_SIZE_X, DINO_RUN_IMG_SIZE_Y), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    sprite.image = dino_run.0.clone();
    sprite.custom_size = Some(DINO_RUN_SIZE);
    sprite.texture_atlas = Some(TextureAtlas {
        layout: texture_atlas_layout,
        index: start_idx,
    });
    anim_indices.first = 0;
    anim_indices.last = 15;
    anim_timer.0.set_duration(Duration::from_secs_f32(RUN_ANIMATION_TIMER_INTERVAL));
}

pub fn animate_jump(dino_jump: &mut Res<DinoJump>, sprite: &mut Sprite, anim_indices: &mut AnimationIndices, anim_timer: &mut AnimationTimer,
                    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>, collider: &mut Collider) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_JUMP_IMG_SIZE_X, DINO_JUMP_IMG_SIZE_Y), 4, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let first = 3;
    sprite.image = dino_jump.0.clone();
    sprite.custom_size = Some(DINO_JUMP_SIZE);
    sprite.texture_atlas = Some(TextureAtlas {
        layout: texture_atlas_layout,
        index: first,
    });
    anim_indices.first = first;
    anim_indices.last = 11;
    anim_timer.0.set_duration(Duration::from_secs_f32(JUMP_ANIMATION_TIMER_INTERVAL));
    collider.size = Vec2::new(
        DINO_JUMP_SIZE.x * HIT_BOX_SCALE_X,
        DINO_JUMP_SIZE.y,
    );
    // not touching the transforms
}

pub fn animate_duck(dino_duck: &mut Res<DinoDuck>, sprite: &mut Sprite, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
                    collider: &mut Collider, transform: &mut Transform) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(DINO_DUCK_IMG_SIZE_X, DINO_DUCK_IMG_SIZE_Y), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    sprite.image = dino_duck.0.clone();
    sprite.custom_size = Some(DINO_DUCK_SIZE);
    sprite.texture_atlas = Some(TextureAtlas {
        layout: texture_atlas_layout,
        index: 0,
    });
    collider.size = Vec2::new(
        DINO_DUCK_SIZE.x * HIT_BOX_SCALE_X,
        DINO_DUCK_SIZE.y * HIT_BOX_SCALE_Y,
    );
    transform.translation = Vec3::new(
        DINO_DUCK_SIZE.x * (1. - HIT_BOX_SCALE_X) / 2.,
        DINO_DUCK_SIZE.y * HIT_BOX_SCALE_Y / 2.,
        0.0,
    );
}