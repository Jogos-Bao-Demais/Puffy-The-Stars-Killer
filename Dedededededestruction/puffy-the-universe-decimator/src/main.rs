use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 32.0;

pub const NUMBER_OF_ENEMIES: usize = 6;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            1.0
        ),
        ..Default::default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    0.0
                ),
                texture: asset_server.load("sprites/PuffyTheStarsKillerPlaceholder.png"),
                ..Default::default() // Set all other field to default
            },
            Player { },
        ),
    );
}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SPRITE_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } 
        else if translation.x > x_max {
            translation.x = x_max;
        }

        if transform.translation.y < y_min {
            translation.y = y_min;
        } 
        else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation
    }

    return;
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // When we are not sure if our entity does'nt exist, we have to make sure that it's ok
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
    
    return;
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let preloaded_texture: Handle<Image> = asset_server.load("sprites/inimigi.png");

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random_x,
                        random_y,
                        0.0
                    ),
                    texture: preloaded_texture.clone(),
                    ..Default::default() // Set all other field to default
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
                }
            ),
        );
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SPRITE_SIZE / 2.0;

    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        if (translation.x < x_min) || (translation.x > x_max) {
            enemy.direction.x *= -1.0;
        }

        if (translation.y < y_min) || (translation.y > y_max) {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn confine_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SPRITE_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } 
        else if translation.x > x_max {
            translation.x = x_max;
        }

        if transform.translation.y < y_min {
            translation.y = y_min;
        } 
        else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_enemies
        ))
        .add_systems(Update, (
            player_movement,
            confine_player,
            enemy_movement,
            update_enemy_direction,
        ))
        .run()
}
