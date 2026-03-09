use bevy::prelude::*;

#[derive(Component)]
pub struct CameraStruct;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        CameraStruct,
    ));
}
