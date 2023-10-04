/// @author Gabriel Spinola - gabr
///
/// References:
/// LINK - https://bevy-cheatbook.github.io
/// LINK - https://sburris.xyz/posts/bevy-gravity/
/// LINK - https://www.youtube.com/watch?v=4TjEo-gDgAg&t=417
///
/// LINK - CU: http://icts.unb.br/jspui/bitstream/10482/41315/1/2020_VicentedePaulaNascimentoLeiteFilho.pdf

use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 32.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
      app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
          player_movement,
          confine_player
        ));
    }
}