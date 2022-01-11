#![allow(unused)]
mod player;

use player::PlayerPlugin;

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;
const SCALE: f32 = 0.5;

pub struct Materials {
    player: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}
struct WinSize {
    w: f32,
    h: f32,
}

struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    //Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //Create main resources
    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}
