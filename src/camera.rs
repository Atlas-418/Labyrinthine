use bevy::prelude::*;
use bevy::input::*;
use bevy::render::view::RenderLayers;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
        app.add_systems(Update, (cam_move, cam_look));
    }
}

#[derive(Component)]
struct CameraController {
    speed: f32,
    turn_speed: f32,
}

#[derive(Debug, Component)]
struct Player;

fn start_camera(
    mut cmd: Commands,
){
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn cam_move(

){

}

fn cam_look(

){

}
