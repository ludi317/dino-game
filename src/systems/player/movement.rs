use crate::components::{AnimationIndices, AnimationTimer, CactusArm, OriginalSize, Player, Velocity};
use crate::constants::GROUND_LEVEL;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::math::Vec2;
use bevy::prelude::{EventReader, KeyCode, Or, Query, Res, Sprite, Time, Touches, Transform, With};

const JUMP_FORCE: f32 = 2000.0;
const GRAVITY: f32 = -4000.0;

pub fn player_movement(
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
    query2: Query<&Transform, With<Player>>,
) {
    let player_transform = query2.single();
    if player_transform.translation.y > GROUND_LEVEL {
        return;
    }
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, Or<(With<Player>, With<CactusArm>)>>) {
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
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
            if e.state.is_pressed()
                && (e.key_code == KeyCode::Space || e.key_code == KeyCode::ArrowUp)
                && transform.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
    for _touch in touches.iter_just_pressed(){
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
                if transform.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

pub fn crouch(
    mut events: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut Sprite, &OriginalSize), With<Player>>,
) {
    for e in events.read() {
        if let Ok((mut sprite, original_size)) = player_query.get_single_mut() {
            if e.state.is_pressed() && e.key_code == KeyCode::ArrowDown {
                // Reduce the player's height to half its original size
                let new_height = original_size.0.y / 2.0;
                if let Some(size) = sprite.custom_size {
                    if size.y > new_height {
                        sprite.custom_size = Some(Vec2::new(size.x, new_height));
                    }
                }
            } else if e.state == ButtonState::Released && e.key_code == KeyCode::ArrowDown {
                sprite.custom_size = Some(original_size.0);
            }
        }
    }
}
