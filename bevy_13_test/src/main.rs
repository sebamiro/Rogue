mod camera;
mod movable;
mod player;

use bevy::prelude::*;
//use bevy::windows::close_on_esc;
use camera::CameraPlugin;
use player::PlayerPlugin;
use movable::MovingPlugin;
use bevy::window::WindowMode::Fullscreen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RogueGame".into(),
                //resolution: (600., 100.).into(),
                resizable: true,
                mode: Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovingPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
