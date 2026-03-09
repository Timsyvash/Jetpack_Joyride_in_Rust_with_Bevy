use bevy::prelude::*;

#[derive(Component)]
pub struct RocketsStruct {
    pub speed: f32,
    pub direction: Vec2,
}

pub fn move_rockets(mut query: Query<(&mut Transform, &RocketsStruct)>,
                    time: Res<Time>) {
    for (mut transform, rocket) in query.iter_mut() {
        transform.translation.x += rocket.direction.x * rocket.speed * time.delta_secs();
        transform.translation.y += rocket.direction.y * rocket.speed * time.delta_secs();
    }
}

pub fn borders_rockets(mut query: Query<&mut Transform, With<RocketsStruct>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y > 150.0 {
            transform.translation.y = 150.0;
        } else if transform.translation.y < -150.0 {
            transform.translation.y = -150.0;
        }
    }
}

pub fn despawn_off_screen_rockets(
    mut commands: Commands,
    rockets_query: Query<(Entity, &Transform), With<RocketsStruct>>,
) {
    for (entity, transform) in rockets_query.iter() {
        if transform.translation.x < -400.0 || transform.translation.x > 400.0 {
            commands.entity(entity).despawn();
        }
    }
}