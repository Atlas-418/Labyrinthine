use bevy::prelude::*;

/* 
TODO-  
! A very descriptive list of what the fuck I need to do
 * Make camera at reasonable height in 3d space
 * Make the camera not phase through the wall
 * Generate maze
 * Make a voxel mesh
 * make the camera rotateable, and move in the direction of rotation

 * I'll come up with more steps later, but as of right now, this is what I need to do.

 * Remember to comment any behaviour I need to remember
 */

const MAZE_WIDTH: usize  = 15; // both of these need to be odd, for the maze to properly generate
const MAZE_HEIGHT: usize = 15; // both of these need to be odd, for the maze to properly generate

//mod world;
//use crate::world::GenWorldPlugin;
mod camera;
use crate::camera::CameraPlugin;
mod maze;
mod cubemesh{pub mod gen_cubemesh;}
use crate::cubemesh::gen_cubemesh::GenWorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GenWorldPlugin)
        .add_plugins(CameraPlugin)
        //.add_systems(Startup, ) 
        .run();
}
