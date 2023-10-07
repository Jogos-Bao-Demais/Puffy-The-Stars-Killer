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
mod resources;

use systems::*;
use crate::entities::player::resources::PuffyTheStarsKillerAnimations;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 32.0;

pub struct PuffyTheStarsKillerPlugin;

impl Plugin for PuffyTheStarsKillerPlugin {
    fn build(&self, app: &mut App) {
      app
        .init_resource::<PuffyTheStarsKillerAnimations>()
        .add_systems(Startup, spawn_puffy_stars_killer)
        .add_systems(Update, (
          puffy_the_stars_killer_movement,
          change_puffy_the_stars_killer_animation,
          confine_puffy_the_stars_killer,
        ));
    }
}