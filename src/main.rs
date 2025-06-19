mod components;
mod constants;
mod resources;
mod states;
mod systems {
    pub mod background;
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

use crate::constants::WINDOW_WIDTH;
use crate::resources::{ObstacleSpawningTimer, RealTimer, ScoreOffset};
use crate::states::GameState::{GameOver, InGame};
use crate::systems::background::{initialize_background, scroll_background};
use crate::systems::game::end::{game_over, restart_game};
use crate::systems::game::pause::toggle_pause;
use crate::systems::game::setup::setup;
#[allow(unused_imports)]
use crate::systems::obstacles::collision::{debug_outlines, detect_collision};
use crate::systems::obstacles::movement::{
    drop_obstacles, move_ground_obstacles, move_sky_obstacles, spawn_obstacles,
};
use crate::systems::player::health::{check_health, render_health_info, render_score_info};
use crate::systems::player::movement::{animate_sprite, apply_gravity, change_time_speed, crouch, drop_player, jump};

use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

#[cfg(debug_assertions)] // Development mode
const SPAWN_INTERVAL: f32 = 1.5;

#[cfg(not(debug_assertions))] // Release mode
const SPAWN_INTERVAL: f32 = 1.5;

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
        title: "Dino Runner".to_string(),
        resolution: (WINDOW_WIDTH, 720.0).into(),
        resizable: false,
        ..default()
    };
    let mut binding = App::new();
    let mut app = binding
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
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_resource(RealTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_resource(ScoreOffset(0.0))
        .insert_state(InGame)
        .add_systems(Startup, (setup, initialize_background))
        .add_systems(
            Update,
            (
                spawn_obstacles,
                move_ground_obstacles,
                move_sky_obstacles,
                drop_obstacles,
                detect_collision,
                render_health_info,
                check_health,
                animate_sprite,
                jump,
                apply_gravity,
                drop_player,
                crouch,
                scroll_background,
                toggle_pause.run_if(input_just_pressed(KeyCode::KeyP)),
                change_time_speed,
                render_score_info,
            )
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver)));

    setup_debug_systems(&mut app);
    app.run();
}

fn setup_debug_systems(app: &mut App) -> &mut App {
    #[cfg(debug_assertions)]
    {
        app.add_systems(Update, debug_outlines);
    }
    app
}
