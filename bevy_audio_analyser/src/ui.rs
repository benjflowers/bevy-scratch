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