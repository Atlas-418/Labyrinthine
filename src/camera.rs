use bevy::prelude::*;
use bevy::input::*;
use bevy::render::view::RenderLayers;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
    }
}

#[derive(Component)]
struct CameraController {
    speed: f32,
    turn_speed: f32,
}


#[derive(Debug, Component)]
struct Player;


#[derive(Debug, Component)]
struct WorldModelCamera;

fn start_camera(

){

}
