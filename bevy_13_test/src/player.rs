use bevy::prelude::*;
use crate::movable::*;

const SPEED: f32 = 50.0;
const MAX_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_control_velocity);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            current_speed: CurrentSpeed::new(SPEED),
            model: SpriteBundle {
                texture: asset_server.load("play.png"),
                ..default()
            }
        },
        Player,
    ));
}

/* fn player_control_acceleration(
    mut query: Query<(&mut Acceleration, &mut CurrentSpeed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let Ok ((mut acceleration, mut current_speed)) = query.get_single_mut() else {
        return;
    };
    let mut new_v = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::KeyW) {
        new_v.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        new_v.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        new_v.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        new_v.x += 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if current_speed.value < MAX_SPEED {
            current_speed.value += 10.0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        if current_speed.value >= 0.0 {
            current_speed.value += 10.0;
        }
    }
    new_v = new_v.normalize_or_zero() * current_speed.value;
    acceleration.value = new_v;
    info!("speed: {:?}", current_speed);
} */

fn player_control_velocity(
    mut query: Query<(&mut Velocity, &mut CurrentSpeed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let Ok ((mut velocity, mut current_speed)) = query.get_single_mut() else {
        return;
    };
    let mut new_v = Vec3::new(0.0, 0.0, 0.0);
    if keyboard_input.pressed(KeyCode::KeyW) {
        new_v.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        new_v.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        new_v.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        new_v.x += 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if current_speed.value < MAX_SPEED {
            current_speed.value += 10.0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        if current_speed.value >= 0.0 {
            current_speed.value -= 10.0;
        }
    }
    
    
    if new_v.x == 0.0 && new_v.y == 0.0 { 
        velocity.value = [0.0, 0.0, 0.0].into();
    }
    else {
        new_v = new_v.normalize_or_zero() * current_speed.value;
        velocity.value += new_v;
        velocity.value = velocity.value.clamp_length_max(MAX_SPEED);
    }
    info!("speed: {:?}", current_speed);
    //info!("v: {:?}", velocity.value);
}
/* 
fn create_drag(mut query: Query<&mut Velocity>) {
    let Ok (mut velocity) = query.get_single_mut() else {
        return;
    };
    velocity.value = velocity.value.lerp(Vec3::new(0.0, 0.0, 0.0), 0.2);
}
 */
