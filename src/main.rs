use bevy::prelude::*;

mod audio_analysis;
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, audio_thing)
    .run();
}

fn audio_thing() {
  audio_analysis::main();
}