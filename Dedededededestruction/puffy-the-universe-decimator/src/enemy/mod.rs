use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 6;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 32.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<EnemySpawnTimer>()
      .add_systems(Startup, spawn_enemies)
      .add_systems(Update, (
        spawn_enemies,
        enemy_movement,
        update_enemy_direction,
        confine_enemy
      ));
  }
}
