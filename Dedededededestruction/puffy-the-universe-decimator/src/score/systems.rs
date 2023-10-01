use bevy::prelude::*;

pub fn update_score(score: Res<ScoreResource>) {
  if score.is_changed() {
      println!("Score: {}", score.value.to_string());
  }
}