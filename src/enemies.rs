use bevy::prelude::*;
use rand::*;
use crate::rocket::*;

#[derive(Component)]
pub struct EnemiesStruct;

pub fn setup_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/players/Robo_Barry/Robo_Barry.png"),
            ..default()
        },
        Transform::from_xyz(300.0, 0.0, 1.0),
        EnemiesStruct,
    ));
}

pub fn move_enemies(mut query: Query<&mut Transform, With<EnemiesStruct>>,
                        time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 50.0 * time.delta_secs();
        let mut rng = thread_rng();
        if transform.translation.x < -350.0 {
            transform.translation.x = 350.0;
            let random_y: f32 = rng.gen_range(-150.0..150.0);
            transform.translation.y = random_y;
        }
    }
}

pub fn shoot_enemies(mut commands: Commands, query: Query<&Transform, With<EnemiesStruct>>,
                    asset_server: Res<AssetServer>) {
    for transform in query.iter() {
        let mut rng = thread_rng();
        if rng.gen_bool(0.01) {
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/rockets/rocket.png"),
                    ..default()
                },
                Transform::from_xyz(transform.translation.x, transform.translation.y, 1.0),
                RocketsStruct {
                    speed: 200.0,
                    direction: Vec2::new(-1.0, 0.0),
                },
            ));
        }
    }
}