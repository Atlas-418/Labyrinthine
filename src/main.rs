use bevy::prelude::*;

/* 
 *   --TODO--  
 * A very descriptive list of what the fuck I need to do
 * Display something to a window
 * Display 3D space to a window
 * Make camera at reasonable height in 3d space
 * Move camera with WASD
 * Make a wall
 * Make the camera not phase through the wall
 * Make walls automateable
 * Generate map
 * Put walls according to map

 * I'll come up with more steps later, but as of right now, this is what I need to do.

 * Remember to comment any behaviour I need to remember
 */

const MAZE_WIDTH: u8  = 10;
const MAZE_HEIGHT: u8 = 10;

mod world;
mod camera;
use crate::world::GenWorldPlugin;
use crate::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GenWorldPlugin)
        .add_plugins(CameraPlugin)
        //.add_systems(Startup, )
        .run();
}
