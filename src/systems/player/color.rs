use crate::components::Player;
use crate::constants::{DINO_DIE_SIZE, DINO_DUCK_SIZE, DINO_JUMP_SIZE};
use crate::resources::{DinoDie, DinoDuck, DinoJump, DinoRun, GameSettings};
use bevy::prelude::*;

/// Points the dino sprite resources at the selected color's sheets, and swaps
/// the sheet the player is currently drawn with, keeping whatever pose it is in.
pub fn apply_dino_color(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    mut player_query: Query<&mut Sprite, With<Player>>,
) {
    let color = settings.dino_color;
    let run: Handle<Image> = asset_server.load(color.sprite_path("run"));
    let duck = asset_server.load(color.sprite_path("duck"));
    let jump = asset_server.load(color.sprite_path("jump"));
    let die = asset_server.load(color.sprite_path("die"));

    if let Ok(mut sprite) = player_query.single_mut() {
        sprite.image = if sprite.custom_size == Some(DINO_DUCK_SIZE) {
            duck.clone()
        } else if sprite.custom_size == Some(DINO_JUMP_SIZE) {
            jump.clone()
        } else if sprite.custom_size == Some(DINO_DIE_SIZE) {
            die.clone()
        } else {
            run.clone()
        };
    }

    commands.insert_resource(DinoRun(run));
    commands.insert_resource(DinoDuck(duck));
    commands.insert_resource(DinoJump(jump));
    commands.insert_resource(DinoDie(die));
}
