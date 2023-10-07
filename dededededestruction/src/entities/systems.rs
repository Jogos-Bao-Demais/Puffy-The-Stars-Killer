use bevy::prelude::*;

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