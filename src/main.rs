use bevy::prelude::*;
/* 
TODO-  
! A very descriptive list of what the fuck I need to do
 * Make the camera not phase through the wall
 * Generate maze
 * Make a voxel mesh

 * I'll come up with more steps later, but as of right now, this is what I need to do.

 * Remember to comment any behaviour I need to remember
 */

#[allow(dead_code)]
const MAZE_WIDTH:  u64 = 25; // both of these need to be odd, for the maze to properly generate
#[allow(dead_code)]
const MAZE_HEIGHT: u64 = 25;

//mod world;
//use crate::world::GenWorldPlugin;
mod camera;
use crate::camera::CameraPlugin;
mod maze;
mod cubemesh{pub mod gen_cubemesh;}
use crate::cubemesh::gen_cubemesh::GenWorldPlugin;

fn main() {
    println!("Starting");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GenWorldPlugin)
        .add_plugins(CameraPlugin)
        //.add_systems(Startup, ) 
        .run();
}
