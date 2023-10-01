use bevy::{prelude::*, window::PrimaryWindow};

use super::{components::CPlayer, PLAYER_SPRITE_SIZE, PLAYER_SPEED};

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