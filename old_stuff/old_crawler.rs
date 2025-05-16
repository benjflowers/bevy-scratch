use bevy::{
  prelude::*,
  render::mesh::Mesh,
};

use rand::Rng;

const MAX_TRAIL_ENTITIES: usize = 1000;
const SPAWN_INTERVAL: f32 = 1.0;
const CRAWLER_COUNT: usize = 2;

#[derive(Component)]
struct Crawler;

#[derive(Component)]
struct Trail;

#[derive(Resource)]
struct TrailResources {
  mesh: Handle<Mesh>,
  material: Handle<ColorMaterial>,
  count: usize,
}

fn main() {
  App::new()
      .add_plugins(DefaultPlugins)
      .insert_resource(TrailTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
      .add_systems(Startup, (setup_camera, setup_resources, spawn_crawlers))
      .add_systems(Update, (crawl, cleanup_old_trails).chain())
      .run();
}

#[derive(Resource)]
struct TrailTimer(Timer);

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn setup_resources(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
){
  let trail_mesh = meshes.add(Circle::new(1.0));
  let trail_material = materials.add(ColorMaterial::from_color(Color::hsl(20.0, 90.0, 0.5)));

  commands.insert_resource(TrailResources {
      mesh: trail_mesh,
      material: trail_material,
      count: 0
  })
}

fn spawn_crawlers(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  let crawler_mesh = meshes.add(Circle::new(2.0));
  let crawler_material = materials.add(ColorMaterial::from_color(Color::hsl(40.0, 60.0, 0.5)));

  for _ in 0..CRAWLER_COUNT {
      commands.spawn((
          Crawler,
          Mesh2d(crawler_mesh.clone()),
          MeshMaterial2d(crawler_material.clone()),
          Transform::from_xyz(0.0, 0.0, 0.0),
      ));
  }
}

fn crawl(
  mut crawlers: Query<&mut Transform, With<Crawler>>,
  mut commands: Commands,
  trail_resources: Res<TrailResources>,
  mut timer: ResMut<TrailTimer>,
  time: Res<Time>,
) {
  // only spawn trails at certain intervals
  timer.0.tick(time.delta());

  let should_spawn_trail = timer.0.just_finished() && trail_resources.count < MAX_TRAIL_ENTITIES;

  let mut rng = rand::rng();

  for mut transform in &mut crawlers {
      // move crawlers
      transform.translation.x += rng.random_range(-2.0..2.0);
      transform.translation.y += rng.random_range(-2.0..2.0);

      // Spawn trails only on timer
      if should_spawn_trail {
          commands.spawn((
              Trail,
              Mesh2d(trail_resources.mesh.clone()),
              MeshMaterial2d(trail_resources.material.clone()),
              Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
          ));
      }
  }

  // Update trail count if we spawned new trails
  if should_spawn_trail {
      commands.insert_resource(TrailResources {
          mesh: trail_resources.mesh.clone(),
          material: trail_resources.material.clone(),
          count: trail_resources.count + crawlers.iter().len(),
      })
  }
}

fn cleanup_old_trails(
  mut commands: Commands,
  trails: Query<Entity, With<Trail>>,
  mut trail_resources: ResMut<TrailResources>
) {
  if trail_resources.count > MAX_TRAIL_ENTITIES {
      println!("Reached Max Trail Entities");
      let mut count = 0;
      let to_remove = (trail_resources.count - MAX_TRAIL_ENTITIES) + 100;

      for entity in trails.iter() {
          commands.entity(entity).despawn();
          count += 1;
          if count >= to_remove {
              break;
          }
      }

      trail_resources.count = trail_resources.count.saturating_sub(count);
  }
}