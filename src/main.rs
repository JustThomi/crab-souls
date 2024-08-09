use bevy::prelude::*;
use bevy_third_person_camera::*;

mod camera;
mod player;
mod world;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
        ))
        .run();
}
