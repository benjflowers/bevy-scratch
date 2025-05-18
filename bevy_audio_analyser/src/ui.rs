use bevy::{prelude::*, color::palettes::basic::*};

#[derive(Event)]
pub struct AudioPlaybackEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<AudioPlaybackEvent>()
      .add_systems(Startup, spawn_ui)
      .add_systems(Update, button_system);
  }
}

fn spawn_ui(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn(button(&assets));
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
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
          Interaction::Pressed => {
              **text = "PLAYING!".to_string();
              *color = PRESSED_BUTTON.into();
              border_color.0 = RED.into();
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
          height: Val::Percent(100.0),
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