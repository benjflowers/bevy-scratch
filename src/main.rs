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
        .add_systems(Update, beat_pixel_manipulation)
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

#[derive(Resource)]
struct ImageAsset(Handle<Image>);

fn setup(mut commands: Commands, bpm_settings: Res<BpmSettings>) {
    commands.spawn(Camera2d);

    let seconds_per_beat = 60.0/bpm_settings.bpm as f32;

    commands.insert_resource(BeatTimer {
        timer: Timer::from_seconds(seconds_per_beat, TimerMode::Repeating)
    });
}

fn set_image(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
    let image_handle: Handle<Image> = asset_server.load(&args.image);
    commands.insert_resource(ImageAsset(image_handle.clone()));
    commands.spawn(
        Sprite{
            image: image_handle,
            ..default()
        }
    );
}

fn beat_pixel_manipulation(
    time: Res<Time>,
    mut beat_timer: ResMut<BeatTimer>,
    image_handle: Res<ImageAsset>,
    mut images: ResMut<Assets<Image>>,
    mut beat_count: Local<u32>,
) {
    // Update timer
    beat_timer.timer.tick(time.delta());
    
    // Check if beat occurred
    if beat_timer.timer.just_finished() {
        *beat_count += 1;
        
        // Get the image to manipulate
        if let Some(image) = images.get_mut(&image_handle.0) {
            // Choose effect based on beat count
            match *beat_count % 4 {
                0 => invert_colors(image),
                1 => shift_colors_red(image),
                2 => shift_colors_blue(image),
                3 => add_scanlines(image, *beat_count),
                _ => {}
            }
            
            println!("Beat {}: Applied image effect", *beat_count);
        }
    }
}

// Invert all colors in the image
fn invert_colors(image: &mut Image) {
    for pixel in image.data.chunks_mut(4) {
        // Invert RGB but leave alpha unchanged
        pixel[0] = 255 - pixel[0]; // R
        pixel[1] = 255 - pixel[1]; // G
        pixel[2] = 255 - pixel[2]; // B
    }
}

// Shift color channels toward red
fn shift_colors_red(image: &mut Image) {
    for pixel in image.data.chunks_mut(4) {
        pixel[0] = pixel[0].saturating_add(50); // Boost red
        pixel[1] = pixel[1].saturating_sub(20); // Reduce green
        pixel[2] = pixel[2].saturating_sub(20); // Reduce blue
    }
}

// Shift color channels toward blue
fn shift_colors_blue(image: &mut Image) {
    for pixel in image.data.chunks_mut(4) {
        pixel[0] = pixel[0].saturating_sub(20); // Reduce red
        pixel[1] = pixel[1].saturating_sub(20); // Reduce green
        pixel[2] = pixel[2].saturating_add(50); // Boost blue
    }
}

fn add_scanlines(image: &mut Image, beat_count: u32) {
    let width = image.texture_descriptor.size.width as usize;
    let scanline_offset = (beat_count as usize * 4) % 30;
    
    for y in scanline_offset..image.texture_descriptor.size.height as usize {
        if y % 10 == 0 {  // Every 10th line
            for x in 0..width {
                let index = (y * width + x) * 4;
                if index + 3 < image.data.len() {
                    // Darken the scanline
                    image.data[index] = image.data[index].saturating_mul(70).wrapping_div(100);
                    image.data[index+1] = image.data[index+1].saturating_mul(70).wrapping_div(100);
                    image.data[index+2] = image.data[index+2].saturating_mul(70).wrapping_div(100);
                }
            }
        }
    }
}
