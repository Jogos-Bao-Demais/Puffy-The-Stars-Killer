/// @author Gabriel Spinola - gabr
///
/// References:
/// LINK - https://bevy-cheatbook.github.io
/// LINK - https://sburris.xyz/posts/bevy-gravity/
/// LINK - https://www.youtube.com/watch?v=4TjEo-gDgAg&t=417
/// LINK - https://github.com/jakobhellermann/bevy-inspector-egui
/// LINK - Tuto: https://youtu.be/dlvXu18L828?si=gEMC8BiEIcgGJ5_F 
/// LINK - UI: https://github.com/jkb0o/belly
/// 
/// LINK - CU: http://icts.unb.br/jspui/bitstream/10482/41315/1/2020_VicentedePaulaNascimentoLeiteFilho.pdf

mod enemy;
mod player;

use std::default;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::random;

use enemy::EnemyPlugin;
use player::PlayerPlugin;

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Puffy The Stars Killer".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin))
        .run()
}
