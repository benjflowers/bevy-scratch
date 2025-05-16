use bevy::{
  prelude::*,
  render::mesh::Mesh,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
  App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, (setup))
      .run();
}

#[derive(Component)]

struct Crawler{
  pos: Vec2,
  size: f32
}

#[derive(Event)]
struct Crawl;

#[derive(Event)]
struct MoveCrawler{
  pos: Vec2
}

impl Crawler {
  fn random(rand: &mut ChaCha8Rng) -> Self {
      Crawler {
          pos: Vec2::new(
              (rand.random::<f32>()),
              (rand.random::<f32>()),
          ),
          size: 4.0
      }
  }
}

fn setup(
  mut commands: Commands
) {
  // spawn camera
  commands.spawn(Camera2d);
  let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
  let mut observer = Observer::new(crawler_move);

  for _ in 0..100 {
      let entity = commands.spawn(Crawler::random(&mut rng)).id();
      observer.watch_entity(entity);
  }

  commands.spawn(observer);
}

fn crawler_move(trigger: Trigger<Crawl>, query: Query<&Crawler>, mut commands: Commands) {
  let id = trigger.entity();
  let Some(mut entity) = commands.get_entity(id) else {
      return;
  };
  let crawler = query.get(id).unwrap();

  commands.trigger(MoveCrawler{
      pos: crawler.pos
  });
  println!("{}", id);
}