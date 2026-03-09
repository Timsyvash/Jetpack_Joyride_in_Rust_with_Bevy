mod game;
mod camera;
mod player;
mod enemies;
mod rocket;
mod monets;

use bevy::prelude::*;
use crate::camera::*;
use crate::enemies::*;
use crate::game::*;
use crate::monets::*;
use crate::player::*;
use crate::rocket::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jetpack Joyride".to_string(),
                resolution: (700, 392).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameStates>()
        .add_systems(Startup, (
            setup_game,
            setup_camera,
            setup_player,
            setup_enemies,
            setup_monets,
            setup_score,
        ))
        .add_systems(Update, (
            gravity_for_player,
            player_control,
            borders_for_player,
            move_enemies,
            move_rockets,
            scroll_background,
            shoot_enemies,
            borders_rockets,
            despawn_off_screen_rockets,
        ).run_if(in_state(GameStates::InGame)))
        .add_systems(Update, (
            collision_player_with_rockets,
            collision_player_with_enemies,
            move_monets,
            collision_player_with_monets,
            update_score,
            spawn_monet,
            despawn_off_screen_monets,
        ).run_if(in_state(GameStates::InGame)))
        .add_systems(Update, (
            start_game.run_if(in_state(GameStates::NotStarted)),
            game_over.run_if(in_state(GameStates::GameOver)),
            restart_game.run_if(in_state(GameStates::GameOver)),
            pause_game,
        ))
        .run();
}