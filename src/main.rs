use bevy::prelude::*;

const PLAYER_SHEET: &str = "clutcorn.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

pub struct WinSize {
  pub w: f32,
  pub h: f32,
}

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
      title: "Clutcorn!".to_string(),
      width: 598.0,
      height: 676.0,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut windows: ResMut<Windows>,
) {
  // camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // capture window size
  let window = windows.get_primary_mut().unwrap();
  let (win_w, win_h) = (window.width(), window.height());

  // position window
  window.set_position(IVec2::new(2780, 4900));

  // add player
  let bottom = -win_h / 2.;
  commands.spawn_bundle(SpriteBundle {
    texture: asset_server.load(PLAYER_SHEET),
    transform: Transform {
      translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
      scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
      ..Default::default()
    },
    ..Default::default()
  });
}
