/// @author Gabriel Spinola - gabr
///
/// References:
/// LINK - https://bevy-cheatbook.github.io
/// LINK - https://sburris.xyz/posts/bevy-gravity/
/// LINK - https://www.youtube.com/watch?v=4TjEo-gDgAg&t=417s
use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 32.0;

pub const NUMBER_OF_ENEMIES: usize = 6;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 32.0;

pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Component)]
pub struct CPlayer;

#[derive(Component)]
pub struct CEnemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct ScoreResource {
    pub value: u32,
}

impl Default for ScoreResource {
    fn default() -> Self {
        ScoreResource { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimerResource {
    pub timer: Timer,
}

impl Default for StarSpawnTimerResource {
    fn default() -> Self {
        StarSpawnTimerResource {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..Default::default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/PuffyTheStarsKillerPlaceholder.png"),
            ..Default::default() // Set all other field to default
        },
        CPlayer {},
    ));
}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<CPlayer>>,
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
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if transform.translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation
    }

    return;
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<CPlayer>>,
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

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: preloaded_texture.clone(),
                ..Default::default() // Set all other field to default
            },
            CEnemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &CEnemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut CEnemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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
    mut enemy_query: Query<&mut Transform, With<CEnemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_player_size: f32 = PLAYER_SPRITE_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_player_size;
    let x_max: f32 = window.width() - half_player_size;
    let y_min: f32 = 0.0 + half_player_size;
    let y_max: f32 = window.height() - half_player_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if transform.translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<CPlayer>>, // entity's a u32 so we can clone it
    enemy_query: Query<&Transform, With<CEnemy>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SPRITE_SIZE / 2.0;
            let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;

            // if the distance between two entities is minor than the sum of their radiuses
            // they're colliding
            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<ScoreResource>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_start_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimerResource>,
    time: Res<Time>,
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut score: ResMut<ScoreResource>,
    star_spawn_timer: Res<StarSpawnTimerResource>,
) {
    if star_spawn_timer.timer.finished() {
        println!("star spawned");
        score.value += 1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ScoreResource>()
        .init_resource::<StarSpawnTimerResource>()
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                tick_start_spawn_timer,
                player_movement,
                confine_player,
                enemy_movement,
                update_enemy_direction,
                enemy_hit_player,
                update_score,
                spawn_stars_over_time
            ),
        )
        .run()
}
