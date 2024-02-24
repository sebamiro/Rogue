mod camera;
mod movable;
mod player;

use bevy::prelude::*;
//use bevy::windows::close_on_esc;
use camera::CameraPlugin;
use player::PlayerPlugin;
use movable::MovingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovingPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
