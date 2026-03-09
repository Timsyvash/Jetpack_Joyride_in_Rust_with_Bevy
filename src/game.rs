use bevy::prelude::*;
use crate::enemies::*;
use crate::player::*;
use crate::rocket::*;
use crate::monets::*;

#[derive(Resource, States, Copy, Hash, PartialEq, Eq, Clone, Debug, Default)]
pub enum GameStates {
    #[default]
    NotStarted,
    InGame,
    GameOver,
    Paused,
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct GameStartTextStruct;

#[derive(Component)]
pub struct GameOverTextStruct;

#[derive(Component)]
pub struct PauseTextStruct;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

const BG_WIDTH: f32 = 700.0;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..2 {
        commands.spawn((
            Sprite {
                image: asset_server.load("images/backgrounds/background_for_jetpack_joyride_game.png"),
                ..default()
            },
            Transform::from_xyz(BG_WIDTH * i as f32, 0.0, 0.0),
            Background,
        ));
    }
}

pub fn scroll_background(mut query: Query<&mut Transform, With<Background>>,
                     time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 100.0 * time.delta_secs();
        if transform.translation.x < -BG_WIDTH {
            transform.translation.x += BG_WIDTH * 2.0;
        }
    }
}

pub fn collision_player_with_rockets(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<PlayerStruct>>,
    rocket_query: Query<(Entity, &Transform), With<RocketsStruct>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single() {
        for (rocket_entity, rocket_transform) in rocket_query.iter() {
            let distance = player_transform.translation.distance(rocket_transform.translation);
            if distance < 40.0 {
                commands.entity(player_entity).despawn();
                commands.entity(rocket_entity).despawn();
                next_state.set(GameStates::GameOver);
                return;
            }
        }
    }
}

pub fn collision_player_with_enemies(
    mut commands: Commands,
    player_query: Query<(&Transform, Entity), With<PlayerStruct>>,
    enemies_query: Query<(Entity, &Transform), With<EnemiesStruct>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    if let Ok((player_transform, player_entity)) = player_query.single() {
        for (enemies_entity, enemies_transform) in enemies_query.iter() {
            let distance = player_transform.translation.distance(enemies_transform.translation);
            if distance < 55.0 {
                commands.entity(player_entity).despawn();
                commands.entity(enemies_entity).despawn();
                next_state.set(GameStates::GameOver);
                return;
            }
        }
    }
}

pub fn start_game(
    mut commands: Commands,
    state: Res<State<GameStates>>,
    key_code: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
    asset_server: Res<AssetServer>,
    text_query: Query<Entity, With<GameStartTextStruct>>,
    existing: Query<(), With<GameStartTextStruct>>,
    mut game_objects: Query<&mut Visibility, Or<(
        With<PlayerStruct>, With<RocketsStruct>,
        With<EnemiesStruct>, With<MonetsStruct>)>>,
) {
    if *state.get() == GameStates::NotStarted && existing.is_empty() {
        for mut visibility in game_objects.iter_mut() {
            *visibility = Visibility::Hidden;
        }

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Натисніть на Q для початку гри"),
                TextFont {
                    font: asset_server.load("fonts/Arsenal-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
                GameStartTextStruct,
            ));
        });
    }

    if key_code.just_pressed(KeyCode::KeyQ) && *state.get() == GameStates::NotStarted {
        for mut visibility in game_objects.iter_mut() {
            *visibility = Visibility::Visible;
        }

        next_state.set(GameStates::InGame);

        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<(), With<GameOverTextStruct>>,
    mut game_objects: Query<&mut Visibility, Or<(
        With<PlayerStruct>,
        With<RocketsStruct>,
        With<EnemiesStruct>,
        With<MonetsStruct>,
    )>>,
) {
    for mut visibility in game_objects.iter_mut() {
        *visibility = Visibility::Hidden;
    }

    if existing.is_empty() {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameOverTextStruct,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Гра закінчена! Натисніть R для нової гри"),
                TextFont {
                    font: asset_server.load("fonts/Arsenal-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor::BLACK,
            ));
        });
    }
}

pub fn restart_game(
    mut commands: Commands,
    key_code: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
    asset_server: Res<AssetServer>,
    text_query: Query<Entity, With<GameOverTextStruct>>,
    rockets_query: Query<Entity, With<RocketsStruct>>,
    enemies_query: Query<Entity, With<EnemiesStruct>>,
    monets_query: Query<Entity, With<MonetsStruct>>,
    mut score: ResMut<Score>,
) {
    if key_code.just_pressed(KeyCode::KeyR) {
        next_state.set(GameStates::InGame);

        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }

        for entity in rockets_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in enemies_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in monets_query.iter() {
            commands.entity(entity).despawn();
        }

        score.value = 0;

        commands.spawn((
            Sprite {
                image: asset_server.load("images/players/Barry_Steakfries/Barry_Steakfries.png"),
                ..default()
            },
            Transform::from_xyz(-300.0, 0.0, 1.0),
            PlayerStruct,
        ));

        commands.spawn((
            Sprite {
                image: asset_server.load("images/players/Robo_Barry/Robo_Barry.png"),
                ..default()
            },
            Transform::from_xyz(300.0, 0.0, 1.0),
            EnemiesStruct,
        ));

        commands.spawn((
            Sprite {
                image: asset_server.load("images/monets/monets.png"),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(350.0, rand::random::<f32>() * 200.0 - 100.0, 1.0),
            MonetsStruct,
        ));
    }
}

pub fn pause_game(
    mut commands: Commands,
    state: Res<State<GameStates>>,
    key_code: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStates>>,
    asset_server: Res<AssetServer>,
    text_query: Query<Entity, With<PauseTextStruct>>,
    mut game_objects: Query<&mut Visibility, Or<(
        With<PlayerStruct>,
        With<RocketsStruct>,
        With<EnemiesStruct>,
        With<MonetsStruct>,
    )>>,
) {
    if key_code.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GameStates::InGame => {
                next_state.set(GameStates::Paused);

                for mut visibility in game_objects.iter_mut() {
                    *visibility = Visibility::Hidden;
                }

                spawn_pause_text(&mut commands, &asset_server);
            }

            GameStates::Paused => {
                next_state.set(GameStates::InGame);

                for mut visibility in game_objects.iter_mut() {
                    *visibility = Visibility::Visible;
                }
                for entity in text_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

fn spawn_pause_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        PauseTextStruct,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Пауза! Натисніть P для продовження"),
            TextFont {
                font: asset_server.load("fonts/Arsenal-Bold.ttf"),
                font_size: 30.0,
                ..default()
            },
            TextColor::BLACK,
        ));
    });
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Очки: 0"),
        TextFont {
            font: asset_server.load("fonts/Arsenal-Bold.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreText,
    ));

    commands.insert_resource(Score { value: 0 });
}

pub fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        **text = format!("Очки: {}", score.value);
    }
}