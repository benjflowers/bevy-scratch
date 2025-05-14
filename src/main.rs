use bevy::prelude::*;
use clap::Parser;
/**
 * 1. provide a bpm
 * 2. provide a path to image
 * 3. provide a runtime
 * 3. based on bpm alter the image
 */

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 120)]
    bpm: u32,

    #[arg(long)]
    image: String,
}

fn main() {
    let args = Args::parse();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BpmSettings {bpm: args.bpm})
        .insert_resource(args)
        .add_systems(Startup, (setup, set_image))
        .run();
}

#[derive(Resource)]
struct BpmSettings {
    bpm: u32
}

#[derive(Resource)]
struct BeatTimer {
    timer: Timer,
}

fn setup(mut commands: Commands, bpm_settings: Res<BpmSettings>) {
    commands.spawn(Camera2d);

    let seconds_per_beat = 60.0/bpm_settings.bpm as f32;

    commands.insert_resource(BeatTimer {
        timer: Timer::from_seconds(seconds_per_beat, TimerMode::Repeating)
    });
}

fn set_image(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
    commands.spawn(
        Sprite{
            image: asset_server.load(&args.image),
            ..default()
        }
    );
}
