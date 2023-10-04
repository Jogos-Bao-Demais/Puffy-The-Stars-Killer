#[derive(Resource)]
pub struct ScoreResource {
  pub value: u32,
}

impl Default for ScoreResource {
  fn default() -> Self {
    ScoreResource { value: 0 }
  }
}
