mod components;
mod constants;
mod resources;
mod states;
mod systems {
    pub mod camera;
    pub mod game {
        pub mod end;
        pub mod pause;
        pub mod setup;
    }
    pub mod player {
        pub mod health;
        pub mod movement;
    }
    pub mod obstacles {
        pub mod cactus;
        pub mod collision;
        pub mod movement;
    }
}

use crate::resources::{AnimationState, ObstacleSpawningTimer};
use crate::states::GameState;
use crate::states::GameState::{GameOver, InGame};
use crate::systems::camera::{initialize_camera_system, move_camera_system};
use crate::systems::game::end::{game_over, restart_game};
use crate::systems::game::pause::{hide_pause_text, show_pause_text, toggle_pause};
use crate::systems::game::setup::setup;
use crate::systems::obstacles::collision::{debug_collider_outlines, detect_collision};
use crate::systems::obstacles::movement::{move_ground, move_obstacles, move_obstacles_y, spawn_obstacles};
use crate::systems::player::health::{check_health, render_health_info};
use crate::systems::player::movement::{
    animate_sprite, apply_gravity, crouch, jump, player_movement,
};

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_parallax::{ParallaxPlugin, ParallaxSystems};
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

const SPAWN_INTERVAL: f32 = 0.5;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::constants::GAME_SPEED;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    main();
}

fn main() {
    let primary_window = Window {
        title: "Dino Runner".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..default()
    };
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(ParallaxPlugin)
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        // Sand foreground
        .insert_resource(AnimationState {
            current: 1920.0, // sand foreground png width
            speed: GAME_SPEED * 2.0
        })
        .insert_state(InGame)
        .add_systems(Startup, (setup, initialize_camera_system))
        .add_systems(Update, toggle_pause)
        .add_systems(OnEnter(GameState::Paused), show_pause_text)
        .add_systems(OnExit(GameState::Paused), hide_pause_text)
        .add_systems(
            Update,
            (
                spawn_obstacles,
                move_obstacles,
                move_obstacles_y,
                detect_collision,
                render_health_info,
                check_health,
                animate_sprite,
                move_camera_system.before(ParallaxSystems),
                jump,
                apply_gravity,
                player_movement,
                crouch,
                move_ground,
            )
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver)))
        .add_systems(Update, debug_collider_outlines)
        .run();
}

