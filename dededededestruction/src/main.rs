mod systems;
mod entities;

use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_rapier2d::prelude::*;

use entities::player::PuffyTheStarsKillerPlugin;
use entities::systems::animate_sprite;
use crate::systems::{exit_game, spawn_camera};

fn main()  {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<()>::default())
    .add_plugins(RapierDebugRenderPlugin {
      mode: DebugRenderMode::all(),
      ..default()
    })
    .add_systems(Startup, spawn_camera)
    .add_plugins(PuffyTheStarsKillerPlugin)
    .add_systems(Update, (exit_game, animate_sprite))
    .run();
}
