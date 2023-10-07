use bevy::prelude::*;
use crate::entities::components::HitBox;

use super::components::{ SpriteAnimation, FrameTime };

pub fn animate_sprite(
	mut query:  Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
	time: Res<Time>,
) {
	for (mut sprite, animation, mut frame_time) in query.iter_mut() {
		frame_time.0 += time.delta_seconds();

		// correct animation if it skipped some frames
		if frame_time.0 > animation.frame_time {
			let frames = (frame_time.0 / animation.frame_time) as usize;
			sprite.index += frames;

			// reset animation
			if sprite.index >= animation.len {
				sprite.index %= animation.len;
			}

			// correction
			frame_time.0 -= animation.frame_time;
		}
	}
}

pub fn check_hit(self_box: HitBox, offset: Vec3, other_box: HitBox, other_offset: Vec3) -> bool {
	let self_h_size = self_box.0.y / 2.;
	let other_h_size = other_box.0.y / 2.;
	let self_w_size = self_box.0.y / 2.;
	let other_w_size = other_box.0.y / 2.;

	// Check for overlap between the two boxes
	offset.x + self_w_size > other_offset.x - other_w_size &&
	offset.x - self_w_size < other_offset.x + other_w_size &&
	offset.y + self_h_size > other_offset.y - other_h_size &&
	offset.y - self_h_size < other_offset.y + other_h_size
}