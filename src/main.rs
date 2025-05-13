use bevy::prelude::*;
use clap::Parser;
/**
 * 1. provide a bpm
 * 2. provide an image
 * 3. based on bpm alter the image
 */

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 120)]
    bpm: u32,

    #[arg(long)]
    image: Option<String>,
}

fn main() {
    let args = Args::parse();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BpmSettings {bpm: args.bpm})
        .run();
}

#[derive(Resource)]
struct BpmSettings {
    bpm: u32
}
