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