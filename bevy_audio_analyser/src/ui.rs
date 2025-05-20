use std::f32::MAX;

use bevy::{prelude::*, color::palettes::basic::*};

#[derive(Component)]
pub struct FrequencyBar {
    pub band_type: BandType,
}

#[derive(PartialEq)]
pub enum BandType {
    Bass,
    Mids,
    Highs,
}

#[derive(Event)]
pub struct AudioPlaybackEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<AudioPlaybackEvent>()
      .add_systems(Startup, (spawn_ui, spawn_frequency_visualization))
      .add_systems(Update, (button_system, update_frequency_bars));
  }
}

fn spawn_ui(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn(button(&assets));
}

const BAR_WIDTH: Val = Val::Px(60.0);
const MAX_BAR_HEIGHT: f32 = 500.0;
const BASS_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);    // Red
const MIDS_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);    // Green
const HIGHS_COLOR: Color = Color::srgb(0.2, 0.2, 0.8);   // Blue

fn spawn_frequency_visualization(mut commands: Commands) {
  // Container for all bars
  commands.spawn(
    frequency_visualization()
  ).with_children(|parent| {
    // Bass frequency bar
    parent.spawn(frequency_bar(BandType::Bass, BASS_COLOR, "BASS"));

    // Mids frequency bar
    parent.spawn(frequency_bar(BandType::Mids, MIDS_COLOR, "MIDS"));
    
    // Highs frequency bar
    parent.spawn(frequency_bar(BandType::Highs, HIGHS_COLOR, "HIGHS"));
  });
}

fn frequency_visualization() -> impl Bundle + use<>{
  (
    Node {
      width: Val::Percent(100.0),
      height: Val::Px(250.0),
      position_type: PositionType::Absolute,
      bottom: Val::Px(20.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::FlexEnd,
      column_gap: Val::Px(20.0),
      ..default()
    }
  )
}

fn frequency_bar(band_type: BandType, color: Color, label: &str) -> impl Bundle + use<> {
  (
    Node {
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      ..default()
    },
    children![
      (
        Node {
          width: BAR_WIDTH,
          height: Val::Px(0.0),
          ..default()
        },
        FrequencyBar { band_type },
        BackgroundColor(color),
      ),
      (
        Text::new(label),
        TextFont {
          font_size: 16.0,
          ..default()
        }
      )
    ]
)}

fn update_frequency_bars(
  mut query: Query<(&mut Node, &FrequencyBar)>,
  controller: Res<crate::audio_controller::AudioController>,
  analysis: Res<crate::audio_analysis::AudioAnalysisResults>
) {
  if !controller.is_playing || !analysis.analyzed {
    return;
  }

  let frame_index = (controller.current_position / analysis.hop_time as f64) as usize;

  if frame_index < analysis.frequency_analysis.len() {
    let current_analysis = &analysis.frequency_analysis[frame_index];

    let scale = 0.5;

    for(mut node, bar) in query.iter_mut() {
      let height = match bar.band_type {
        BandType::Bass => current_analysis.bass * scale,
        BandType::Mids => current_analysis.mids * scale,
        BandType::Highs => current_analysis.highs * scale
      };

      let clamped_height = (height * MAX_BAR_HEIGHT).clamp(5.0, MAX_BAR_HEIGHT);
      node.height = Val::Px(clamped_height);
    }
  }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
  mut interaction_query: Query<
      (
          &Interaction,
          &mut BackgroundColor,
          &mut BorderColor,
          &Children,
      ),
      (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
  mut play_event_w: EventWriter<AudioPlaybackEvent>
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
          Interaction::Pressed => {
              **text = "PLAYING!".to_string();
              *color = PRESSED_BUTTON.into();
              border_color.0 = RED.into();
              play_event_w.write(AudioPlaybackEvent);
          }
          Interaction::Hovered => {
              **text = "CAREFUL".to_string();
              *color = HOVERED_BUTTON.into();
              border_color.0 = Color::WHITE;
          }
          Interaction::None => {
              **text = "PLAY".to_string();
              *color = NORMAL_BUTTON.into();
              border_color.0 = Color::BLACK;
          }
      }
  }
}

fn button(asset_server: &AssetServer) -> impl Bundle + use<> {
  (
      Node {
          width: Val::Percent(100.0),
          height: Val::Percent(50.0),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
      },
      children![(
          Button,
          Node {
              width: Val::Px(150.0),
              height: Val::Px(65.0),
              border: UiRect::all(Val::Px(5.0)),
              // horizontally center child text
              justify_content: JustifyContent::Center,
              // vertically center child text
              align_items: AlignItems::Center,
              ..default()
          },
          BorderColor(Color::BLACK),
          BorderRadius::MAX,
          BackgroundColor(NORMAL_BUTTON),
          children![(
              Text::new("PLAY"),
              TextFont {
                  font: asset_server.load("fonts/Unageo-Light.ttf"),
                  font_size: 33.0,
                  ..default()
              },
              TextColor(Color::srgb(0.9, 0.9, 0.9)),
              TextShadow::default(),
          )]
      )],
  )
}