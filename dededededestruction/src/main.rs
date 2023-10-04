mod player;
mod systems;
mod enemy;

use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;

use player::PlayerPlugin;
use crate::systems::{exit_game, spawn_camera};


fn main()  {
  const a: i32 = 2;
  let b: u32 = 10;

  println!("test {} {}", a, b);


  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .add_plugins(PlayerPlugin)
    .add_systems(Update, exit_game)
    .run();
}

fn test_system(test: &str) {

  println!("Test: {}", test.to_string());
}