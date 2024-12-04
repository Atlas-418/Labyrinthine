use bevy::prelude::*;
//use bevy::input::*;
//use bevy::transform;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
        app.add_systems(Update, (cam_move, cam_look));
    }
}
/*
#[derive(Component)]
struct CameraController {
    speed: f32,
    turn_speed: f32,
}

#[derive(Debug, Component)]
struct Player;
*/
fn start_camera(
    mut cmd: Commands,
){
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn cam_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //cam_query: Query<&Camera3d>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
){
    let mut movement = Vec3::ZERO;
/*
! try 'match keyboard_input.pressed()' instead of 'get_pressed'
* see, I really want to get the match to work, but I think I'll have to settle for an if 
    match keyboard_input.get_pressed() {
        KeyCode::KeyW => {
            movement += Vec3::Z * 0.9;
        },
        KeyCode::KeyS => {
            movement -= Vec3::Z * 0.9;
        },
        _ => panic!(),
    }
*/
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += Vec3::Z * 0.9;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement -= Vec3::Z * 0.9;
    }

    for mut transform in &mut query {
        transform.translation += movement * time.delta_secs();
    }
}

fn cam_look(

){

}
