mod enemy;
mod player;

/// @author Gabriel Spinola - gabr
///
/// References:
/// LINK - https://bevy-cheatbook.github.io
/// LINK - https://sburris.xyz/posts/bevy-gravity/
/// LINK - https://www.youtube.com/watch?v=4TjEo-gDgAg&t=417
/// 
/// LINK - CU: http://icts.unb.br/jspui/bitstream/10482/41315/1/2020_VicentedePaulaNascimentoLeiteFilho.pdf
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::random;

use enemy::EnemyPlugin;
use player::PlayerPlugin;

#[derive(Resource)]
pub struct StarSpawnTimerResource {
    pub timer: Timer,
}

impl Default for StarSpawnTimerResource {
    fn default() -> Self {
        StarSpawnTimerResource {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<CPlayer>>, // entity's a u32 so we can clone it
    mut game_over_event_write: EventWriter<GameOver>,
    enemy_query: Query<&Transform, With<CEnemy>>,
    score: Res<ScoreResource>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SPRITE_SIZE / 2.0;
            let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;

            // if the distance between two entities is minor than the sum of their radiuses
            // they're colliding
            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
                game_over_event_write.send(GameOver { score: score.value });
            }
        }
    }
}





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin))
        .run()
}
