
/* 
TODO-  
! A very descriptive list of what the fuck I need to do
 * Make the camera not phase through the wall
 * Generate maze
 * Make a voxel mesh

 * I'll come up with more steps later, but as of right now, this is what I need to do.

 * Remember to comment any behaviour I need to remember
 */


mod camera;
mod maze;
mod cubemesh{pub mod gen_cubemesh;}

use bevy::prelude::*;
use crate::maze::*;
use crate::camera::CameraPlugin;
use crate::cubemesh::gen_cubemesh::GenWorldPlugin;


const MAZE_WIDTH:  u64 = 25; // both of these need to be odd, for the maze to properly generate
const MAZE_HEIGHT: u64 = 25;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref MAZE: Maze = Maze::make_maze(MazeAlgorithm::Random);
}

fn main() {
    println!("Starting");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GenWorldPlugin)
        .add_plugins(CameraPlugin)
        //.add_systems(Startup, ) 
        .run();
}
