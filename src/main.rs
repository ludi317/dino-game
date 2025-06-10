mod resources;
mod states;
mod constants;
mod components;
mod systems {
    pub mod camera;
    pub mod collision;
    pub mod game {
        pub mod setup;
        pub mod pause;
        pub mod end;
    }
    pub mod player {
        pub mod movement;
        pub mod health;
    }
    pub mod obstacles {
        pub mod movement;
        pub mod cactus;
    }
}

use crate::constants::SPAWN_INTERVAL;
use crate::resources::ObstacleSpawningTimer;
use crate::states::GameState;
use crate::states::GameState::{GameOver, InGame};
use crate::systems::camera::{initialize_camera_system, move_camera_system};
use crate::systems::game::pause::{hide_pause_text, show_pause_text, toggle_pause};
use crate::systems::game::setup::setup;
use crate::systems::collision::detect_collision;
use crate::systems::game::end::{game_over, restart_game};
use crate::systems::obstacles::movement::{move_obstacles, spawn_obstacles};
use crate::systems::player::health::{check_health, render_health_info};
use crate::systems::player::movement::{animate_sprite, apply_gravity, crouch, jump, player_movement};

use bevy::prelude::*;
use bevy_parallax::{ParallaxPlugin, ParallaxSystems};
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::RngCore;



#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    main();
}

fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
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
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParallaxPlugin)
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_state(InGame)
        .add_systems(Startup, (setup, initialize_camera_system))
        .add_systems(Update, toggle_pause)
        .add_systems(OnEnter(GameState::Paused), show_pause_text)
        .add_systems(OnExit(GameState::Paused), hide_pause_text)
        .add_systems(Update,
            (spawn_obstacles, move_obstacles, detect_collision, render_health_info, check_health,
             animate_sprite, move_camera_system.before(ParallaxSystems), jump, apply_gravity, player_movement, crouch)
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver)))
        .run();
}
