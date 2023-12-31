use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
	let window: &Window = window_query.get_single().unwrap();

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
		..Default::default()
	});
}

// Events are used to send data between systems.
pub fn exit_game( 
	mut app_exit_event_write: EventWriter<AppExit>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		app_exit_event_write.send(AppExit);
	}
}

/*
pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
	for event in game_over_event_reader.iter() {
		println!("Final score is: {}", event.score.to_string());
	}
}*/
