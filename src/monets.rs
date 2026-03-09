use bevy::prelude::*;
use crate::player::*;
use crate::game::*;
use rand::*;

#[derive(Component)]
pub struct MonetsStruct;

pub fn setup_monets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/monets/monets.png"),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(300.0, random::<f32>() * 200.0 - 100.0, 1.0),
        MonetsStruct,
    ));
}

pub fn spawn_monet(mut commands: Commands, asset_server: Res<AssetServer>,
                   mut timer: Local<Option<Timer>>, time: Res<Time>) {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(2.0, TimerMode::Repeating));

    timer.tick(time.delta());

    if timer.just_finished() {
        let mut rng = thread_rng();
        commands.spawn((
            Sprite {
                image: asset_server.load("images/monets/monets.png"),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(300.0, rng.gen_range(-140.0..140.0), 1.0),
            MonetsStruct,
        ));
    }
}

pub fn despawn_off_screen_monets(
    mut commands: Commands,
    monets_query: Query<(Entity, &Transform), With<MonetsStruct>>,
) {
    for (entity, transform) in monets_query.iter() {
        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_monets(mut query: Query<&mut Transform, With<MonetsStruct>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 100.0 * time.delta_secs();
    }
}

pub fn collision_player_with_monets(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite), With<PlayerStruct>>,
    mut monet_query: Query<(Entity, &Transform, &Sprite), With<MonetsStruct>>,
    mut score: ResMut<Score>,
) {
    let Ok((player_transform, player_sprite)) = player_query.single_mut() else {
        return;
    };

    let player_size = player_sprite.custom_size.unwrap_or(Vec2::new(50.0, 50.0));

    for (monet_entity, monet_transform, monet_sprite) in monet_query.iter_mut() {
        let monet_size = monet_sprite.custom_size.unwrap_or(Vec2::new(30.0, 30.0));

        let collision = collide(
            player_transform.translation,
            player_size,
            monet_transform.translation,
            monet_size,
        );

        if collision.is_some() {
            commands.entity(monet_entity).despawn();
            score.value += 1;
        }
    }
}

fn collide(
    pos_a: Vec3,
    size_a: Vec2,
    pos_b: Vec3,
    size_b: Vec2,
) -> Option<()> {
    let collision_x = (pos_a.x - size_a.x / 2.0) < (pos_b.x + size_b.x / 2.0)
        && (pos_a.x + size_a.x / 2.0) > (pos_b.x - size_b.x / 2.0);
    let collision_y = (pos_a.y - size_a.y / 2.0) < (pos_b.y + size_b.y / 2.0)
        && (pos_a.y + size_a.y / 2.0) > (pos_b.y - size_b.y / 2.0);

    if collision_x && collision_y {
        Some(())
    } else {
        None
    }
}