use std::collections::HashMap;
use bevy::prelude::*;
use crate::entities::components::SpriteAnimation;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PuffyTheStarsKillerAnimationsKeys {
	IDLE,
	RUN,
}

#[derive(Resource)]
pub struct PuffyTheStarsKillerAnimations {
	animationsMap: HashMap<PuffyTheStarsKillerAnimationsKeys, (Handle<TextureAtlas>, SpriteAnimation)>
}

impl PuffyTheStarsKillerAnimations {
	pub fn add(&mut self, key: PuffyTheStarsKillerAnimationsKeys, handle: Handle<TextureAtlas>, animation: SpriteAnimation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
		return self.animationsMap.insert(key, (handle, animation))
	}

	pub fn get(&mut self, key: PuffyTheStarsKillerAnimationsKeys) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
		return self.animationsMap.get(&key).cloned()
	}
}

impl FromWorld for PuffyTheStarsKillerAnimations {
	fn from_world(world: &mut World) -> Self {
		let mut puffy_the_stars_killer_animations = PuffyTheStarsKillerAnimations { animationsMap: HashMap::new() };
		let mut texture_atlases = world.resource::<Assets<TextureAtlas>>();
		let asset_server = world.resource::<AssetServer>();

		let idle_atlas = TextureAtlas::from_grid(
			asset_server.load("temp/free/Main Characters/Virtual Guy/Idle (32x32).png"),
			Vec2::splat(32.), // states that our texture is 32 by 32 size
			11,
			1,
			None,
			None
		);

		let run_atlas = TextureAtlas::from_grid(
			asset_server.load("temp/Free/Main Characters/Virtual Guy/Jump (32x32).png"),
			Vec2::splat(32.), // states that our texture is 32 by 32 size
			12,
			1,
			None,
			None
		);

		puffy_the_stars_killer_animations.add(PuffyTheStarsKillerAnimationsKeys::IDLE, texture_atlases.add(idle_atlas), SpriteAnimation { len: 11, frame_time: 1./20. });
		puffy_the_stars_killer_animations.add(PuffyTheStarsKillerAnimationsKeys::RUN, texture_atlases.add(run_atlas), SpriteAnimation { len: 12, frame_time: 1./20. });

		return puffy_the_stars_killer_animations
	}
}