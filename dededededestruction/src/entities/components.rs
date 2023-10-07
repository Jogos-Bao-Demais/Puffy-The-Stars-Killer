use bevy::prelude::{Component, Vec2};

#[derive(Component, Clone, Copy)]
pub struct SpriteAnimation {
	pub len: usize,
	pub frame_time: f32,
}

#[derive(Component)]
pub struct FrameTime(pub f32);

pub struct HitBox(pub Vec2);