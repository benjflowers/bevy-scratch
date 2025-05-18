use bevy::prelude;

#[derive(Event)]
pub struct AudioPlaybackEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<AudioPlaybackEvent>()
      .add_systems(Startup, spawn_ui);
  }
}

fn spawn_ui(mut commands: Commands) {
  println!("UI STUFF");
}