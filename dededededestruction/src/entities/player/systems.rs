use bevy::{prelude::*, window::PrimaryWindow};

use crate::entities::components::{FrameTime, SpriteAnimation};
use crate::entities::player::PuffyTheStarsKillerPlugin;
use crate::entities::player::resources::{PuffyTheStarsKillerAnimations, PuffyTheStarsKillerAnimationsKeys};

use super::{components::PuffyTheStarsKiller, PLAYER_SPEED, PLAYER_SPRITE_SIZE};

pub fn spawn_puffy_stars_killer(
  mut commands: Commands,
  mut texture_atlas: ResMut<Assets<TextureAtlas>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window: &Window = window_query.get_single().unwrap();

  let atlas = TextureAtlas::from_grid(
    asset_server.load("temp/free/Main Characters/Virtual Guy/Idle (32x32).png"),
    Vec2::splat(32.), // states that our texture is 32 by 32 size
    11,
    1,
    None,
    None
  );

  commands.spawn((
    PuffyTheStarsKiller,
    SpriteSheetBundle {
      transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      texture_atlas: texture_atlas.add(atlas),
      sprite: TextureAtlasSprite { index: 0, ..Default::default() },
      ..Default::default() // Set all other field to default
    },
    SpriteAnimation {
      len: 11,
      frame_time: 1./20.
    },
    FrameTime(0.0)
  ));
}

pub fn puffy_the_stars_killer_movement(
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut player_transform: Query<&mut Transform, With<PuffyTheStarsKiller>>,
) {
  // When we are not sure if our entity does'nt exist, we have to make sure that it's ok
  if let Ok(mut transform) = player_transform.get_single_mut() {
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
      transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D])  {
      transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S])  {
      transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W])  {
      transform.translation.y += PLAYER_SPEED * time.delta_seconds();
    }
  }

  return;
}

fn flip_puffy_the_stars_killer(mut flip: bool, move_keys: &[KeyCode; 4]) {
  todo!();
}

pub fn change_puffy_the_stars_killer_animation(
  mut player: Query<
    (&mut Handle<TextureAtlasSprite>, &mut SpriteAnimation, &mut TextureAtlasSprite),
    With<PuffyTheStarsKiller>
  >,
  mut texture_atlas: ResMut<Assets<TextureAtlas>>,
  asset_server: Res<AssetServer>,
  mut puffy_animation: ResMut<PuffyTheStarsKillerAnimations>,
  input: Res<Input<KeyCode>>,
) {
  let movement_keys = [KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right];

  if let Ok((mut atlas, mut animation, mut sprite)) = player.get_single_mut() {
    if input.any_just_pressed(movement_keys) {

      puffy_animation
    }

    if input.any_just_pressed(movement_keys[..=2]) {
      sprite.flip_x = true;
    }
    else if input.any_just_pressed([3..=4]) {
      sprite.flip_x = false;
    }

    if !input.any_just_released(movement_keys) && !input.any_pressed(movement_keys) {

    }
  }

  return;
}

pub fn confine_puffy_the_stars_killer(
  mut player_query: Query<&mut Transform, With<PuffyTheStarsKiller>>,
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