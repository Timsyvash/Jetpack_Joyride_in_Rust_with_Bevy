use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerStruct;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/players/Barry_Steakfries/Barry_Steakfries.png"),
                ..default()
        },
        Transform::from_xyz(-180.0, 0.0, 1.0),
        PlayerStruct,
    ));
}

pub fn gravity_for_player(mut query: Query<&mut Transform, With<PlayerStruct>>,
time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let gravity = -25.0;
        let back = -30.0;
        transform.translation.y += gravity * time.delta_secs();
        transform.translation.x += back * time.delta_secs();
    }
}

pub fn player_control(mut query: Query<(&mut Transform, &mut Sprite), With<PlayerStruct>>,
                      key_code: Res<ButtonInput<KeyCode>>, asset_server: Res<AssetServer>) {
    for (mut transform, mut sprite) in query.iter_mut() {
        if key_code.pressed(KeyCode::KeyW) || key_code.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 5.0;
            sprite.image = asset_server.load("images/players/Barry_Steakfries/Barry_Steakfries.png");
        }
        if key_code.pressed(KeyCode::KeyD) || key_code.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.0;
            sprite.image = asset_server.load("images/players/Barry_Steakfries/Barry_Steakfries_flies.png");
        }
    }
}

pub fn borders_for_player(mut query: Query<&mut Transform, With<PlayerStruct>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y < -160.0 {
            transform.translation.y = -160.0;
        }
        if transform.translation.y > 160.0 {
            transform.translation.y = 160.0;
        }
        if transform.translation.x < -320.0 {
            transform.translation.x = -320.0;
        }
        if transform.translation.x > 320.0 {
            transform.translation.x = 320.0;
        }
    }
}